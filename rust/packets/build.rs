//
//  Copyright 2024 Google, Inc.
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
    // Locate prebuilt pdl generated rust packet definition files
    let prebuilts: [[&str; 2]; 4] = [
        ["MAC80211_HWSIM_PACKETS_PREBUILT", "mac80211_hwsim_packets.rs"],
        ["IEEE80211_PACKETS_PREBUILT", "ieee80211_packets.rs"],
        ["LLC_PACKETS_PREBUILT", "llc_packets.rs"],
        ["NETLINK_PACKETS_PREBUILT", "netlink_packets.rs"],
    ];
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    for [var, name] in prebuilts {
        let env_prebuilt = env::var(var);
        let out_file = out_dir.join(name);
        // Check and use prebuilt pdl generated rust file from env var
        if env_prebuilt.is_ok() {
            let env_prebuilt = env_prebuilt.unwrap();
            println!("cargo:rerun-if-changed={}", env_prebuilt);
            std::fs::copy(env_prebuilt.as_str(), out_file.as_os_str().to_str().unwrap()).unwrap();
        // Prebuilt env var not set - check and use pdl generated file that is already present in out_dir
        } else if out_file.exists() {
            println!(
                "cargo:warning=env var {} not set. Using prebuilt found at: {}",
                var,
                out_file.display()
            );
        } else {
            panic!("Unable to find env var or prebuilt pdl generated rust file for: {}.", name);
        };
    }
}
