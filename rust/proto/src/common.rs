// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.21.0-rc2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `netsim/common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:netsim.common.ChipKind)
pub enum ChipKind {
    // @@protoc_insertion_point(enum_value:netsim.common.ChipKind.UNSPECIFIED)
    UNSPECIFIED = 0,
    // @@protoc_insertion_point(enum_value:netsim.common.ChipKind.BLUETOOTH)
    BLUETOOTH = 1,
    // @@protoc_insertion_point(enum_value:netsim.common.ChipKind.WIFI)
    WIFI = 2,
    // @@protoc_insertion_point(enum_value:netsim.common.ChipKind.UWB)
    UWB = 3,
    // @@protoc_insertion_point(enum_value:netsim.common.ChipKind.BLUETOOTH_BEACON)
    BLUETOOTH_BEACON = 4,
}

impl ::protobuf::Enum for ChipKind {
    const NAME: &'static str = "ChipKind";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChipKind> {
        match value {
            0 => ::std::option::Option::Some(ChipKind::UNSPECIFIED),
            1 => ::std::option::Option::Some(ChipKind::BLUETOOTH),
            2 => ::std::option::Option::Some(ChipKind::WIFI),
            3 => ::std::option::Option::Some(ChipKind::UWB),
            4 => ::std::option::Option::Some(ChipKind::BLUETOOTH_BEACON),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ChipKind] = &[
        ChipKind::UNSPECIFIED,
        ChipKind::BLUETOOTH,
        ChipKind::WIFI,
        ChipKind::UWB,
        ChipKind::BLUETOOTH_BEACON,
    ];
}

impl ::protobuf::EnumFull for ChipKind {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ChipKind").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ChipKind {
    fn default() -> Self {
        ChipKind::UNSPECIFIED
    }
}

impl ChipKind {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ChipKind>("ChipKind")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13netsim/common.proto\x12\rnetsim.common*S\n\x08ChipKind\x12\x0f\n\
    \x0bUNSPECIFIED\x10\0\x12\r\n\tBLUETOOTH\x10\x01\x12\x08\n\x04WIFI\x10\
    \x02\x12\x07\n\x03UWB\x10\x03\x12\x14\n\x10BLUETOOTH_BEACON\x10\x04b\x06\
    proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(ChipKind::generated_enum_descriptor_data());
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