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

#![allow(dead_code)]

use log::info;

pub fn handle_uwb_request(_facade_id: u32, _packet: &[u8]) {
    info!("handle_uwb_request");
}

pub fn uwb_reset(_facade_id: u32) {
    info!("uwb_reset");
}
pub fn uwb_remove(_facade_id: u32) {
    info!("uwb_remove");
}

pub fn uwb_patch(_facade_id: u32, _proto_bytes: &[u8]) {
    info!("uwb_patch");
}

pub fn uwb_get(_facade_id: u32) -> Vec<u8> {
    info!("uwb_get");
    Vec::<u8>::new()
}

// Returns facade_id
pub fn uwb_add(_chip_id: u32) -> u32 {
    info!("uwb_get");
    1
}

pub fn uwb_start() {
    info!("uwb_start");
}

pub fn uwb_stop() {
    info!("uwb_stop");
}
