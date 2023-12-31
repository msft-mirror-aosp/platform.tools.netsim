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

//! A utility module for writing pcap files
//!
//! This module includes writing appropriate pcap headers with given
//! linktype and appending records with header based on the assigned
//! protocol.

use std::{
    io::{Result, Write},
    time::Duration,
};
macro_rules! be_vec {
    ( $( $x:expr ),* ) => {
         Vec::<u8>::new().iter().copied()
         $( .chain($x.to_be_bytes()) )*
         .collect()
       };
    }

/// The indication of packet direction for HCI packets.
pub enum PacketDirection {
    /// Host To Controller as u32 value
    HostToController = 0,
    /// Controller to Host as u32 value
    ControllerToHost = 1,
}

/// Supported LinkTypes for packet capture
/// https://www.tcpdump.org/linktypes.html
pub enum LinkType {
    /// Radiotap link-layer information followed by an 802.11 header.
    Ieee802_11RadioTap = 127,
    /// Bluetooth HCI UART transport layer
    BluetoothHciH4WithPhdr = 201,
}

/// Returns the file size after writing the header of the
/// pcap file.
pub fn write_pcap_header<W: Write>(link_type: LinkType, output: &mut W) -> Result<usize> {
    // https://tools.ietf.org/id/draft-gharris-opsawg-pcap-00.html#name-file-header
    let header: Vec<u8> = be_vec![
        0xa1b2c3d4u32, // magic number
        2u16,          // major version
        4u16,          // minor version
        0u32,          // reserved 1
        0u32,          // reserved 2
        u32::MAX,      // snaplen
        link_type as u32
    ];

    output.write_all(&header)?;
    Ok(header.len())
}

/// Returns the file size after appending a single packet record.
pub fn append_record<W: Write>(
    timestamp: Duration,
    output: &mut W,
    packet_direction: PacketDirection,
    packet: &[u8],
) -> Result<usize> {
    // Record (direciton, type, packet)
    let record: Vec<u8> = be_vec![packet_direction as u32];

    // https://tools.ietf.org/id/draft-gharris-opsawg-pcap-00.html#name-packet-record
    let length = record.len() + packet.len();
    let header: Vec<u8> = be_vec![
        timestamp.as_secs() as u32, // seconds
        timestamp.subsec_micros(),  // microseconds
        length as u32,              // Captured Packet Length
        length as u32               // Original Packet Length
    ];
    let mut bytes = Vec::<u8>::with_capacity(header.len() + length);
    bytes.extend(&header);
    bytes.extend(&record);
    bytes.extend(packet);
    output.write_all(&bytes)?;
    output.flush()?;
    Ok(header.len() + length)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    static EXPECTED: &[u8; 76] = include_bytes!("sample.pcap");

    #[test]
    /// The test is done with the golden file sample.pcap with following packets:
    /// Packet 1: HCI_EVT from Controller to Host (Sent Command Complete (LE Set Advertise Enable))
    /// Packet 2: HCI_CMD from Host to Controller (Rcvd LE Set Advertise Enable) [250 milisecs later]
    fn test_pcap_file() {
        let mut actual = Vec::<u8>::new();
        write_pcap_header(LinkType::BluetoothHciH4WithPhdr, &mut actual).unwrap();
        append_record(
            Duration::from_secs(0),
            &mut actual,
            PacketDirection::HostToController,
            &[4, 14, 4, 1, 10, 32, 0],
        )
        .unwrap();
        append_record(
            Duration::from_millis(250),
            &mut actual,
            PacketDirection::ControllerToHost,
            &[1, 10, 32, 1, 0],
        )
        .unwrap();
        assert_eq!(actual, EXPECTED);
    }
}
