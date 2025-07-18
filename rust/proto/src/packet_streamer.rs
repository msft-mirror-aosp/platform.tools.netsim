// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.21.12
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `netsim/packet_streamer.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.packet.PacketRequest)
pub struct PacketRequest {
    // message oneof groups
    pub request_type: ::std::option::Option<packet_request::Request_type>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.packet.PacketRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacketRequest {
    fn default() -> &'a PacketRequest {
        <PacketRequest as ::protobuf::Message>::default_instance()
    }
}

impl PacketRequest {
    pub fn new() -> PacketRequest {
        ::std::default::Default::default()
    }

    // .netsim.startup.ChipInfo initial_info = 1;

    pub fn initial_info(&self) -> &super::startup::ChipInfo {
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::InitialInfo(ref v)) => v,
            _ => <super::startup::ChipInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_initial_info(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_initial_info(&self) -> bool {
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::InitialInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_initial_info(&mut self, v: super::startup::ChipInfo) {
        self.request_type = ::std::option::Option::Some(packet_request::Request_type::InitialInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_initial_info(&mut self) -> &mut super::startup::ChipInfo {
        if let ::std::option::Option::Some(packet_request::Request_type::InitialInfo(_)) = self.request_type {
        } else {
            self.request_type = ::std::option::Option::Some(packet_request::Request_type::InitialInfo(super::startup::ChipInfo::new()));
        }
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::InitialInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_initial_info(&mut self) -> super::startup::ChipInfo {
        if self.has_initial_info() {
            match self.request_type.take() {
                ::std::option::Option::Some(packet_request::Request_type::InitialInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::startup::ChipInfo::new()
        }
    }

    // .netsim.packet.HCIPacket hci_packet = 2;

    pub fn hci_packet(&self) -> &super::hci_packet::HCIPacket {
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::HciPacket(ref v)) => v,
            _ => <super::hci_packet::HCIPacket as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_hci_packet(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_hci_packet(&self) -> bool {
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::HciPacket(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hci_packet(&mut self, v: super::hci_packet::HCIPacket) {
        self.request_type = ::std::option::Option::Some(packet_request::Request_type::HciPacket(v))
    }

    // Mutable pointer to the field.
    pub fn mut_hci_packet(&mut self) -> &mut super::hci_packet::HCIPacket {
        if let ::std::option::Option::Some(packet_request::Request_type::HciPacket(_)) = self.request_type {
        } else {
            self.request_type = ::std::option::Option::Some(packet_request::Request_type::HciPacket(super::hci_packet::HCIPacket::new()));
        }
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::HciPacket(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hci_packet(&mut self) -> super::hci_packet::HCIPacket {
        if self.has_hci_packet() {
            match self.request_type.take() {
                ::std::option::Option::Some(packet_request::Request_type::HciPacket(v)) => v,
                _ => panic!(),
            }
        } else {
            super::hci_packet::HCIPacket::new()
        }
    }

    // bytes packet = 3;

    pub fn packet(&self) -> &[u8] {
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::Packet(ref v)) => v,
            _ => &[],
        }
    }

    pub fn clear_packet(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_packet(&self) -> bool {
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::Packet(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_packet(&mut self, v: ::std::vec::Vec<u8>) {
        self.request_type = ::std::option::Option::Some(packet_request::Request_type::Packet(v))
    }

    // Mutable pointer to the field.
    pub fn mut_packet(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(packet_request::Request_type::Packet(_)) = self.request_type {
        } else {
            self.request_type = ::std::option::Option::Some(packet_request::Request_type::Packet(::std::vec::Vec::new()));
        }
        match self.request_type {
            ::std::option::Option::Some(packet_request::Request_type::Packet(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_packet(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_packet() {
            match self.request_type.take() {
                ::std::option::Option::Some(packet_request::Request_type::Packet(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::startup::ChipInfo>(
            "initial_info",
            PacketRequest::has_initial_info,
            PacketRequest::initial_info,
            PacketRequest::mut_initial_info,
            PacketRequest::set_initial_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::hci_packet::HCIPacket>(
            "hci_packet",
            PacketRequest::has_hci_packet,
            PacketRequest::hci_packet,
            PacketRequest::mut_hci_packet,
            PacketRequest::set_hci_packet,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "packet",
            PacketRequest::has_packet,
            PacketRequest::packet,
            PacketRequest::set_packet,
        ));
        oneofs.push(packet_request::Request_type::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacketRequest>(
            "PacketRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacketRequest {
    const NAME: &'static str = "PacketRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.request_type = ::std::option::Option::Some(packet_request::Request_type::InitialInfo(is.read_message()?));
                },
                18 => {
                    self.request_type = ::std::option::Option::Some(packet_request::Request_type::HciPacket(is.read_message()?));
                },
                26 => {
                    self.request_type = ::std::option::Option::Some(packet_request::Request_type::Packet(is.read_bytes()?));
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.request_type {
            match v {
                &packet_request::Request_type::InitialInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &packet_request::Request_type::HciPacket(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &packet_request::Request_type::Packet(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(3, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.request_type {
            match v {
                &packet_request::Request_type::InitialInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &packet_request::Request_type::HciPacket(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &packet_request::Request_type::Packet(ref v) => {
                    os.write_bytes(3, v)?;
                },
            };
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PacketRequest {
        PacketRequest::new()
    }

    fn clear(&mut self) {
        self.request_type = ::std::option::Option::None;
        self.request_type = ::std::option::Option::None;
        self.request_type = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacketRequest {
        static instance: PacketRequest = PacketRequest {
            request_type: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacketRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacketRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacketRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PacketRequest`
pub mod packet_request {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:netsim.packet.PacketRequest.request_type)
    pub enum Request_type {
        // @@protoc_insertion_point(oneof_field:netsim.packet.PacketRequest.initial_info)
        InitialInfo(super::super::startup::ChipInfo),
        // @@protoc_insertion_point(oneof_field:netsim.packet.PacketRequest.hci_packet)
        HciPacket(super::super::hci_packet::HCIPacket),
        // @@protoc_insertion_point(oneof_field:netsim.packet.PacketRequest.packet)
        Packet(::std::vec::Vec<u8>),
    }

    impl ::protobuf::Oneof for Request_type {
    }

    impl ::protobuf::OneofFull for Request_type {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PacketRequest as ::protobuf::MessageFull>::descriptor().oneof_by_name("request_type").unwrap()).clone()
        }
    }

    impl Request_type {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Request_type>("request_type")
        }
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.packet.PacketResponse)
pub struct PacketResponse {
    // message oneof groups
    pub response_type: ::std::option::Option<packet_response::Response_type>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.packet.PacketResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacketResponse {
    fn default() -> &'a PacketResponse {
        <PacketResponse as ::protobuf::Message>::default_instance()
    }
}

impl PacketResponse {
    pub fn new() -> PacketResponse {
        ::std::default::Default::default()
    }

    // string error = 1;

    pub fn error(&self) -> &str {
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::Error(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_error(&mut self) {
        self.response_type = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::Error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response_type = ::std::option::Option::Some(packet_response::Response_type::Error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(packet_response::Response_type::Error(_)) = self.response_type {
        } else {
            self.response_type = ::std::option::Option::Some(packet_response::Response_type::Error(::std::string::String::new()));
        }
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::Error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response_type.take() {
                ::std::option::Option::Some(packet_response::Response_type::Error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .netsim.packet.HCIPacket hci_packet = 2;

    pub fn hci_packet(&self) -> &super::hci_packet::HCIPacket {
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::HciPacket(ref v)) => v,
            _ => <super::hci_packet::HCIPacket as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_hci_packet(&mut self) {
        self.response_type = ::std::option::Option::None;
    }

    pub fn has_hci_packet(&self) -> bool {
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::HciPacket(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hci_packet(&mut self, v: super::hci_packet::HCIPacket) {
        self.response_type = ::std::option::Option::Some(packet_response::Response_type::HciPacket(v))
    }

    // Mutable pointer to the field.
    pub fn mut_hci_packet(&mut self) -> &mut super::hci_packet::HCIPacket {
        if let ::std::option::Option::Some(packet_response::Response_type::HciPacket(_)) = self.response_type {
        } else {
            self.response_type = ::std::option::Option::Some(packet_response::Response_type::HciPacket(super::hci_packet::HCIPacket::new()));
        }
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::HciPacket(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hci_packet(&mut self) -> super::hci_packet::HCIPacket {
        if self.has_hci_packet() {
            match self.response_type.take() {
                ::std::option::Option::Some(packet_response::Response_type::HciPacket(v)) => v,
                _ => panic!(),
            }
        } else {
            super::hci_packet::HCIPacket::new()
        }
    }

    // bytes packet = 3;

    pub fn packet(&self) -> &[u8] {
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::Packet(ref v)) => v,
            _ => &[],
        }
    }

    pub fn clear_packet(&mut self) {
        self.response_type = ::std::option::Option::None;
    }

    pub fn has_packet(&self) -> bool {
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::Packet(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_packet(&mut self, v: ::std::vec::Vec<u8>) {
        self.response_type = ::std::option::Option::Some(packet_response::Response_type::Packet(v))
    }

    // Mutable pointer to the field.
    pub fn mut_packet(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(packet_response::Response_type::Packet(_)) = self.response_type {
        } else {
            self.response_type = ::std::option::Option::Some(packet_response::Response_type::Packet(::std::vec::Vec::new()));
        }
        match self.response_type {
            ::std::option::Option::Some(packet_response::Response_type::Packet(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_packet(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_packet() {
            match self.response_type.take() {
                ::std::option::Option::Some(packet_response::Response_type::Packet(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "error",
            PacketResponse::has_error,
            PacketResponse::error,
            PacketResponse::set_error,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::hci_packet::HCIPacket>(
            "hci_packet",
            PacketResponse::has_hci_packet,
            PacketResponse::hci_packet,
            PacketResponse::mut_hci_packet,
            PacketResponse::set_hci_packet,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "packet",
            PacketResponse::has_packet,
            PacketResponse::packet,
            PacketResponse::set_packet,
        ));
        oneofs.push(packet_response::Response_type::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacketResponse>(
            "PacketResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacketResponse {
    const NAME: &'static str = "PacketResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.response_type = ::std::option::Option::Some(packet_response::Response_type::Error(is.read_string()?));
                },
                18 => {
                    self.response_type = ::std::option::Option::Some(packet_response::Response_type::HciPacket(is.read_message()?));
                },
                26 => {
                    self.response_type = ::std::option::Option::Some(packet_response::Response_type::Packet(is.read_bytes()?));
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.response_type {
            match v {
                &packet_response::Response_type::Error(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &packet_response::Response_type::HciPacket(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &packet_response::Response_type::Packet(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(3, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.response_type {
            match v {
                &packet_response::Response_type::Error(ref v) => {
                    os.write_string(1, v)?;
                },
                &packet_response::Response_type::HciPacket(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &packet_response::Response_type::Packet(ref v) => {
                    os.write_bytes(3, v)?;
                },
            };
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PacketResponse {
        PacketResponse::new()
    }

    fn clear(&mut self) {
        self.response_type = ::std::option::Option::None;
        self.response_type = ::std::option::Option::None;
        self.response_type = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacketResponse {
        static instance: PacketResponse = PacketResponse {
            response_type: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacketResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacketResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacketResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PacketResponse`
pub mod packet_response {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:netsim.packet.PacketResponse.response_type)
    pub enum Response_type {
        // @@protoc_insertion_point(oneof_field:netsim.packet.PacketResponse.error)
        Error(::std::string::String),
        // @@protoc_insertion_point(oneof_field:netsim.packet.PacketResponse.hci_packet)
        HciPacket(super::super::hci_packet::HCIPacket),
        // @@protoc_insertion_point(oneof_field:netsim.packet.PacketResponse.packet)
        Packet(::std::vec::Vec<u8>),
    }

    impl ::protobuf::Oneof for Response_type {
    }

    impl ::protobuf::OneofFull for Response_type {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PacketResponse as ::protobuf::MessageFull>::descriptor().oneof_by_name("response_type").unwrap()).clone()
        }
    }

    impl Response_type {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Response_type>("response_type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cnetsim/packet_streamer.proto\x12\rnetsim.packet\x1a\x17netsim/hci_\
    packet.proto\x1a\x14netsim/startup.proto\"\xb3\x01\n\rPacketRequest\x12=\
    \n\x0cinitial_info\x18\x01\x20\x01(\x0b2\x18.netsim.startup.ChipInfoH\0R\
    \x0binitialInfo\x129\n\nhci_packet\x18\x02\x20\x01(\x0b2\x18.netsim.pack\
    et.HCIPacketH\0R\thciPacket\x12\x18\n\x06packet\x18\x03\x20\x01(\x0cH\0R\
    \x06packetB\x0e\n\x0crequest_type\"\x8e\x01\n\x0ePacketResponse\x12\x16\
    \n\x05error\x18\x01\x20\x01(\tH\0R\x05error\x129\n\nhci_packet\x18\x02\
    \x20\x01(\x0b2\x18.netsim.packet.HCIPacketH\0R\thciPacket\x12\x18\n\x06p\
    acket\x18\x03\x20\x01(\x0cH\0R\x06packetB\x0f\n\rresponse_type2b\n\x0ePa\
    cketStreamer\x12P\n\rStreamPackets\x12\x1c.netsim.packet.PacketRequest\
    \x1a\x1d.netsim.packet.PacketResponse(\x010\x01b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::hci_packet::file_descriptor().clone());
            deps.push(super::startup::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(PacketRequest::generated_message_descriptor_data());
            messages.push(PacketResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
