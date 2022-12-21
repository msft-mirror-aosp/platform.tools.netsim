//! Netsim cxx libraries.

mod frontend_http_server;
use crate::frontend_http_server::run_frontend_http_server;

#[cxx::bridge(namespace = "netsim")]
mod ffi {

    extern "Rust" {

        #[cxx_name = "StreamPacketHandler"]
        fn stream_packets_handler(packet_stream_client: UniquePtr<PacketStreamClient>);

        #[cxx_name = "RunFrontendHttpServer"]
        fn run_frontend_http_server();
    }

    unsafe extern "C++" {
        include!("backend/backend_server.h");

        type PacketStreamClient;

        #[rust_name = "read"]
        fn Read(&self) -> UniquePtr<CxxString>;
        #[rust_name = "write"]
        fn Write(&self, response: &CxxString);
    }
}

// A handler for StreamPackets method in c++ Grpc server.
pub fn stream_packets_handler(_client: cxx::UniquePtr<ffi::PacketStreamClient>) {
    // TODO: Stream packets.
}
