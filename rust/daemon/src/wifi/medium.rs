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

use super::ieee80211::MacAddress;
use super::packets::mac80211_hwsim::{HwsimAttr, HwsimCmd, HwsimMsg, HwsimMsgHdr};
use super::packets::netlink::{NlAttrHdr, NlMsgHdr};
use crate::devices::chip::ChipIdentifier;
use crate::wifi::frame::Frame;
use crate::wifi::hwsim_attr_set::HwsimAttrSet;
use anyhow::{anyhow, Context};
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum HwsimCmdEnum {
    Unspec,
    Register,
    Frame(Box<Frame>),
    TxInfoFrame,
    NewRadio,
    DelRadio,
    GetRadio,
    AddMacAddr,
    DelMacAddr,
}

struct Station {
    client_id: u32,
    addr: MacAddress, // virtual interface mac address
}

pub struct Medium {
    callback: HwsimCmdCallback,
    stations: HashMap<MacAddress, Station>,
}

type HwsimCmdCallback = fn(u32, &[u8]);

impl Medium {
    pub fn new(callback: HwsimCmdCallback) -> Medium {
        Self { callback, stations: HashMap::new() }
    }
    fn get_station_by_addr(&self, addr: MacAddress) -> Option<&Station> {
        self.stations.get(&addr)
    }
}

impl Station {
    fn new(client_id: u32, addr: MacAddress) -> Self {
        Self { client_id, addr }
    }
}

/// Process commands from the kernel's mac80211_hwsim subsystem.
///
/// This is the processing that will be implemented:
///
/// * The source MacAddress in 802.11 frames is re-mapped to a globally
/// unique MacAddress because resumed Emulator AVDs appear with the
/// same address.
///
/// * 802.11 frames sent between stations
///
/// * 802.11 multicast frames are re-broadcast to connected stations.
///

impl Medium {
    pub fn process(&mut self, client_id: u32, packet: &[u8]) -> bool {
        match self.process_internal(client_id, packet) {
            Ok(b) => b,
            Err(e) => {
                warn!("error processing wifi {e}");
                // continue processing hwsim cmd on error
                false
            }
        }
    }

    fn process_internal(&mut self, client_id: u32, packet: &[u8]) -> anyhow::Result<bool> {
        let hwsim_msg = HwsimMsg::parse(packet)?;
        match (hwsim_msg.get_hwsim_hdr().hwsim_cmd) {
            HwsimCmd::Frame => {
                let frame = Frame::parse(&hwsim_msg)?;
                info!(
                    "Frame chip {}, addr {}, flags {}, cookie {:?}, ieee80211 {}",
                    client_id, frame.transmitter, frame.flags, frame.cookie, frame.ieee80211
                );
                let addr = frame.transmitter;
                // Creates Stations on the fly when there is no config file
                let _ = self.stations.entry(addr).or_insert_with(|| Station::new(client_id, addr));
                let sender = self.stations.get(&addr).unwrap();
                self.queue_frame(sender, frame)
            }
            HwsimCmd::AddMacAddr => {
                let attr_set = HwsimAttrSet::parse(hwsim_msg.get_attributes())?;
                if let (Some(addr), Some(hwaddr)) = (attr_set.transmitter, attr_set.receiver) {
                    info!("ADD_MAC_ADDR transmitter {:?} receiver {:?}", hwaddr, addr);
                } else {
                    warn!("ADD_MAC_ADDR missing transmitter or receiver");
                }
                Ok(false)
            }
            HwsimCmd::DelMacAddr => {
                let attr_set = HwsimAttrSet::parse(hwsim_msg.get_attributes())?;
                if let (Some(addr), Some(hwaddr)) = (attr_set.transmitter, attr_set.receiver) {
                    info!("DEL_MAC_ADDR transmitter {:?} receiver {:?}", hwaddr, addr);
                } else {
                    warn!("DEL_MAC_ADDR missing transmitter or receiver");
                }
                Ok(false)
            }
            _ => {
                info!("Another command found {:?}", hwsim_msg);
                Ok(false)
            }
        }
    }

    fn queue_frame(&self, station: &Station, frame: Frame) -> anyhow::Result<bool> {
        let destination = frame.ieee80211.get_destination();
        if let Some(station) = self.stations.get(&destination) {
            info!("Frame deliver from {} to {}", station.addr, destination);
            // rewrite packet to destination client: ToAP -> FromAP
            Ok(true)
        } else if destination.is_multicast() {
            info!("Frame multicast {}", frame.ieee80211);
            Ok(true)
        } else {
            // pass to libslirp
            Ok(false)
        }
    }
}

// It's usd by radiotap.rs for packet capture.
pub fn parse_hwsim_cmd(packet: &[u8]) -> anyhow::Result<HwsimCmdEnum> {
    let hwsim_msg = HwsimMsg::parse(packet)?;
    match (hwsim_msg.get_hwsim_hdr().hwsim_cmd) {
        HwsimCmd::Frame => {
            let frame = Frame::parse(&hwsim_msg)?;
            Ok(HwsimCmdEnum::Frame(Box::new(frame)))
        }
        _ => Err(anyhow!("Unknown HwsimMsg cmd={:?}", hwsim_msg.get_hwsim_hdr().hwsim_cmd)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_netlink_attr() {
        let packet: Vec<u8> = include!("test_packets/hwsim_cmd_frame.csv");
        assert!(parse_hwsim_cmd(&packet).is_ok());

        // missing transmitter attribute
        let packet2: Vec<u8> = include!("test_packets/hwsim_cmd_frame2.csv");
        assert!(parse_hwsim_cmd(&packet2).is_err());

        // missing cookie attribute
        let packet3: Vec<u8> = include!("test_packets/hwsim_cmd_frame_no_cookie.csv");
        assert!(parse_hwsim_cmd(&packet3).is_err());

        // HwsimkMsg cmd=TxInfoFrame packet
        let packet3: Vec<u8> = include!("test_packets/hwsim_cmd_tx_info.csv");
        assert!(parse_hwsim_cmd(&packet3).is_err());
    }

    #[test]
    fn test_is_mdns_packet() {
        let packet: Vec<u8> = include!("test_packets/hwsim_cmd_frame_mdns.csv");
        let hwsim_msg = HwsimMsg::parse(&packet).unwrap();
        let mdns_frame = Frame::parse(&hwsim_msg).unwrap();
        assert!(mdns_frame.ieee80211.get_destination().is_multicast());

        let packet: Vec<u8> = include!("test_packets/hwsim_cmd_frame.csv");
        let hwsim_msg = HwsimMsg::parse(&packet).unwrap();
        let non_mdns_frame = Frame::parse(&hwsim_msg).unwrap();
        assert!(!non_mdns_frame.ieee80211.get_destination().is_multicast());
    }
}
