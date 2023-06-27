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

use std::{collections::HashMap, net::TcpStream};

use log::info;

use crate::http_server::{
    collect_query, http_request::HttpRequest, server_response::ResponseWritable,
};

// This feature is enabled only for CMake builds
#[cfg(feature = "local_ssl")]
use crate::openssl;

/// Generate Sec-Websocket-Accept value from given Sec-Websocket-Key value
fn generate_websocket_accept(websocket_key: String) -> String {
    let concat = websocket_key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let hashed = openssl::sha::sha1(concat.as_bytes());
    data_encoding::BASE64.encode(&hashed)
}

/// Check if all required fields exist in queries and request headers
fn has_required_websocket_fields(queries: &HashMap<&str, &str>, request: &HttpRequest) -> bool {
    queries.contains_key("name")
        && queries.contains_key("kind")
        && request.headers.get("Sec-Websocket-Key").is_some()
}

/// Handler for websocket server connection
pub fn handle_websocket(request: &HttpRequest, param: &str, writer: ResponseWritable) {
    match collect_query(param) {
        Ok(queries) if has_required_websocket_fields(&queries, request) => {
            let websocket_accept =
                generate_websocket_accept(request.headers.get("Sec-Websocket-Key").unwrap());
            writer.put_ok_switch_protocol(
                "websocket",
                &[("Sec-WebSocket-Accept", websocket_accept.as_str())],
            )
        }
        Ok(_) => writer.put_error(404, "Missing query fields and/or Sec-Websocket-Key"),
        Err(err) => writer.put_error(404, err),
    }
}

pub fn run_websocket_transport(_stream: TcpStream, queries: HashMap<&str, &str>) {
    info!("Running Websocket server");
    info!("Queries for Websocket: {queries:?}");
    todo!();
}
