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

use clap::Parser;
use log::warn;

use crate::args::NetsimdArgs;
use crate::ffi;
use crate::service::{Service, ServiceParams};

/// Wireless network simulator for android (and other) emulated devices.

#[no_mangle]
pub extern "C" fn rust_main() {
    ffi::set_up_crash_report();

    // TODO: Initialize netsim logger in the beginning of main.
    let netsimd_args = NetsimdArgs::parse();

    // Redirect stdout and stderr to files only if netsimd is not invoked
    // by Cuttlefish. Some Cuttlefish builds fail when writing logs to files.
    #[cfg(not(feature = "cuttlefish"))]
    if !netsimd_args.logtostderr {
        cxx::let_cxx_string!(netsimd_temp_dir = netsim_common::system::netsimd_temp_dir_string());
        ffi::redirect_std_stream(&netsimd_temp_dir);
    }

    let fd_startup_str = netsimd_args.fd_startup_str.unwrap_or_default();
    let no_cli_ui = netsimd_args.no_cli_ui;
    let no_web_ui = netsimd_args.no_web_ui;
    let instance_num = ffi::get_instance(netsimd_args.instance.unwrap_or_default());
    let hci_port: u16 = ffi::get_hci_port(netsimd_args.hci_port.unwrap_or_default(), instance_num)
        .try_into()
        .unwrap();
    let dev = netsimd_args.dev;

    #[cfg(feature = "cuttlefish")]
    if fd_startup_str.is_empty() {
        warn!("Failed to start netsim daemon because fd startup flag `-s` is empty");
        return;
    }

    if ffi::is_netsimd_alive(instance_num) {
        warn!("Failed to start netsim daemon because a netsim daemon is already running");
        return;
    }
    let service_params =
        ServiceParams::new(fd_startup_str, no_cli_ui, no_web_ui, hci_port, instance_num, dev);

    let service = unsafe { Service::new(service_params) };
    service.set_up();
    service.run();
}
