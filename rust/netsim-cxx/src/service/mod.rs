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

use crate::bluetooth as bluetooth_facade;
use crate::captures;
use crate::captures::handlers::clear_pcap_files;
use crate::config::get_dev;
use crate::http_server::run_http_server;
use crate::resource;
use crate::wifi as wifi_facade;
use log::info;
use log::warn;
use netsim_common::util::netsim_logger;
use std::env;

/// Module to control startup, run, and cleanup netsimd services.

pub struct ServiceParams {
    fd_startup_str: String,
    no_cli_ui: bool,
    no_web_ui: bool,
    hci_port: u16,
    instance_num: u16,
    dev: bool,
}

// TODO: Replace Run() in server.cc.

pub struct Service {
    // netsimd states, like device resource.
    service_params: ServiceParams,
}

impl Service {
    pub fn new(service_params: ServiceParams) -> Service {
        Service { service_params }
    }

    /// Sets up the states for netsimd.
    pub fn set_up(&self) {
        netsim_logger::init("netsimd");
        if clear_pcap_files() {
            info!("netsim generated pcap files in temp directory has been removed.");
        }

        // Start all the subscribers for events
        let events_rx = resource::clone_events().lock().unwrap().subscribe();
        captures::capture::spawn_capture_event_subscriber(events_rx);

        bluetooth_facade::bluetooth_start(self.service_params.instance_num);
        wifi_facade::wifi_start();
    }

    /// Runs the netsimd services.
    pub fn run(&self) {
        // TODO: run servers, like calling run_http_server().

        // Environment variable "NETSIM_GRPC_PORT" is set in forge
        // jobs. We do not run http server on forge.
        let forge_job =
            env::var("NETSIM_GRPC_PORT").map(|val| val.parse::<u32>().unwrap_or(0)).unwrap_or(0)
                != 0;

        // forge and no_web_ui disables the web server
        if !forge_job && !self.service_params.no_web_ui {
            run_http_server(self.service_params.instance_num);
        }

        if get_dev() {
            new_test_beacon(0);
            new_test_beacon(1);
        }
    }
}

// For cxx.
pub fn create_service(
    fd_startup_str: String,
    no_cli_ui: bool,
    no_web_ui: bool,
    hci_port: u16,
    instance_num: u16,
    dev: bool,
) -> Box<Service> {
    let service_params =
        ServiceParams { fd_startup_str, no_cli_ui, no_web_ui, hci_port, instance_num, dev };
    Box::new(Service::new(service_params))
}

pub fn new_test_beacon(idx: u32) {
    use crate::bluetooth::new_beacon;
    use frontend_proto::model::chip::bluetooth_beacon::{
        AdvertiseData as AdvertiseDataProto, AdvertiseSettings as AdvertiseSettingsProto,
    };
    use frontend_proto::model::chip_create::{
        BluetoothBeaconCreate as BluetoothBeaconCreateProto, Chip as ChipProto,
    };
    use frontend_proto::model::{ChipCreate as ChipCreateProto, DeviceCreate as DeviceCreateProto};
    use protobuf::MessageField;

    if let Err(err) = new_beacon(&DeviceCreateProto {
        name: format!("test-beacon-device-{idx}"),
        chips: vec![ChipCreateProto {
            name: format!("test-beacon-chip-{idx}"),
            chip: Some(ChipProto::BleBeacon(BluetoothBeaconCreateProto {
                address: format!("00:00:00:00:00:{:x}", idx),
                settings: MessageField::some(AdvertiseSettingsProto {
                    tx_power_level: 0,
                    interval: 1280,
                    ..Default::default()
                }),
                adv_data: MessageField::some(AdvertiseDataProto {
                    include_device_name: true,
                    include_tx_power_level: true,
                    manufacturer_data: vec![1u8, 2, 3, 4],
                    ..Default::default()
                }),
                ..Default::default()
            })),
            ..Default::default()
        }],
        ..Default::default()
    }) {
        warn!("Failed to create beacon device {idx}: {err}");
    }
}
