// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! PCAP handlers and singleton for HTTP and gRPC server.
//!
//! This module implements a handler for GET, PATCH, LIST pcap
//!
//! /v1/pcap --> handle_pcaps
//! /v1/pcap/{id} --> handle_pcap
//! handle_pcap_cxx calls handle_pcaps or handle_pcap based on the method

use cxx::CxxVector;
use frontend_proto::common::ChipKind;
use frontend_proto::frontend::GetPcapResponse;
use lazy_static::lazy_static;
use protobuf::Message;
use std::collections::hash_map::{Iter, Values};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ffi::CxxServerResponseWriter;
use crate::http_server::http_request::{HttpHeaders, HttpRequest};
use crate::http_server::server_response::ResponseWritable;
use crate::CxxServerResponseWriterWrapper;

use super::capture::Pcap;
use super::managers::{handle_pcap_list, handle_pcap_patch};
use super::pcap_util::{append_record, PacketDirection};

// The Pcap resource is a singleton that manages all pcaps
lazy_static! {
    static ref RESOURCE: RwLock<PcapMaps> = RwLock::new(PcapMaps::new());
}

// Pcaps contains a recent copy of all chips and their ChipKind, chip_id,
// and owning device name. Information for any recent or ongoing captures is
// also stored in the ProtoPcap.
pub type ChipId = i32;
pub type FacadeId = i32;

// facade_id_to_pcap allows for fast lookups when handle_request, handle_response
// is invoked from packet_hub.
pub struct PcapMaps {
    facade_key_to_pcap: HashMap<(ChipKind, FacadeId), Arc<Mutex<Pcap>>>,
    chip_id_to_pcap: HashMap<ChipId, Arc<Mutex<Pcap>>>,
}

impl PcapMaps {
    fn new() -> Self {
        PcapMaps {
            facade_key_to_pcap: HashMap::<(ChipKind, FacadeId), Arc<Mutex<Pcap>>>::new(),
            chip_id_to_pcap: HashMap::<ChipId, Arc<Mutex<Pcap>>>::new(),
        }
    }

    pub fn contains(&self, key: ChipId) -> bool {
        self.chip_id_to_pcap.contains_key(&key)
    }

    pub fn get(&mut self, key: ChipId) -> Option<&mut Arc<Mutex<Pcap>>> {
        self.chip_id_to_pcap.get_mut(&key)
    }

    pub fn insert(&mut self, pcap: Pcap) {
        let chip_id = pcap.id;
        let facade_key = pcap.get_facade_key();
        let arc_pcap = Arc::new(Mutex::new(pcap));
        self.chip_id_to_pcap.insert(chip_id, arc_pcap.clone());
        self.facade_key_to_pcap.insert(facade_key, arc_pcap);
    }

    pub fn is_empty(&self) -> bool {
        self.chip_id_to_pcap.is_empty()
    }

    pub fn iter(&self) -> Iter<ChipId, Arc<Mutex<Pcap>>> {
        self.chip_id_to_pcap.iter()
    }

    // When pcap is removed, remove from each map and also invoke closing of files.
    pub fn remove(&mut self, key: &ChipId) {
        if let Some(arc_pcap) = self.chip_id_to_pcap.get(key) {
            if let Ok(mut pcap) = arc_pcap.lock() {
                self.facade_key_to_pcap.remove(&pcap.get_facade_key());
                pcap.stop_capture();
            }
        } else {
            println!("key does not exist in Pcaps");
            return;
        }
        self.chip_id_to_pcap.remove(key);
    }

    pub fn values(&self) -> Values<ChipId, Arc<Mutex<Pcap>>> {
        self.chip_id_to_pcap.values()
    }
}

/// The Rust pcap handler used directly by Http frontend for LIST, GET, and PATCH
pub fn handle_pcap(request: &HttpRequest, param: &str, writer: ResponseWritable) {
    if request.uri.as_str() == "/v1/pcaps" {
        match request.method.as_str() {
            "GET" => {
                let mut pcaps = RESOURCE.write().unwrap();
                handle_pcap_list(writer, &mut pcaps);
            }
            _ => writer.put_error(404, "Not found."),
        }
    } else {
        match request.method.as_str() {
            "GET" => {
                // TODO: Implement handle_pcap_get in controller.rs
                writer.put_ok_with_length("text/plain", 0);
                let response_bytes = GetPcapResponse::new().write_to_bytes().unwrap();
                writer.put_chunk(&response_bytes);
                writer.put_chunk(&response_bytes);
            }
            "PATCH" => {
                let mut pcaps = RESOURCE.write().unwrap();
                let id = match param.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        writer.put_error(404, "Incorrect ID type for pcap, ID should be i32.");
                        return;
                    }
                };
                let body = &request.body;
                let state = String::from_utf8(body.to_vec()).unwrap();
                match state.as_str() {
                    "1" => handle_pcap_patch(writer, &mut pcaps, id, true),
                    "2" => handle_pcap_patch(writer, &mut pcaps, id, false),
                    _ => writer.put_error(404, "Incorrect state for PatchPcap"),
                }
            }
            _ => writer.put_error(404, "Not found."),
        }
    }
}

/// pcap handle cxx for grpc server to call
pub fn handle_pcap_cxx(
    responder: Pin<&mut CxxServerResponseWriter>,
    method: String,
    param: String,
    body: String,
) {
    let mut request = HttpRequest {
        method,
        uri: String::new(),
        headers: HttpHeaders::new(),
        version: "1.1".to_string(),
        body: body.as_bytes().to_vec(),
    };
    if param.is_empty() {
        request.uri = "/v1/pcaps".to_string();
    } else {
        request.uri = format!("/v1/pcaps/{}", param);
    }
    handle_pcap(
        &request,
        param.as_str(),
        &mut CxxServerResponseWriterWrapper { writer: responder },
    );
}

// Helper function for translating u32 representation of ChipKind
fn int_to_chip_kind(kind: u32) -> ChipKind {
    match kind {
        1 => ChipKind::BLUETOOTH,
        2 => ChipKind::WIFI,
        3 => ChipKind::UWB,
        _ => ChipKind::UNSPECIFIED,
    }
}

// A common code for handle_request and handle_response cxx mehtods
fn handle_packet(
    kind: u32,
    facade_id: u32,
    packet: &CxxVector<u8>,
    packet_type: u32,
    direction: PacketDirection,
) {
    let pcaps = RESOURCE.read().unwrap();
    let facade_key = Pcap::new_facade_key(int_to_chip_kind(kind), facade_id as i32);
    if let Some(mut pcap) =
        pcaps.facade_key_to_pcap.get(&facade_key).map(|arc_pcap| arc_pcap.lock().unwrap())
    {
        if let Some(ref mut file) = pcap.file {
            if int_to_chip_kind(kind) == ChipKind::BLUETOOTH {
                let timestamp =
                    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                match append_record(timestamp, file, direction, packet_type, packet.as_slice()) {
                    Ok(size) => {
                        pcap.size += size;
                        pcap.records += 1;
                    }
                    Err(err) => {
                        println!("netsimd: {err:?}");
                    }
                }
            }
        }
    };
}

// Cxx Method for packet_hub to invoke (Host to Controller Packet Flow)
pub fn handle_pcap_request(kind: u32, facade_id: u32, packet: &CxxVector<u8>, packet_type: u32) {
    handle_packet(kind, facade_id, packet, packet_type, PacketDirection::HostToController)
}

// Cxx Method for packet_hub to invoke (Controller to Host Packet Flow)
pub fn handle_pcap_response(kind: u32, facade_id: u32, packet: &CxxVector<u8>, packet_type: u32) {
    handle_packet(kind, facade_id, packet, packet_type, PacketDirection::ControllerToHost)
}
