// This file is generated by rust-protobuf 2.28.0. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `startup.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StartupInfo {
    // message fields
    pub devices: ::protobuf::RepeatedField<StartupInfo_Device>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
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

    // repeated .netsim.startup.StartupInfo.Device devices = 1;


    pub fn get_devices(&self) -> &[StartupInfo_Device] {
        &self.devices
    }
    pub fn clear_devices(&mut self) {
        self.devices.clear();
    }

    // Param is passed by value, moved
    pub fn set_devices(&mut self, v: ::protobuf::RepeatedField<StartupInfo_Device>) {
        self.devices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_devices(&mut self) -> &mut ::protobuf::RepeatedField<StartupInfo_Device> {
        &mut self.devices
    }

    // Take field
    pub fn take_devices(&mut self) -> ::protobuf::RepeatedField<StartupInfo_Device> {
        ::std::mem::replace(&mut self.devices, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StartupInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.devices {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.devices)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.devices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.devices {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> StartupInfo {
        StartupInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StartupInfo_Device>>(
                "devices",
                |m: &StartupInfo| { &m.devices },
                |m: &mut StartupInfo| { &mut m.devices },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StartupInfo>(
                "StartupInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StartupInfo {
        static instance: ::protobuf::rt::LazyV2<StartupInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StartupInfo::new)
    }
}

impl ::protobuf::Clear for StartupInfo {
    fn clear(&mut self) {
        self.devices.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartupInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StartupInfo_Device {
    // message fields
    pub serial: ::std::string::String,
    pub chips: ::protobuf::RepeatedField<Chip>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StartupInfo_Device {
    fn default() -> &'a StartupInfo_Device {
        <StartupInfo_Device as ::protobuf::Message>::default_instance()
    }
}

impl StartupInfo_Device {
    pub fn new() -> StartupInfo_Device {
        ::std::default::Default::default()
    }

    // string serial = 1;


    pub fn get_serial(&self) -> &str {
        &self.serial
    }
    pub fn clear_serial(&mut self) {
        self.serial.clear();
    }

    // Param is passed by value, moved
    pub fn set_serial(&mut self, v: ::std::string::String) {
        self.serial = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serial(&mut self) -> &mut ::std::string::String {
        &mut self.serial
    }

    // Take field
    pub fn take_serial(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.serial, ::std::string::String::new())
    }

    // repeated .netsim.startup.Chip chips = 2;


    pub fn get_chips(&self) -> &[Chip] {
        &self.chips
    }
    pub fn clear_chips(&mut self) {
        self.chips.clear();
    }

    // Param is passed by value, moved
    pub fn set_chips(&mut self, v: ::protobuf::RepeatedField<Chip>) {
        self.chips = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chips(&mut self) -> &mut ::protobuf::RepeatedField<Chip> {
        &mut self.chips
    }

    // Take field
    pub fn take_chips(&mut self) -> ::protobuf::RepeatedField<Chip> {
        ::std::mem::replace(&mut self.chips, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StartupInfo_Device {
    fn is_initialized(&self) -> bool {
        for v in &self.chips {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.serial)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chips)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.serial.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.serial);
        }
        for value in &self.chips {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.serial.is_empty() {
            os.write_string(1, &self.serial)?;
        }
        for v in &self.chips {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> StartupInfo_Device {
        StartupInfo_Device::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "serial",
                |m: &StartupInfo_Device| { &m.serial },
                |m: &mut StartupInfo_Device| { &mut m.serial },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chip>>(
                "chips",
                |m: &StartupInfo_Device| { &m.chips },
                |m: &mut StartupInfo_Device| { &mut m.chips },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StartupInfo_Device>(
                "StartupInfo.Device",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StartupInfo_Device {
        static instance: ::protobuf::rt::LazyV2<StartupInfo_Device> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StartupInfo_Device::new)
    }
}

impl ::protobuf::Clear for StartupInfo_Device {
    fn clear(&mut self) {
        self.serial.clear();
        self.chips.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartupInfo_Device {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartupInfo_Device {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ChipInfo {
    // message fields
    pub serial: ::std::string::String,
    pub chip: ::protobuf::SingularPtrField<Chip>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
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

    // string serial = 1;


    pub fn get_serial(&self) -> &str {
        &self.serial
    }
    pub fn clear_serial(&mut self) {
        self.serial.clear();
    }

    // Param is passed by value, moved
    pub fn set_serial(&mut self, v: ::std::string::String) {
        self.serial = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serial(&mut self) -> &mut ::std::string::String {
        &mut self.serial
    }

    // Take field
    pub fn take_serial(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.serial, ::std::string::String::new())
    }

    // .netsim.startup.Chip chip = 2;


    pub fn get_chip(&self) -> &Chip {
        self.chip.as_ref().unwrap_or_else(|| <Chip as ::protobuf::Message>::default_instance())
    }
    pub fn clear_chip(&mut self) {
        self.chip.clear();
    }

    pub fn has_chip(&self) -> bool {
        self.chip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chip(&mut self, v: Chip) {
        self.chip = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chip(&mut self) -> &mut Chip {
        if self.chip.is_none() {
            self.chip.set_default();
        }
        self.chip.as_mut().unwrap()
    }

    // Take field
    pub fn take_chip(&mut self) -> Chip {
        self.chip.take().unwrap_or_else(|| Chip::new())
    }
}

impl ::protobuf::Message for ChipInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.chip {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.serial)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.chip)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.serial.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.serial);
        }
        if let Some(ref v) = self.chip.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.serial.is_empty() {
            os.write_string(1, &self.serial)?;
        }
        if let Some(ref v) = self.chip.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ChipInfo {
        ChipInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "serial",
                |m: &ChipInfo| { &m.serial },
                |m: &mut ChipInfo| { &mut m.serial },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chip>>(
                "chip",
                |m: &ChipInfo| { &m.chip },
                |m: &mut ChipInfo| { &mut m.chip },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ChipInfo>(
                "ChipInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ChipInfo {
        static instance: ::protobuf::rt::LazyV2<ChipInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ChipInfo::new)
    }
}

impl ::protobuf::Clear for ChipInfo {
    fn clear(&mut self) {
        self.serial.clear();
        self.chip.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChipInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChipInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Chip {
    // message fields
    pub kind: Chip_ChipKind,
    pub id: ::std::string::String,
    pub manufacturer: ::std::string::String,
    pub model: ::std::string::String,
    pub fd_in: i32,
    pub fd_out: i32,
    pub loopback: bool,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
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

    // .netsim.startup.Chip.ChipKind kind = 1;


    pub fn get_kind(&self) -> Chip_ChipKind {
        self.kind
    }
    pub fn clear_kind(&mut self) {
        self.kind = Chip_ChipKind::UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: Chip_ChipKind) {
        self.kind = v;
    }

    // string id = 2;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string manufacturer = 3;


    pub fn get_manufacturer(&self) -> &str {
        &self.manufacturer
    }
    pub fn clear_manufacturer(&mut self) {
        self.manufacturer.clear();
    }

    // Param is passed by value, moved
    pub fn set_manufacturer(&mut self, v: ::std::string::String) {
        self.manufacturer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manufacturer(&mut self) -> &mut ::std::string::String {
        &mut self.manufacturer
    }

    // Take field
    pub fn take_manufacturer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.manufacturer, ::std::string::String::new())
    }

    // string model = 4;


    pub fn get_model(&self) -> &str {
        &self.model
    }
    pub fn clear_model(&mut self) {
        self.model.clear();
    }

    // Param is passed by value, moved
    pub fn set_model(&mut self, v: ::std::string::String) {
        self.model = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model(&mut self) -> &mut ::std::string::String {
        &mut self.model
    }

    // Take field
    pub fn take_model(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.model, ::std::string::String::new())
    }

    // int32 fd_in = 5;


    pub fn get_fd_in(&self) -> i32 {
        self.fd_in
    }
    pub fn clear_fd_in(&mut self) {
        self.fd_in = 0;
    }

    // Param is passed by value, moved
    pub fn set_fd_in(&mut self, v: i32) {
        self.fd_in = v;
    }

    // int32 fd_out = 6;


    pub fn get_fd_out(&self) -> i32 {
        self.fd_out
    }
    pub fn clear_fd_out(&mut self) {
        self.fd_out = 0;
    }

    // Param is passed by value, moved
    pub fn set_fd_out(&mut self, v: i32) {
        self.fd_out = v;
    }

    // bool loopback = 7;


    pub fn get_loopback(&self) -> bool {
        self.loopback
    }
    pub fn clear_loopback(&mut self) {
        self.loopback = false;
    }

    // Param is passed by value, moved
    pub fn set_loopback(&mut self, v: bool) {
        self.loopback = v;
    }
}

impl ::protobuf::Message for Chip {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.kind, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.manufacturer)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.model)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.fd_in = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.fd_out = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.loopback = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.kind != Chip_ChipKind::UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(1, self.kind);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        }
        if !self.manufacturer.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.manufacturer);
        }
        if !self.model.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.model);
        }
        if self.fd_in != 0 {
            my_size += ::protobuf::rt::value_size(5, self.fd_in, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fd_out != 0 {
            my_size += ::protobuf::rt::value_size(6, self.fd_out, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.loopback != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.kind != Chip_ChipKind::UNSPECIFIED {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.kind))?;
        }
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        }
        if !self.manufacturer.is_empty() {
            os.write_string(3, &self.manufacturer)?;
        }
        if !self.model.is_empty() {
            os.write_string(4, &self.model)?;
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
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Chip {
        Chip::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Chip_ChipKind>>(
                "kind",
                |m: &Chip| { &m.kind },
                |m: &mut Chip| { &mut m.kind },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &Chip| { &m.id },
                |m: &mut Chip| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "manufacturer",
                |m: &Chip| { &m.manufacturer },
                |m: &mut Chip| { &mut m.manufacturer },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "model",
                |m: &Chip| { &m.model },
                |m: &mut Chip| { &mut m.model },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "fd_in",
                |m: &Chip| { &m.fd_in },
                |m: &mut Chip| { &mut m.fd_in },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "fd_out",
                |m: &Chip| { &m.fd_out },
                |m: &mut Chip| { &mut m.fd_out },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "loopback",
                |m: &Chip| { &m.loopback },
                |m: &mut Chip| { &mut m.loopback },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Chip>(
                "Chip",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Chip {
        static instance: ::protobuf::rt::LazyV2<Chip> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Chip::new)
    }
}

impl ::protobuf::Clear for Chip {
    fn clear(&mut self) {
        self.kind = Chip_ChipKind::UNSPECIFIED;
        self.id.clear();
        self.manufacturer.clear();
        self.model.clear();
        self.fd_in = 0;
        self.fd_out = 0;
        self.loopback = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Chip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Chip {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Chip_ChipKind {
    UNSPECIFIED = 0,
    BLUETOOTH = 1,
    WIFI = 2,
    UWB = 3,
}

impl ::protobuf::ProtobufEnum for Chip_ChipKind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Chip_ChipKind> {
        match value {
            0 => ::std::option::Option::Some(Chip_ChipKind::UNSPECIFIED),
            1 => ::std::option::Option::Some(Chip_ChipKind::BLUETOOTH),
            2 => ::std::option::Option::Some(Chip_ChipKind::WIFI),
            3 => ::std::option::Option::Some(Chip_ChipKind::UWB),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Chip_ChipKind] = &[
            Chip_ChipKind::UNSPECIFIED,
            Chip_ChipKind::BLUETOOTH,
            Chip_ChipKind::WIFI,
            Chip_ChipKind::UWB,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Chip_ChipKind>("Chip.ChipKind", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Chip_ChipKind {
}

impl ::std::default::Default for Chip_ChipKind {
    fn default() -> Self {
        Chip_ChipKind::UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for Chip_ChipKind {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rstartup.proto\x12\x0enetsim.startup\"\x99\x01\n\x0bStartupInfo\x12<\
    \n\x07devices\x18\x01\x20\x03(\x0b2\".netsim.startup.StartupInfo.DeviceR\
    \x07devices\x1aL\n\x06Device\x12\x16\n\x06serial\x18\x01\x20\x01(\tR\x06\
    serial\x12*\n\x05chips\x18\x02\x20\x03(\x0b2\x14.netsim.startup.ChipR\
    \x05chips\"L\n\x08ChipInfo\x12\x16\n\x06serial\x18\x01\x20\x01(\tR\x06se\
    rial\x12(\n\x04chip\x18\x02\x20\x01(\x0b2\x14.netsim.startup.ChipR\x04ch\
    ip\"\x8a\x02\n\x04Chip\x121\n\x04kind\x18\x01\x20\x01(\x0e2\x1d.netsim.s\
    tartup.Chip.ChipKindR\x04kind\x12\x0e\n\x02id\x18\x02\x20\x01(\tR\x02id\
    \x12\"\n\x0cmanufacturer\x18\x03\x20\x01(\tR\x0cmanufacturer\x12\x14\n\
    \x05model\x18\x04\x20\x01(\tR\x05model\x12\x13\n\x05fd_in\x18\x05\x20\
    \x01(\x05R\x04fdIn\x12\x15\n\x06fd_out\x18\x06\x20\x01(\x05R\x05fdOut\
    \x12\x1a\n\x08loopback\x18\x07\x20\x01(\x08R\x08loopback\"=\n\x08ChipKin\
    d\x12\x0f\n\x0bUNSPECIFIED\x10\0\x12\r\n\tBLUETOOTH\x10\x01\x12\x08\n\
    \x04WIFI\x10\x02\x12\x07\n\x03UWB\x10\x03b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}