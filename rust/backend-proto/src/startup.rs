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

//! Generated file from `netsim/startup.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.startup.StartupInfo)
pub struct StartupInfo {
    // message fields
    // @@protoc_insertion_point(field:netsim.startup.StartupInfo.devices)
    pub devices: ::std::vec::Vec<startup_info::Device>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.startup.StartupInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartupInfo {
    fn default() -> &'a StartupInfo {
        <StartupInfo as ::protobuf::Message>::default_instance()
    }
}

impl StartupInfo {
    pub fn new() -> StartupInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "devices",
            |m: &StartupInfo| { &m.devices },
            |m: &mut StartupInfo| { &mut m.devices },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartupInfo>(
            "StartupInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartupInfo {
    const NAME: &'static str = "StartupInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.devices.push(is.read_message()?);
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
        for value in &self.devices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.devices {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StartupInfo {
        StartupInfo::new()
    }

    fn clear(&mut self) {
        self.devices.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartupInfo {
        static instance: StartupInfo = StartupInfo {
            devices: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartupInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartupInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartupInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `StartupInfo`
pub mod startup_info {
    #[derive(PartialEq,Clone,Default,Debug)]
    // @@protoc_insertion_point(message:netsim.startup.StartupInfo.Device)
    pub struct Device {
        // message fields
        // @@protoc_insertion_point(field:netsim.startup.StartupInfo.Device.name)
        pub name: ::std::string::String,
        // @@protoc_insertion_point(field:netsim.startup.StartupInfo.Device.chips)
        pub chips: ::std::vec::Vec<super::Chip>,
        // special fields
        // @@protoc_insertion_point(special_field:netsim.startup.StartupInfo.Device.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Device {
        fn default() -> &'a Device {
            <Device as ::protobuf::Message>::default_instance()
        }
    }

    impl Device {
        pub fn new() -> Device {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "name",
                |m: &Device| { &m.name },
                |m: &mut Device| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
                "chips",
                |m: &Device| { &m.chips },
                |m: &mut Device| { &mut m.chips },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Device>(
                "StartupInfo.Device",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Device {
        const NAME: &'static str = "Device";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.name = is.read_string()?;
                    },
                    18 => {
                        self.chips.push(is.read_message()?);
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
            if !self.name.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.name);
            }
            for value in &self.chips {
                let len = value.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            };
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.name.is_empty() {
                os.write_string(1, &self.name)?;
            }
            for v in &self.chips {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
            };
            os.write_unknown_fields(self.special_fields.unknown_fields())?;
            ::std::result::Result::Ok(())
        }

        fn special_fields(&self) -> &::protobuf::SpecialFields {
            &self.special_fields
        }

        fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
            &mut self.special_fields
        }

        fn new() -> Device {
            Device::new()
        }

        fn clear(&mut self) {
            self.name.clear();
            self.chips.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Device {
            static instance: Device = Device {
                name: ::std::string::String::new(),
                chips: ::std::vec::Vec::new(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Device {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("StartupInfo.Device").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Device {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Device {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.startup.ChipInfo)
pub struct ChipInfo {
    // message fields
    // @@protoc_insertion_point(field:netsim.startup.ChipInfo.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.startup.ChipInfo.chip)
    pub chip: ::protobuf::MessageField<Chip>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.startup.ChipInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChipInfo {
    fn default() -> &'a ChipInfo {
        <ChipInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChipInfo {
    pub fn new() -> ChipInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &ChipInfo| { &m.name },
            |m: &mut ChipInfo| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Chip>(
            "chip",
            |m: &ChipInfo| { &m.chip },
            |m: &mut ChipInfo| { &mut m.chip },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChipInfo>(
            "ChipInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChipInfo {
    const NAME: &'static str = "ChipInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.chip)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(v) = self.chip.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(v) = self.chip.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> ChipInfo {
        ChipInfo::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.chip.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChipInfo {
        static instance: ChipInfo = ChipInfo {
            name: ::std::string::String::new(),
            chip: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChipInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChipInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChipInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChipInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.startup.Chip)
pub struct Chip {
    // message fields
    // @@protoc_insertion_point(field:netsim.startup.Chip.kind)
    pub kind: ::protobuf::EnumOrUnknown<super::common::ChipKind>,
    // @@protoc_insertion_point(field:netsim.startup.Chip.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.startup.Chip.manufacturer)
    pub manufacturer: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.startup.Chip.product_name)
    pub product_name: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.startup.Chip.fd_in)
    pub fd_in: i32,
    // @@protoc_insertion_point(field:netsim.startup.Chip.fd_out)
    pub fd_out: i32,
    // @@protoc_insertion_point(field:netsim.startup.Chip.loopback)
    pub loopback: bool,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.startup.Chip.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Chip {
    fn default() -> &'a Chip {
        <Chip as ::protobuf::Message>::default_instance()
    }
}

impl Chip {
    pub fn new() -> Chip {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "kind",
            |m: &Chip| { &m.kind },
            |m: &mut Chip| { &mut m.kind },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Chip| { &m.id },
            |m: &mut Chip| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "manufacturer",
            |m: &Chip| { &m.manufacturer },
            |m: &mut Chip| { &mut m.manufacturer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "product_name",
            |m: &Chip| { &m.product_name },
            |m: &mut Chip| { &mut m.product_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fd_in",
            |m: &Chip| { &m.fd_in },
            |m: &mut Chip| { &mut m.fd_in },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fd_out",
            |m: &Chip| { &m.fd_out },
            |m: &mut Chip| { &mut m.fd_out },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "loopback",
            |m: &Chip| { &m.loopback },
            |m: &mut Chip| { &mut m.loopback },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Chip>(
            "Chip",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Chip {
    const NAME: &'static str = "Chip";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.kind = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.id = is.read_string()?;
                },
                26 => {
                    self.manufacturer = is.read_string()?;
                },
                34 => {
                    self.product_name = is.read_string()?;
                },
                40 => {
                    self.fd_in = is.read_int32()?;
                },
                48 => {
                    self.fd_out = is.read_int32()?;
                },
                56 => {
                    self.loopback = is.read_bool()?;
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
        if self.kind != ::protobuf::EnumOrUnknown::new(super::common::ChipKind::UNSPECIFIED) {
            my_size += ::protobuf::rt::int32_size(1, self.kind.value());
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        }
        if !self.manufacturer.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.manufacturer);
        }
        if !self.product_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.product_name);
        }
        if self.fd_in != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.fd_in);
        }
        if self.fd_out != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.fd_out);
        }
        if self.loopback != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.kind != ::protobuf::EnumOrUnknown::new(super::common::ChipKind::UNSPECIFIED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.kind))?;
        }
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        }
        if !self.manufacturer.is_empty() {
            os.write_string(3, &self.manufacturer)?;
        }
        if !self.product_name.is_empty() {
            os.write_string(4, &self.product_name)?;
        }
        if self.fd_in != 0 {
            os.write_int32(5, self.fd_in)?;
        }
        if self.fd_out != 0 {
            os.write_int32(6, self.fd_out)?;
        }
        if self.loopback != false {
            os.write_bool(7, self.loopback)?;
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

    fn new() -> Chip {
        Chip::new()
    }

    fn clear(&mut self) {
        self.kind = ::protobuf::EnumOrUnknown::new(super::common::ChipKind::UNSPECIFIED);
        self.id.clear();
        self.manufacturer.clear();
        self.product_name.clear();
        self.fd_in = 0;
        self.fd_out = 0;
        self.loopback = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Chip {
        static instance: Chip = Chip {
            kind: ::protobuf::EnumOrUnknown::from_i32(0),
            id: ::std::string::String::new(),
            manufacturer: ::std::string::String::new(),
            product_name: ::std::string::String::new(),
            fd_in: 0,
            fd_out: 0,
            loopback: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Chip {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Chip").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Chip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Chip {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14netsim/startup.proto\x12\x0enetsim.startup\x1a\x13netsim/common.pr\
    oto\"\x95\x01\n\x0bStartupInfo\x12<\n\x07devices\x18\x01\x20\x03(\x0b2\"\
    .netsim.startup.StartupInfo.DeviceR\x07devices\x1aH\n\x06Device\x12\x12\
    \n\x04name\x18\x01\x20\x01(\tR\x04name\x12*\n\x05chips\x18\x02\x20\x03(\
    \x0b2\x14.netsim.startup.ChipR\x05chips\"H\n\x08ChipInfo\x12\x12\n\x04na\
    me\x18\x01\x20\x01(\tR\x04name\x12(\n\x04chip\x18\x02\x20\x01(\x0b2\x14.\
    netsim.startup.ChipR\x04chip\"\xd2\x01\n\x04Chip\x12+\n\x04kind\x18\x01\
    \x20\x01(\x0e2\x17.netsim.common.ChipKindR\x04kind\x12\x0e\n\x02id\x18\
    \x02\x20\x01(\tR\x02id\x12\"\n\x0cmanufacturer\x18\x03\x20\x01(\tR\x0cma\
    nufacturer\x12!\n\x0cproduct_name\x18\x04\x20\x01(\tR\x0bproductName\x12\
    \x13\n\x05fd_in\x18\x05\x20\x01(\x05R\x04fdIn\x12\x15\n\x06fd_out\x18\
    \x06\x20\x01(\x05R\x05fdOut\x12\x1a\n\x08loopback\x18\x07\x20\x01(\x08R\
    \x08loopbackb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::common::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(StartupInfo::generated_message_descriptor_data());
            messages.push(ChipInfo::generated_message_descriptor_data());
            messages.push(Chip::generated_message_descriptor_data());
            messages.push(startup_info::Device::generated_message_descriptor_data());
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
