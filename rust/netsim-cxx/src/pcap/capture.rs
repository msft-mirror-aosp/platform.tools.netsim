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

use std::fs::{File, OpenOptions};
use std::io::Result;

use frontend_proto::{
    common::ChipKind,
    model::{Pcap as ProtoPcap, State},
};

use crate::ffi::get_facade_id;

use super::pcap_util::write_pcap_header;

pub struct Pcap {
    facade_id: i32,
    pub file: Option<File>,
    // Following items will be returned as ProtoPcap. (state: file.is_some())
    pub id: i32,
    chip_kind: ChipKind,
    device_name: String,
    pub size: usize,
    pub records: i32,
    timestamp: i32, // TODO: Creation time of File. (TimeStamp type in Protobuf)
    pub valid: bool,
}

impl Pcap {
    pub fn new(chip_kind: ChipKind, chip_id: i32, device_name: String) -> Self {
        Pcap {
            facade_id: get_facade_id(chip_id),
            id: chip_id,
            chip_kind,
            device_name,
            size: 0,
            records: 0,
            timestamp: 0,
            valid: true,
            file: None,
        }
    }

    // Creates a Pcap file with headers and store it under temp directory
    // The lifecycle of the file is NOT tied to the lifecycle of the struct
    // Format: /tmp/netsim-pcaps/{chip_id}-{device_name}-{chip_kind}.pcap
    pub fn start_capture(&mut self) -> Result<()> {
        let mut filename = std::env::temp_dir();
        filename.push("netsim-pcaps");
        std::fs::create_dir_all(&filename)?;
        filename.push(format!("{:?}-{:}-{:?}.pcap", self.id, self.device_name, self.chip_kind));
        let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(filename)?;
        let size = write_pcap_header(&mut file)?;
        self.size = size;
        self.records = 0;
        self.file = Some(file);
        Ok(())
    }

    // Closes file by removing ownership of self.file
    // Capture info will still retain the size and record count
    // So it can be downloaded easily when GetPcap is invoked.
    pub fn stop_capture(&mut self) {
        self.file = None;
    }

    pub fn new_facade_key(kind: ChipKind, facade_id: i32) -> (ChipKind, i32) {
        (kind, facade_id)
    }

    pub fn get_facade_key(&self) -> (ChipKind, i32) {
        Pcap::new_facade_key(self.chip_kind, self.facade_id)
    }

    pub fn get_pcap_proto(&self) -> ProtoPcap {
        ProtoPcap {
            id: self.id,
            chip_kind: self.chip_kind,
            chip_id: self.id,
            device_name: self.device_name.clone(),
            state: match self.file.is_some() {
                true => State::ON,
                false => State::OFF,
            },
            size: self.size as i32,
            records: self.records,
            timestamp: self.timestamp,
            valid: self.valid,
            ..Default::default()
        }
    }
}
