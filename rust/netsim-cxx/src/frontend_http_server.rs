mod http_request;
mod thread_pool;

extern crate frontend_proto;

use crate::frontend_http_server::http_request::HttpRequest;
use crate::frontend_http_server::thread_pool::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

const RESPONSE_200: &str = "HTTP/1.1 200 OK";
const RESPONSE_200_JS: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/javascript";
const RESPONSE_200_SVG: &str = "HTTP/1.1 200 OK\r\nContent-Type: image/svg+xml";
const RESPONSE_200_PNG: &str = "HTTP/1.1 200 OK\r\nContent-Type: image/png";
const RESPONSE_404: &str = "HTTP/1.1 404 NOT FOUND";

pub fn run_frontend_http_server() {
    let listener = TcpListener::bind("127.0.0.1:7681").unwrap();
    let pool = ThreadPool::new(4);
    println!("Frontend http server is listening on http://localhost:7681");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down frontend http server.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut filepath = std::env::current_exe().unwrap();
    filepath.pop();
    filepath.push("netsim-ui");

    if let Ok(request) = HttpRequest::parse::<&TcpStream>(&mut BufReader::new(&stream)) {
        if request.method == "GET" {
            let (status_line, mut contents) = if request.uri == "/" {
                filepath.push("index.html");
                if !filepath.exists() {
                    (RESPONSE_404, None)
                } else {
                    (RESPONSE_200, Some(fs::read_to_string(filepath.as_path()).unwrap()))
                }
            } else if request.uri == "/get-version" {
                (RESPONSE_200, Some("{version: \"123b\"}".to_string()))
            } else {
                filepath.push(&request.uri);
                if !filepath.exists() {
                    (RESPONSE_404, None)
                } else if request.uri.ends_with(".js") {
                    (RESPONSE_200_JS, Some(fs::read_to_string(filepath.as_path()).unwrap()))
                } else if request.uri.ends_with(".svg") {
                    (RESPONSE_200_SVG, Some(fs::read_to_string(filepath.as_path()).unwrap()))
                } else if request.uri.ends_with(".png") {
                    (RESPONSE_200_PNG, Some(fs::read_to_string(filepath.as_path()).unwrap()))
                } else {
                    (RESPONSE_200, Some(fs::read_to_string(filepath.as_path()).unwrap()))
                }
            };
            if contents.is_none() {
                contents = Some(String::from(include_str!("404.html")));
            }
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.as_ref().unwrap().len(),
                contents.unwrap().as_str()
            );
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
