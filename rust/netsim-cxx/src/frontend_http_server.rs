mod http_request;
mod http_response;
mod thread_pool;

extern crate frontend_proto;

use crate::frontend_http_server::http_request::HttpRequest;
use crate::frontend_http_server::http_response::HttpResponse;

use crate::frontend_http_server::thread_pool::ThreadPool;

use std::ffi::OsStr;
use std::fs;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

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

fn to_content_type(file_path: &Path) -> &str {
    match file_path.extension().and_then(OsStr::to_str) {
        Some("html") => "text/html",
        Some("txt") => "text/plain",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("js") => "application/javascript",
        _ => "application/octet-stream",
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut filepath = std::env::current_exe().unwrap();
    filepath.pop();
    filepath.push("netsim-ui");

    let http_response = if let Ok(request) =
        HttpRequest::parse::<&TcpStream>(&mut BufReader::new(&stream))
    {
        if request.method == "GET" {
            if request.uri == "/get-version" {
                HttpResponse::new_200("application/json", "{version: \"123b\"}".as_bytes().to_vec())
            } else {
                if request.uri == "/" {
                    filepath.push("index.html")
                } else {
                    filepath.push(&request.uri)
                }
                if let Ok(body) = fs::read(&filepath) {
                    HttpResponse::new_200(to_content_type(&filepath), body)
                } else {
                    HttpResponse::new_404()
                }
            }
        } else {
            // POST, PATCH etc.
            HttpResponse::new_404()
        }
    } else {
        // Request parse error
        HttpResponse::new_404()
    };
    if let Err(e) = http_response.write_to(&mut stream) {
        println!("handle_connection: error {e}");
    }
}
