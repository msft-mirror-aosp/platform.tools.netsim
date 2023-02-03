//! Netsim cxx libraries.

mod frontend_http_server;
mod ranging;
mod version;

use crate::frontend_http_server::run_frontend_http_server;
use crate::ranging::*;
use crate::version::*;

#[cxx::bridge(namespace = "netsim")]
mod ffi {

    extern "Rust" {

        #[cxx_name = "RunFrontendHttpServer"]
        fn run_frontend_http_server();

        // Ranging

        #[cxx_name = "DistanceToRssi"]
        fn distance_to_rssi(tx_power: i8, distance: f32) -> i8;

        // Version

        #[cxx_name = "GetVersion"]
        fn get_version() -> String;
    }

    unsafe extern "C++" {
        include!("controller/controller.h");

        #[allow(dead_code)]
        #[rust_name = "get_devices"]
        #[namespace = "netsim::scene_controller"]
        fn GetDevices(
            request: &CxxString,
            response: Pin<&mut CxxString>,
            error_message: Pin<&mut CxxString>,
        ) -> u32;

        #[allow(dead_code)]
        #[rust_name = "update_device"]
        #[namespace = "netsim::scene_controller"]
        fn UpdateDevice(
            request: &CxxString,
            response: Pin<&mut CxxString>,
            error_message: Pin<&mut CxxString>,
        ) -> u32;
    }
}
