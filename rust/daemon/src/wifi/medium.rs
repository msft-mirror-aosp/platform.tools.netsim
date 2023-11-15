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

use super::packets::mac80211_hwsim::{HwsimAttr, HwsimMsg, HwsimMsgHdr};
use super::packets::netlink::{NlAttrHdr, NlMsgHdr};
use log::{info, warn};

const NLA_ALIGNTO: usize = 4;

fn nla_align(len: usize) -> usize {
    len.wrapping_add(NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

pub fn parse_hwsim_cmd_frame(packet: &[u8]) {
    match HwsimMsg::parse(packet) {
        Ok(hwsim_msg) => {
            info!(
                "HwsimkMsg len={} cmd={:?}",
                hwsim_msg.nl_hdr.nlmsg_len as usize
                    - std::mem::size_of::<NlMsgHdr>()
                    - std::mem::size_of::<HwsimMsgHdr>(),
                hwsim_msg.hwsim_hdr.hwsim_cmd
            );
            let mut index: usize = 0;
            let attributes = hwsim_msg.attributes;
            while (index < attributes.len()) {
                // Parse a generic netlink attribute to get the size
                let nla = NlAttrHdr::parse(&attributes[index..index + 4]).unwrap();
                let nla_len = nla.nla_len as usize;
                let hwsim_attr = HwsimAttr::parse(&attributes[index..index + nla_len]);
                info!("Attribute {:?}", hwsim_attr);
                index += nla_align(nla_len);
            }
        }
        Err(e) => {
            warn!("Unable to parse netlink message! {:?}", e);
        }
    }
}

pub fn test_parse_hwsim_cmd_frame() {
    let packet: Vec<u8> = vec![
        188, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 10, 0, 2, 0, 2, 21, 178, 0,
        0, 0, 0, 0, 98, 0, 3, 0, 64, 0, 0, 0, 255, 255, 255, 255, 255, 255, 74, 129, 38, 251, 211,
        154, 255, 255, 255, 255, 255, 255, 128, 12, 0, 0, 1, 8, 2, 4, 11, 22, 12, 18, 24, 36, 50,
        4, 48, 72, 96, 108, 45, 26, 126, 16, 27, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 255, 22, 35, 1, 120, 200, 26, 64, 0, 0, 191, 206, 0, 0, 0, 0, 0, 0,
        0, 0, 250, 255, 250, 255, 0, 0, 8, 0, 4, 0, 2, 0, 0, 0, 8, 0, 19, 0, 118, 9, 0, 0, 12, 0,
        7, 0, 0, 1, 255, 0, 255, 0, 255, 0, 16, 0, 21, 0, 0, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0,
        12, 0, 8, 0, 201, 0, 0, 0, 0, 0, 0, 0,
    ];
    parse_hwsim_cmd_frame(&packet);
}
