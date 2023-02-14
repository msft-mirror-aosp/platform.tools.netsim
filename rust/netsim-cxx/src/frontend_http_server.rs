mod http_request;
mod http_response;
mod http_router;
mod thread_pool;

extern crate frontend_proto;

use crate::frontend_http_server::http_request::HttpRequest;
use crate::frontend_http_server::http_response::HttpResponse;
use crate::frontend_http_server::http_router::Router;
use crate::version::VERSION;

use crate::frontend_http_server::thread_pool::ThreadPool;

use crate::ffi::get_devices;
use crate::ffi::patch_device;
use cxx::let_cxx_string;
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
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn handle_file(method: &str, path: &str) -> HttpResponse {
    if method == "GET" {
        let mut filepath = std::env::current_exe().unwrap();
        filepath.pop();
        filepath.push("netsim-ui");
        if let Some(stripped_path) = path.strip_prefix('/') {
            filepath.push(stripped_path);
        } else {
            filepath.push(path);
        }
        if let Ok(body) = fs::read(&filepath) {
            return HttpResponse::new_200(to_content_type(&filepath), body);
        }
    }
    let body = format!("404 not found (netsim): handle_file with unknown path {path}");
    HttpResponse::new_404(body.into_bytes())
}

fn handle_pcap_file(request: &HttpRequest, id: &str) -> HttpResponse {
    if &request.method == "GET" {
        let mut filepath = std::env::current_exe().unwrap();
        filepath.pop();
        filepath.push("/tmp");
        filepath.push(format!("{id}-hci.pcap"));
        if let Ok(body) = fs::read(&filepath) {
            return HttpResponse::new_200(to_content_type(&filepath), body);
        }
    }
    let body = "404 not found (netsim): pcap file not exists for the device".to_string();
    HttpResponse::new_404(body.into_bytes())
}

// TODO handlers accept additional "context" including filepath
fn handle_index(request: &HttpRequest, _param: &str) -> HttpResponse {
    handle_file(&request.method, "index.html")
}

fn handle_static(request: &HttpRequest, path: &str) -> HttpResponse {
    handle_file(&request.method, path)
}

fn handle_version(_request: &HttpRequest, _param: &str) -> HttpResponse {
    HttpResponse::new_200(
        "text/plain",
        format!("{{version: \"{}\"}}", VERSION).into_bytes().to_vec(),
    )
}

fn handle_devices(request: &HttpRequest, _param: &str) -> HttpResponse {
    if &request.method == "GET" {
        let_cxx_string!(request = "");
        let_cxx_string!(response = "");
        let_cxx_string!(error_message = "");
        let status = get_devices(&request, response.as_mut(), error_message.as_mut());
        if status == 200 {
            HttpResponse::new_200("text/plain", response.to_string().into_bytes())
        } else {
            let body = format!("404 Not found (netsim): {:?}", error_message.to_string());
            HttpResponse::new_404(body.into_bytes())
        }
    } else if &request.method == "PATCH" {
        let_cxx_string!(new_request = &request.body);
        let_cxx_string!(response = "");
        let_cxx_string!(error_message = "");
        let status = patch_device(&new_request, response.as_mut(), error_message.as_mut());
        if status == 200 {
            HttpResponse::new_200("text/plain", response.to_string().into_bytes())
        } else {
            let body = format!("404 Not found (netsim): {:?}", error_message.to_string());
            HttpResponse::new_404(body.into_bytes())
        }
    } else {
        let body = format!(
            "404 Not found (netsim): {:?} is not a valid method for this route",
            request.method.to_string()
        );
        HttpResponse::new_404(body.into_bytes())
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut router = Router::new();
    router.add_route("/", handle_index);
    router.add_route("/get-version", handle_version);
    router.add_route("/v1/devices", handle_devices);
    router.add_route(r"/pcap/{id}", handle_pcap_file);
    router.add_route(r"/{path}", handle_static);
    let response =
        if let Ok(request) = HttpRequest::parse::<&TcpStream>(&mut BufReader::new(&stream)) {
            router.handle_request(&request)
        } else {
            let body = "404 not found (netsim): parse header failed".to_string();
            HttpResponse::new_404(body.into_bytes())
        };
    if let Err(e) = response.write_to(&mut stream) {
        println!("netsim: handle_connection error {e}");
    }
}
