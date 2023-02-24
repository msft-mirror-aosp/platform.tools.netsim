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

//! PCAP module for micro HTTP server.
//!
//! This module implements a handler for GET, PATCH, LIST pcap
//!
//! /v1/pcap --> handle_pcaps
//! /v1/pcap/{id} --> handle_pcap
//! handle_pcap_cxx calls handle_pcaps or handle_pcap based on the method

use crate::frontend_http_server::http_request::HttpRequest;
use crate::frontend_http_server::server_response::ServerResponseWritable;

/// The Rust pcap handler used directly by Http frontend for GET, PATCH
pub fn handle_pcap(request: &HttpRequest, _param: &str, writer: &mut dyn ServerResponseWritable) {
    match request.method.as_str() {
        "GET" => writer.put_ok("text/plain", "GetPcap"),
        "PATCH" => writer.put_ok("text/plain", "PatchPcap"),
        _ => {
            writer.put_error(404, "Not found.");
        }
    }
}

/// The Rust pcap handler used directly by Http frontend for LIST
pub fn handle_pcaps(request: &HttpRequest, _param: &str, writer: &mut dyn ServerResponseWritable) {
    match request.method.as_str() {
        "GET" => writer.put_ok("text/plain", "ListPcap"),
        _ => {
            writer.put_error(404, "Not found.");
        }
    }
}
