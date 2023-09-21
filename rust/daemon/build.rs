//
//  Copyright 2023 Google, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at:
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::env;
use std::path::PathBuf;

fn main() {
    let _build = cxx_build::bridge("src/ffi.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");

    let link_layer_prebuilt = env::var("LINK_LAYER_PACKETS_PREBUILT").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or("link_layer_packets.rs".to_owned()));
    std::fs::copy(
        link_layer_prebuilt.as_str(),
        out_dir.join("link_layer_packets.rs").as_os_str().to_str().unwrap(),
    )
    .unwrap();
}
