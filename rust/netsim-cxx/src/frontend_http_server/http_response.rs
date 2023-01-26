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

//! Response module for micro HTTP server.
//!
//! This library implements the basic parts of Response Message from
//! (RFC 5322)[ https://www.rfc-editor.org/rfc/rfc5322.html] "HTTP
//! Message Format."
//!
//! This library is only used for serving the netsim client and is not
//! meant to implement all aspects of RFC 5322.

use std::io::Write;

pub struct HttpResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let mut buffer = format!("HTTP/1.1 {}\r\n", self.status_code).into_bytes();
        for (name, value) in &self.headers {
            buffer.extend_from_slice(format!("{name}: {value}\r\n").as_bytes());
        }
        buffer.extend_from_slice(b"\r\n");
        buffer.extend_from_slice(&self.body);
        writer.write_all(&buffer)
    }

    pub fn new_200(content_type: &str, body: Vec<u8>) -> HttpResponse {
        HttpResponse {
            status_code: 200,
            headers: vec![
                ("Content-Type".to_owned(), content_type.to_owned()),
                ("Content-Length".to_owned(), body.len().to_string()),
            ],
            body,
        }
    }

    pub fn new_404() -> HttpResponse {
        HttpResponse {
            status_code: 404,
            headers: vec![("Content-Type".to_owned(), "text/plain".to_owned())],
            body: b"404 Not Found".to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_to() {
        let response = HttpResponse::new_200("text/plain", b"Hello World".to_vec());
        let mut stream = Cursor::new(Vec::new());
        response.write_to(&mut stream).unwrap();
        let written_bytes = stream.get_ref();
        let expected_bytes =
            b"HTTP/1.1 200\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nHello World";
        assert_eq!(written_bytes, expected_bytes);
    }
}
