// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.21.12
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

//! Generated file from `netsim/config.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.config.SlirpOptions)
pub struct SlirpOptions {
    // message fields
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.disabled)
    pub disabled: bool,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.not_ipv4)
    pub not_ipv4: bool,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.restricted)
    pub restricted: bool,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vnet)
    pub vnet: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vhost)
    pub vhost: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vmask)
    pub vmask: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.not_ipv6)
    pub not_ipv6: bool,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vprefix6)
    pub vprefix6: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vprefixLen)
    pub vprefixLen: u32,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vhost6)
    pub vhost6: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.vhostname)
    pub vhostname: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.tftpath)
    pub tftpath: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.bootfile)
    pub bootfile: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.dhcpstart)
    pub dhcpstart: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.dns)
    pub dns: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.SlirpOptions.dns6)
    pub dns6: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.config.SlirpOptions.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SlirpOptions {
    fn default() -> &'a SlirpOptions {
        <SlirpOptions as ::protobuf::Message>::default_instance()
    }
}

impl SlirpOptions {
    pub fn new() -> SlirpOptions {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(16);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "disabled",
            |m: &SlirpOptions| { &m.disabled },
            |m: &mut SlirpOptions| { &mut m.disabled },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "not_ipv4",
            |m: &SlirpOptions| { &m.not_ipv4 },
            |m: &mut SlirpOptions| { &mut m.not_ipv4 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "restricted",
            |m: &SlirpOptions| { &m.restricted },
            |m: &mut SlirpOptions| { &mut m.restricted },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vnet",
            |m: &SlirpOptions| { &m.vnet },
            |m: &mut SlirpOptions| { &mut m.vnet },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vhost",
            |m: &SlirpOptions| { &m.vhost },
            |m: &mut SlirpOptions| { &mut m.vhost },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vmask",
            |m: &SlirpOptions| { &m.vmask },
            |m: &mut SlirpOptions| { &mut m.vmask },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "not_ipv6",
            |m: &SlirpOptions| { &m.not_ipv6 },
            |m: &mut SlirpOptions| { &mut m.not_ipv6 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vprefix6",
            |m: &SlirpOptions| { &m.vprefix6 },
            |m: &mut SlirpOptions| { &mut m.vprefix6 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vprefixLen",
            |m: &SlirpOptions| { &m.vprefixLen },
            |m: &mut SlirpOptions| { &mut m.vprefixLen },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vhost6",
            |m: &SlirpOptions| { &m.vhost6 },
            |m: &mut SlirpOptions| { &mut m.vhost6 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vhostname",
            |m: &SlirpOptions| { &m.vhostname },
            |m: &mut SlirpOptions| { &mut m.vhostname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tftpath",
            |m: &SlirpOptions| { &m.tftpath },
            |m: &mut SlirpOptions| { &mut m.tftpath },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bootfile",
            |m: &SlirpOptions| { &m.bootfile },
            |m: &mut SlirpOptions| { &mut m.bootfile },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dhcpstart",
            |m: &SlirpOptions| { &m.dhcpstart },
            |m: &mut SlirpOptions| { &mut m.dhcpstart },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dns",
            |m: &SlirpOptions| { &m.dns },
            |m: &mut SlirpOptions| { &mut m.dns },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dns6",
            |m: &SlirpOptions| { &m.dns6 },
            |m: &mut SlirpOptions| { &mut m.dns6 },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SlirpOptions>(
            "SlirpOptions",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SlirpOptions {
    const NAME: &'static str = "SlirpOptions";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.disabled = is.read_bool()?;
                },
                16 => {
                    self.not_ipv4 = is.read_bool()?;
                },
                24 => {
                    self.restricted = is.read_bool()?;
                },
                34 => {
                    self.vnet = is.read_string()?;
                },
                42 => {
                    self.vhost = is.read_string()?;
                },
                50 => {
                    self.vmask = is.read_string()?;
                },
                56 => {
                    self.not_ipv6 = is.read_bool()?;
                },
                66 => {
                    self.vprefix6 = is.read_string()?;
                },
                72 => {
                    self.vprefixLen = is.read_uint32()?;
                },
                82 => {
                    self.vhost6 = is.read_string()?;
                },
                90 => {
                    self.vhostname = is.read_string()?;
                },
                98 => {
                    self.tftpath = is.read_string()?;
                },
                106 => {
                    self.bootfile = is.read_string()?;
                },
                114 => {
                    self.dhcpstart = is.read_string()?;
                },
                122 => {
                    self.dns = is.read_string()?;
                },
                130 => {
                    self.dns6 = is.read_string()?;
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
        if self.disabled != false {
            my_size += 1 + 1;
        }
        if self.not_ipv4 != false {
            my_size += 1 + 1;
        }
        if self.restricted != false {
            my_size += 1 + 1;
        }
        if !self.vnet.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.vnet);
        }
        if !self.vhost.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.vhost);
        }
        if !self.vmask.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.vmask);
        }
        if self.not_ipv6 != false {
            my_size += 1 + 1;
        }
        if !self.vprefix6.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.vprefix6);
        }
        if self.vprefixLen != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.vprefixLen);
        }
        if !self.vhost6.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.vhost6);
        }
        if !self.vhostname.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.vhostname);
        }
        if !self.tftpath.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.tftpath);
        }
        if !self.bootfile.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.bootfile);
        }
        if !self.dhcpstart.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.dhcpstart);
        }
        if !self.dns.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.dns);
        }
        if !self.dns6.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.dns6);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.disabled != false {
            os.write_bool(1, self.disabled)?;
        }
        if self.not_ipv4 != false {
            os.write_bool(2, self.not_ipv4)?;
        }
        if self.restricted != false {
            os.write_bool(3, self.restricted)?;
        }
        if !self.vnet.is_empty() {
            os.write_string(4, &self.vnet)?;
        }
        if !self.vhost.is_empty() {
            os.write_string(5, &self.vhost)?;
        }
        if !self.vmask.is_empty() {
            os.write_string(6, &self.vmask)?;
        }
        if self.not_ipv6 != false {
            os.write_bool(7, self.not_ipv6)?;
        }
        if !self.vprefix6.is_empty() {
            os.write_string(8, &self.vprefix6)?;
        }
        if self.vprefixLen != 0 {
            os.write_uint32(9, self.vprefixLen)?;
        }
        if !self.vhost6.is_empty() {
            os.write_string(10, &self.vhost6)?;
        }
        if !self.vhostname.is_empty() {
            os.write_string(11, &self.vhostname)?;
        }
        if !self.tftpath.is_empty() {
            os.write_string(12, &self.tftpath)?;
        }
        if !self.bootfile.is_empty() {
            os.write_string(13, &self.bootfile)?;
        }
        if !self.dhcpstart.is_empty() {
            os.write_string(14, &self.dhcpstart)?;
        }
        if !self.dns.is_empty() {
            os.write_string(15, &self.dns)?;
        }
        if !self.dns6.is_empty() {
            os.write_string(16, &self.dns6)?;
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

    fn new() -> SlirpOptions {
        SlirpOptions::new()
    }

    fn clear(&mut self) {
        self.disabled = false;
        self.not_ipv4 = false;
        self.restricted = false;
        self.vnet.clear();
        self.vhost.clear();
        self.vmask.clear();
        self.not_ipv6 = false;
        self.vprefix6.clear();
        self.vprefixLen = 0;
        self.vhost6.clear();
        self.vhostname.clear();
        self.tftpath.clear();
        self.bootfile.clear();
        self.dhcpstart.clear();
        self.dns.clear();
        self.dns6.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SlirpOptions {
        static instance: SlirpOptions = SlirpOptions {
            disabled: false,
            not_ipv4: false,
            restricted: false,
            vnet: ::std::string::String::new(),
            vhost: ::std::string::String::new(),
            vmask: ::std::string::String::new(),
            not_ipv6: false,
            vprefix6: ::std::string::String::new(),
            vprefixLen: 0,
            vhost6: ::std::string::String::new(),
            vhostname: ::std::string::String::new(),
            tftpath: ::std::string::String::new(),
            bootfile: ::std::string::String::new(),
            dhcpstart: ::std::string::String::new(),
            dns: ::std::string::String::new(),
            dns6: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SlirpOptions {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SlirpOptions").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SlirpOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SlirpOptions {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.config.HostapdOptions)
pub struct HostapdOptions {
    // message fields
    // @@protoc_insertion_point(field:netsim.config.HostapdOptions.disabled)
    pub disabled: bool,
    // @@protoc_insertion_point(field:netsim.config.HostapdOptions.ssid)
    pub ssid: ::std::string::String,
    // @@protoc_insertion_point(field:netsim.config.HostapdOptions.passwd)
    pub passwd: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.config.HostapdOptions.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HostapdOptions {
    fn default() -> &'a HostapdOptions {
        <HostapdOptions as ::protobuf::Message>::default_instance()
    }
}

impl HostapdOptions {
    pub fn new() -> HostapdOptions {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "disabled",
            |m: &HostapdOptions| { &m.disabled },
            |m: &mut HostapdOptions| { &mut m.disabled },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ssid",
            |m: &HostapdOptions| { &m.ssid },
            |m: &mut HostapdOptions| { &mut m.ssid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "passwd",
            |m: &HostapdOptions| { &m.passwd },
            |m: &mut HostapdOptions| { &mut m.passwd },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HostapdOptions>(
            "HostapdOptions",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HostapdOptions {
    const NAME: &'static str = "HostapdOptions";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.disabled = is.read_bool()?;
                },
                18 => {
                    self.ssid = is.read_string()?;
                },
                26 => {
                    self.passwd = is.read_string()?;
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
        if self.disabled != false {
            my_size += 1 + 1;
        }
        if !self.ssid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ssid);
        }
        if !self.passwd.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.passwd);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.disabled != false {
            os.write_bool(1, self.disabled)?;
        }
        if !self.ssid.is_empty() {
            os.write_string(2, &self.ssid)?;
        }
        if !self.passwd.is_empty() {
            os.write_string(3, &self.passwd)?;
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

    fn new() -> HostapdOptions {
        HostapdOptions::new()
    }

    fn clear(&mut self) {
        self.disabled = false;
        self.ssid.clear();
        self.passwd.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HostapdOptions {
        static instance: HostapdOptions = HostapdOptions {
            disabled: false,
            ssid: ::std::string::String::new(),
            passwd: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HostapdOptions {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HostapdOptions").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HostapdOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HostapdOptions {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.config.WiFi)
pub struct WiFi {
    // message fields
    // @@protoc_insertion_point(field:netsim.config.WiFi.slirp_options)
    pub slirp_options: ::protobuf::MessageField<SlirpOptions>,
    // @@protoc_insertion_point(field:netsim.config.WiFi.hostapd_options)
    pub hostapd_options: ::protobuf::MessageField<HostapdOptions>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.config.WiFi.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WiFi {
    fn default() -> &'a WiFi {
        <WiFi as ::protobuf::Message>::default_instance()
    }
}

impl WiFi {
    pub fn new() -> WiFi {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, SlirpOptions>(
            "slirp_options",
            |m: &WiFi| { &m.slirp_options },
            |m: &mut WiFi| { &mut m.slirp_options },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, HostapdOptions>(
            "hostapd_options",
            |m: &WiFi| { &m.hostapd_options },
            |m: &mut WiFi| { &mut m.hostapd_options },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WiFi>(
            "WiFi",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WiFi {
    const NAME: &'static str = "WiFi";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.slirp_options)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.hostapd_options)?;
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
        if let Some(v) = self.slirp_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.hostapd_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.slirp_options.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.hostapd_options.as_ref() {
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

    fn new() -> WiFi {
        WiFi::new()
    }

    fn clear(&mut self) {
        self.slirp_options.clear();
        self.hostapd_options.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WiFi {
        static instance: WiFi = WiFi {
            slirp_options: ::protobuf::MessageField::none(),
            hostapd_options: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WiFi {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WiFi").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WiFi {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WiFi {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.config.Bluetooth)
pub struct Bluetooth {
    // message fields
    // @@protoc_insertion_point(field:netsim.config.Bluetooth.properties)
    pub properties: ::protobuf::MessageField<super::configuration::Controller>,
    // @@protoc_insertion_point(field:netsim.config.Bluetooth.address_reuse)
    pub address_reuse: ::std::option::Option<bool>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.config.Bluetooth.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Bluetooth {
    fn default() -> &'a Bluetooth {
        <Bluetooth as ::protobuf::Message>::default_instance()
    }
}

impl Bluetooth {
    pub fn new() -> Bluetooth {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::configuration::Controller>(
            "properties",
            |m: &Bluetooth| { &m.properties },
            |m: &mut Bluetooth| { &mut m.properties },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "address_reuse",
            |m: &Bluetooth| { &m.address_reuse },
            |m: &mut Bluetooth| { &mut m.address_reuse },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Bluetooth>(
            "Bluetooth",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Bluetooth {
    const NAME: &'static str = "Bluetooth";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.properties)?;
                },
                16 => {
                    self.address_reuse = ::std::option::Option::Some(is.read_bool()?);
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
        if let Some(v) = self.properties.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.address_reuse {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.properties.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.address_reuse {
            os.write_bool(2, v)?;
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

    fn new() -> Bluetooth {
        Bluetooth::new()
    }

    fn clear(&mut self) {
        self.properties.clear();
        self.address_reuse = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Bluetooth {
        static instance: Bluetooth = Bluetooth {
            properties: ::protobuf::MessageField::none(),
            address_reuse: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Bluetooth {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Bluetooth").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Bluetooth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bluetooth {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:netsim.config.Config)
pub struct Config {
    // message fields
    // @@protoc_insertion_point(field:netsim.config.Config.bluetooth)
    pub bluetooth: ::protobuf::MessageField<Bluetooth>,
    // @@protoc_insertion_point(field:netsim.config.Config.wifi)
    pub wifi: ::protobuf::MessageField<WiFi>,
    // special fields
    // @@protoc_insertion_point(special_field:netsim.config.Config.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Config {
    fn default() -> &'a Config {
        <Config as ::protobuf::Message>::default_instance()
    }
}

impl Config {
    pub fn new() -> Config {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Bluetooth>(
            "bluetooth",
            |m: &Config| { &m.bluetooth },
            |m: &mut Config| { &mut m.bluetooth },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, WiFi>(
            "wifi",
            |m: &Config| { &m.wifi },
            |m: &mut Config| { &mut m.wifi },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Config>(
            "Config",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Config {
    const NAME: &'static str = "Config";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.bluetooth)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.wifi)?;
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
        if let Some(v) = self.bluetooth.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.wifi.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.bluetooth.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.wifi.as_ref() {
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

    fn new() -> Config {
        Config::new()
    }

    fn clear(&mut self) {
        self.bluetooth.clear();
        self.wifi.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Config {
        static instance: Config = Config {
            bluetooth: ::protobuf::MessageField::none(),
            wifi: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Config {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Config").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Config {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Config {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13netsim/config.proto\x12\rnetsim.config\x1a\x1drootcanal/configurat\
    ion.proto\"\xac\x03\n\x0cSlirpOptions\x12\x1a\n\x08disabled\x18\x01\x20\
    \x01(\x08R\x08disabled\x12\x19\n\x08not_ipv4\x18\x02\x20\x01(\x08R\x07no\
    tIpv4\x12\x1e\n\nrestricted\x18\x03\x20\x01(\x08R\nrestricted\x12\x12\n\
    \x04vnet\x18\x04\x20\x01(\tR\x04vnet\x12\x14\n\x05vhost\x18\x05\x20\x01(\
    \tR\x05vhost\x12\x14\n\x05vmask\x18\x06\x20\x01(\tR\x05vmask\x12\x19\n\
    \x08not_ipv6\x18\x07\x20\x01(\x08R\x07notIpv6\x12\x1a\n\x08vprefix6\x18\
    \x08\x20\x01(\tR\x08vprefix6\x12\x1e\n\nvprefixLen\x18\t\x20\x01(\rR\nvp\
    refixLen\x12\x16\n\x06vhost6\x18\n\x20\x01(\tR\x06vhost6\x12\x1c\n\tvhos\
    tname\x18\x0b\x20\x01(\tR\tvhostname\x12\x18\n\x07tftpath\x18\x0c\x20\
    \x01(\tR\x07tftpath\x12\x1a\n\x08bootfile\x18\r\x20\x01(\tR\x08bootfile\
    \x12\x1c\n\tdhcpstart\x18\x0e\x20\x01(\tR\tdhcpstart\x12\x10\n\x03dns\
    \x18\x0f\x20\x01(\tR\x03dns\x12\x12\n\x04dns6\x18\x10\x20\x01(\tR\x04dns\
    6\"X\n\x0eHostapdOptions\x12\x1a\n\x08disabled\x18\x01\x20\x01(\x08R\x08\
    disabled\x12\x12\n\x04ssid\x18\x02\x20\x01(\tR\x04ssid\x12\x16\n\x06pass\
    wd\x18\x03\x20\x01(\tR\x06passwd\"\x90\x01\n\x04WiFi\x12@\n\rslirp_optio\
    ns\x18\x01\x20\x01(\x0b2\x1b.netsim.config.SlirpOptionsR\x0cslirpOptions\
    \x12F\n\x0fhostapd_options\x18\x02\x20\x01(\x0b2\x1d.netsim.config.Hosta\
    pdOptionsR\x0ehostapdOptions\"\xa0\x01\n\tBluetooth\x12H\n\nproperties\
    \x18\x01\x20\x01(\x0b2#.rootcanal.configuration.ControllerH\0R\nproperti\
    es\x88\x01\x01\x12(\n\raddress_reuse\x18\x02\x20\x01(\x08H\x01R\x0caddre\
    ssReuse\x88\x01\x01B\r\n\x0b_propertiesB\x10\n\x0e_address_reuse\"i\n\
    \x06Config\x126\n\tbluetooth\x18\x01\x20\x01(\x0b2\x18.netsim.config.Blu\
    etoothR\tbluetooth\x12'\n\x04wifi\x18\x02\x20\x01(\x0b2\x13.netsim.confi\
    g.WiFiR\x04wifib\x06proto3\
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
            deps.push(super::configuration::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(SlirpOptions::generated_message_descriptor_data());
            messages.push(HostapdOptions::generated_message_descriptor_data());
            messages.push(WiFi::generated_message_descriptor_data());
            messages.push(Bluetooth::generated_message_descriptor_data());
            messages.push(Config::generated_message_descriptor_data());
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