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

use crate::devices::devices_handler::{add_chip, remove_chip};
use crate::ffi::handle_request_cxx;
use crate::transport::h4;
use frontend_proto::common::ChipKind;
use lazy_static::lazy_static;
use log::{error, info};
use std::collections::HashMap;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::RwLock;
use std::thread;

// The HCI server implements the Bluetooth UART transport protocol
// (a.k.a. H4) over TCP. Each new connection on the HCI port spawns a
// new virtual controller associated with a new device.

/// Start the socket-based transport.
///
/// The socket transport reads/writes host-controller messages
/// for bluetooth (h4 hci) over a [TcpStream] transport.
///
pub fn run_socket_transport(hci_port: u16) {
    thread::Builder::new()
        .name("hci_transport".to_string())
        .spawn(move || {
            accept_incoming(hci_port)
                .unwrap_or_else(|e| println!("netsimd: Failed to accept incoming stream: {:?}", e));
        })
        .unwrap();
}

fn accept_incoming(hci_port: u16) -> std::io::Result<()> {
    let hci_socket = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, hci_port);
    let listener = TcpListener::bind(hci_socket)?;
    println!("netsimd: hci listening on: {}", hci_port);

    for stream in listener.incoming() {
        let stream = stream?;
        println!("netsimd: hci_client host addr: {}", stream.peer_addr().unwrap());
        thread::Builder::new()
            .name("hci_transport client".to_string())
            .spawn(move || {
                handle_hci_client(stream);
            })
            .unwrap();
    }
    Ok(())
}

// TRANSPORTS is a singleton that contains the outpurepr: et FiFo
lazy_static! {
    static ref TRANSPORTS: RwLock<HashMap<String, TcpStream>> = RwLock::new(HashMap::new());
}

fn get_key(kind: u32, facade_id: u32) -> String {
    format!("{}/{}", kind, facade_id)
}

pub fn handle_response(kind: u32, facade_id: u32, packet: &cxx::CxxVector<u8>, packet_type: u8) {
    let key = get_key(kind, facade_id);
    let binding = TRANSPORTS.read().unwrap();
    if let Some(mut stream) = binding.get(&key) {
        // todo add error checking
        let mut buffer = Vec::new();
        buffer.push(packet_type);
        buffer.extend(packet);
        if let Err(e) = stream.write_all(&buffer[..]) {
            println!("netsimd: error writing {}", e);
        };
    };
}

fn handle_hci_client(stream: TcpStream) {
    // ...
    let result = match add_chip(
        &stream.peer_addr().unwrap().port().to_string(),
        &format!("socket-{}", stream.peer_addr().unwrap()),
        ChipKind::BLUETOOTH,
        &format!("socket-{}", stream.peer_addr().unwrap()),
        "Google",
        "Google",
    ) {
        Ok(chip_result) => chip_result,
        Err(err) => {
            error!("{err}");
            return;
        }
    };
    let key = get_key(ChipKind::BLUETOOTH as u32, result.facade_id);
    let tcp_rx = stream.try_clone().unwrap();
    {
        TRANSPORTS.write().unwrap().insert(key.clone(), stream);
    }

    let _ = reader(tcp_rx, ChipKind::BLUETOOTH, result.facade_id);

    info!("remove chip: device {}, chip {}", result.device_id, result.chip_id);
    if let Err(err) = remove_chip(result.device_id, result.chip_id) {
        error!("{err}");
    };
    TRANSPORTS.write().unwrap().remove(&key);
}

/// read from the socket and pass to the packet hub.
///
fn reader(mut tcp_rx: TcpStream, kind: ChipKind, facade_id: u32) -> std::io::Result<()> {
    loop {
        if let ChipKind::BLUETOOTH = kind {
            match h4::read_h4_packet(&mut tcp_rx) {
                Ok(packet) => {
                    let kind: u32 = kind as u32;
                    handle_request_cxx(kind, facade_id, &packet.payload, packet.h4_type);
                }
                Err(error) => {
                    println!(
                        "netsimd: end socket reader {}: {:?}",
                        &tcp_rx.peer_addr().unwrap(),
                        error
                    );
                    return Ok(());
                }
            }
        }
    }
}
