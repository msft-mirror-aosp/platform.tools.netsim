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

//! Packet Capture handlers and singleton for HTTP and gRPC server.
//!
//! This module implements a handler for GET, PATCH, LIST capture
//!
//! /v1/captures --> handle_capture_list
//!
//! /v1/captures/{id} --> handle_capture_patch, handle_capture_get
//!
//! handle_capture_cxx calls handle_capture, which calls handle_capture_* based on uri.
//! handle_packet_request and handle_packet_response is invoked by packet_hub
//! to write packets to files if capture state is on.

// TODO(b/274506882): Implement gRPC status proto on error responses. Also write better
// and more descriptive error messages with proper error codes.

use cxx::CxxVector;
use frontend_proto::common::ChipKind;
use frontend_proto::frontend::{ListCaptureResponse, ListDeviceResponse};
use log::error;
use netsim_common::util::time_display::TimeDisplay;
use protobuf_json_mapping::{print_to_string_with_options, PrintOptions};
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Result};
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::captures::capture::ChipId;
use crate::ffi::CxxServerResponseWriter;
use crate::http_server::http_request::{HttpHeaders, HttpRequest};
use crate::http_server::server_response::ResponseWritable;
use crate::resource::get_captures_resource;
use crate::system;
use crate::CxxServerResponseWriterWrapper;

use super::capture::CaptureInfo;
use super::pcap_util::{append_record, PacketDirection};
use super::PCAP_MIME_TYPE;

const CHUNK_LEN: usize = 1024;
const JSON_PRINT_OPTION: PrintOptions = PrintOptions {
    enum_values_int: false,
    proto_field_name: false,
    always_output_default_values: true,
    _future_options: (),
};

/// Updates the Captures collection to reflect the currently connected devices.
///
/// This function removes entries from Captures when devices/chips
/// go away and adds entries when new devices/chips connect.
/// Note: if a device disconnects and there is captured data, the entry
/// remains with a flag valid = false so it can be retrieved.
pub fn update_captures() {
    let device_response = match crate::devices::devices_handler::get_devices() {
        Ok(scene) => ListDeviceResponse { devices: scene.devices, ..Default::default() },
        Err(err) => {
            error!("{err}");
            return;
        }
    };

    // Adding to Captures hashmap
    let captures_arc = get_captures_resource();
    let mut captures = captures_arc.write().unwrap();
    let mut chip_ids = HashSet::<ChipId>::new();
    for device in device_response.devices {
        for chip in device.chips {
            chip_ids.insert(chip.id);
            if !captures.contains(chip.id) {
                let mut capture = CaptureInfo::new(
                    chip.kind.enum_value_or_default(),
                    chip.id,
                    device.name.clone(),
                );
                // TODO(b/268271460): Add ability to set default capture state.
                // Currently, the default capture state is ON
                let _ = capture.start_capture();
                captures.insert(capture);
            }
        }
    }

    // Two cases when device gets disconnected:
    // 1. The device had no capture, remove completely.
    // 2. The device had capture, indicate by capture.set_valid(false)
    enum RemovalIndicator {
        Gone(ChipId),   // type ChipId = i32
        Unused(ChipId), // type ChipId = i32
    }

    // Check if the active_capture entry still exists in the chips.
    let mut removal = Vec::<RemovalIndicator>::new();
    for (chip_id, capture) in captures.iter() {
        let lock = capture.lock().unwrap();
        let proto_capture = lock.get_capture_proto();
        if !chip_ids.contains(chip_id) {
            if proto_capture.size == 0 {
                removal.push(RemovalIndicator::Unused(chip_id.to_owned()));
            } else {
                removal.push(RemovalIndicator::Gone(chip_id.to_owned()))
            }
        }
    }

    // Now remove/update the captures based on the loop above
    for indicator in removal {
        match indicator {
            RemovalIndicator::Unused(key) => captures.remove(&key),
            RemovalIndicator::Gone(key) => {
                for capture in captures.get(key).iter() {
                    let mut lock = capture.lock().unwrap();
                    lock.stop_capture();
                    // Valid is marked false if the capture of the device is disconnected from netsim
                    lock.valid = false;
                }
            }
        }
    }
}

/// Helper function for getting file name from the given fields.
fn get_file(id: ChipId, device_name: String, chip_kind: ChipKind) -> Result<File> {
    let mut filename = system::netsimd_temp_dir();
    filename.push("pcaps");
    filename.push(format!("{:?}-{:}-{:?}.pcap", id, device_name, chip_kind));
    File::open(filename)
}

// TODO: GetCapture should return the information of the capture. Need to reconsider
// uri hierarchy.
// GET /captures/id/{id} --> Get Capture information
// GET /captures/contents/{id} --> Download Pcap file
/// Performs GetCapture to download pcap file and write to writer.
pub fn handle_capture_get(writer: ResponseWritable, id: ChipId) {
    // Get the most updated active captures
    update_captures();
    let captures_lock = get_captures_resource();
    let mut captures = captures_lock.write().unwrap();
    if let Some(capture) = captures.get(id).map(|arc_capture| arc_capture.lock().unwrap()) {
        if capture.size == 0 {
            writer.put_error(
                404,
                &format!(
                    "Capture file not found for {:?}-{}-{:?}",
                    id, capture.device_name, capture.chip_kind
                ),
            );
        } else if let Ok(mut file) = get_file(id, capture.device_name.clone(), capture.chip_kind) {
            let mut buffer = [0u8; CHUNK_LEN];
            let time_display = TimeDisplay::new(capture.seconds, capture.nanos as u32);
            let header_value = format!(
                "attachment; filename=\"{:?}-{:}-{:?}-{}.pcap\"",
                id,
                capture.device_name.clone(),
                capture.chip_kind,
                time_display.utc_display()
            );
            writer.put_ok_with_length(
                PCAP_MIME_TYPE,
                capture.size,
                &[("Content-Disposition", header_value.as_str())],
            );
            loop {
                match file.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(length) => writer.put_chunk(&buffer[..length]),
                    Err(_) => {
                        writer.put_error(404, "Error reading pcap file");
                        break;
                    }
                }
            }
        } else {
            writer.put_error(404, "Cannot open Capture file");
        }
    } else {
        writer.put_error(404, "Cannot access Capture Resource")
    };
}

/// Performs ListCapture to get the list of CaptureInfos and write to writer.
pub fn handle_capture_list(writer: ResponseWritable) {
    // Get the most updated active captures
    update_captures();
    let captures_arc = get_captures_resource();
    let captures = captures_arc.write().unwrap();
    // Instantiate ListCaptureResponse and add Captures
    let mut response = ListCaptureResponse::new();
    for capture in captures.values() {
        response.captures.push(capture.lock().unwrap().get_capture_proto());
    }

    // Perform protobuf-json-mapping with the given protobuf
    if let Ok(json_response) = print_to_string_with_options(&response, &JSON_PRINT_OPTION) {
        writer.put_ok("text/json", &json_response, &[])
    } else {
        writer.put_error(404, "proto to JSON mapping failure")
    }
}

/// Performs PatchCapture to patch a CaptureInfo with id.
/// Writes the result of PatchCapture to writer.
pub fn handle_capture_patch(writer: ResponseWritable, id: ChipId, state: bool) {
    // Get the most updated active captures
    update_captures();
    let captures_arc = get_captures_resource();
    let mut captures = captures_arc.write().unwrap();
    if let Some(mut capture) = captures.get(id).map(|arc_capture| arc_capture.lock().unwrap()) {
        match state {
            true => {
                if let Err(err) = capture.start_capture() {
                    writer.put_error(404, err.to_string().as_str());
                    return;
                }
            }
            false => capture.stop_capture(),
        }

        // Perform protobuf-json-mapping with the given protobuf
        if let Ok(json_response) =
            print_to_string_with_options(&capture.get_capture_proto(), &JSON_PRINT_OPTION)
        {
            writer.put_ok("text/json", &json_response, &[]);
        } else {
            writer.put_error(404, "proto to JSON mapping failure");
        }
    };
}

/// The Rust capture handler used directly by Http frontend or handle_capture_cxx for LIST, GET, and PATCH
pub fn handle_capture(request: &HttpRequest, param: &str, writer: ResponseWritable) {
    if request.uri.as_str() == "/v1/captures" {
        match request.method.as_str() {
            "GET" => {
                handle_capture_list(writer);
            }
            _ => writer.put_error(404, "Not found."),
        }
    } else {
        match request.method.as_str() {
            "GET" => {
                let id = match param.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        writer.put_error(404, "Incorrect ID type for capture, ID should be i32.");
                        return;
                    }
                };
                handle_capture_get(writer, id);
            }
            "PATCH" => {
                let id = match param.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        writer.put_error(404, "Incorrect ID type for capture, ID should be i32.");
                        return;
                    }
                };
                let body = &request.body;
                let state = String::from_utf8(body.to_vec()).unwrap();
                match state.as_str() {
                    "1" => handle_capture_patch(writer, id, true),
                    "2" => handle_capture_patch(writer, id, false),
                    _ => writer.put_error(404, "Incorrect state for PatchCapture"),
                }
            }
            _ => writer.put_error(404, "Not found."),
        }
    }
}

/// Capture handler cxx for grpc server to call
pub fn handle_capture_cxx(
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
        request.uri = "/v1/captures".to_string();
    } else {
        request.uri = format!("/v1/captures/{}", param);
    }
    handle_capture(
        &request,
        param.as_str(),
        &mut CxxServerResponseWriterWrapper { writer: responder },
    );
}

/// Helper function for translating u32 representation of ChipKind
fn int_to_chip_kind(kind: u32) -> ChipKind {
    match kind {
        1 => ChipKind::BLUETOOTH,
        2 => ChipKind::WIFI,
        3 => ChipKind::UWB,
        _ => ChipKind::UNSPECIFIED,
    }
}

/// A common code for handle_request and handle_response cxx mehtods
fn handle_packet(
    kind: u32,
    facade_id: u32,
    packet: &CxxVector<u8>,
    packet_type: u32,
    direction: PacketDirection,
) {
    let captures_arc = get_captures_resource();
    let captures = captures_arc.write().unwrap();
    let facade_key = CaptureInfo::new_facade_key(int_to_chip_kind(kind), facade_id as i32);
    // TODO: Create event channel to invoke update_captures when new device is added.
    if !captures.facade_key_to_capture.contains_key(&facade_key) {
        update_captures();
    }
    if let Some(mut capture) = captures
        .facade_key_to_capture
        .get(&facade_key)
        .map(|arc_capture| arc_capture.lock().unwrap())
    {
        if let Some(ref mut file) = capture.file {
            if int_to_chip_kind(kind) == ChipKind::BLUETOOTH {
                let timestamp =
                    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                match append_record(timestamp, file, direction, packet_type, packet.as_slice()) {
                    Ok(size) => {
                        capture.size += size;
                        capture.records += 1;
                    }
                    Err(err) => {
                        error!("{err:?}");
                    }
                }
            }
        }
    };
}

/// Cxx Method for packet_hub to invoke (Host to Controller Packet Flow)
pub fn handle_packet_request(kind: u32, facade_id: u32, packet: &CxxVector<u8>, packet_type: u32) {
    handle_packet(kind, facade_id, packet, packet_type, PacketDirection::HostToController)
}

/// Cxx Method for packet_hub to invoke (Controller to Host Packet Flow)
pub fn handle_packet_response(kind: u32, facade_id: u32, packet: &CxxVector<u8>, packet_type: u32) {
    handle_packet(kind, facade_id, packet, packet_type, PacketDirection::ControllerToHost)
}

/// Cxx Method for clearing pcap files in temp directory
pub fn clear_pcap_files() -> bool {
    let mut path = system::netsimd_temp_dir();
    path.push("pcaps");

    // Check if the directory exists.
    if std::fs::metadata(&path).is_err() {
        return false;
    }

    // Delete the directory.
    std::fs::remove_dir_all(&path).is_ok()
}
