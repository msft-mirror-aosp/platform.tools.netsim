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

use frontend_proto::model::chip::Bluetooth;
use protobuf::Message;

pub fn handle_bluetooth_request(facade_id: u32, packet_type: u8, packet: &Vec<u8>) {
    crate::ffi::handle_bt_request(facade_id, packet_type, packet);
}

pub fn bluetooth_reset(facade_id: u32) {
    crate::ffi::bluetooth_reset(facade_id);
}

pub fn bluetooth_remove(facade_id: u32) {
    crate::ffi::bluetooth_remove(facade_id);
}

pub fn bluetooth_patch(facade_id: u32, bluetooth: &Bluetooth) {
    let bluetooth_bytes = bluetooth.write_to_bytes().unwrap();
    crate::ffi::bluetooth_patch_cxx(facade_id, &bluetooth_bytes);
}

pub fn bluetooth_get(facade_id: u32) -> Bluetooth {
    let bluetooth_bytes = crate::ffi::bluetooth_get_cxx(facade_id);
    Bluetooth::parse_from_bytes(&bluetooth_bytes).unwrap()
}

// Returns facade_id
pub fn bluetooth_add(device_id: u32) -> u32 {
    crate::ffi::bluetooth_add(device_id)
}

/// Starts the Bluetooth service.
pub fn bluetooth_start() {
    crate::ffi::bluetooth_start();
}

/// Stops the Bluetooth service.
pub fn bluetooth_stop() {
    crate::ffi::bluetooth_stop();
}