// Copyright 2024 Google LLC
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

use crate::libslirp_sys::{self, SLIRP_MAX_DNS_SERVERS};
use log::warn;
use std::ffi::CString;
use std::io;
use std::net::SocketAddr;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use std::path::PathBuf;
use tokio;

const MAX_DNS_SERVERS: usize = SLIRP_MAX_DNS_SERVERS as usize;

// Rust SlirpConfig

pub struct SlirpConfig {
    pub version: u32,
    pub restricted: i32,
    pub in_enabled: bool,
    pub vnetwork: Ipv4Addr,
    pub vnetmask: Ipv4Addr,
    pub vhost: Ipv4Addr,
    pub in6_enabled: bool,
    pub vprefix_addr6: Ipv6Addr,
    pub vprefix_len: u8,
    pub vhost6: Ipv6Addr,
    pub vhostname: Option<String>,
    pub tftp_server_name: Option<String>,
    pub tftp_path: Option<PathBuf>,
    pub bootfile: Option<String>,
    pub vdhcp_start: Ipv4Addr,
    pub vnameserver: Ipv4Addr,
    pub vnameserver6: Ipv6Addr,
    pub vdnssearch: Vec<String>,
    pub vdomainname: Option<String>,
    pub if_mtu: usize,
    pub if_mru: usize,
    pub disable_host_loopback: bool,
    pub enable_emu: bool,
    pub outbound_addr: Option<SocketAddrV4>,
    pub outbound_addr6: Option<SocketAddrV6>,
    pub disable_dns: bool,
    pub disable_dhcp: bool,
    pub mfr_id: u32,
    pub oob_eth_addr: [u8; 6usize],
    pub http_proxy_on: bool,
    pub host_dns: Vec<SocketAddr>,
}

impl Default for SlirpConfig {
    fn default() -> Self {
        SlirpConfig {
            version: 5,
            // No restrictions by default
            restricted: 0,
            in_enabled: true,
            // Private network address
            vnetwork: Ipv4Addr::new(10, 0, 2, 0),
            vnetmask: Ipv4Addr::new(255, 255, 255, 0),
            // Default host address
            vhost: Ipv4Addr::new(10, 0, 2, 2),
            // IPv6 disabled by default
            in6_enabled: true,
            vprefix_addr6: "fec0::".parse().unwrap(),
            vprefix_len: 64,
            vhost6: "fec0::2".parse().unwrap(),
            vhostname: None, // Some("slirp".to_string()),
            tftp_server_name: None,
            tftp_path: None,
            bootfile: None,
            // DHCP starting address
            vdhcp_start: Ipv4Addr::new(10, 0, 2, 16),
            // Public DNS server
            vnameserver: Ipv4Addr::new(10, 0, 2, 3),
            vnameserver6: "fec0::3".parse().unwrap(),
            vdnssearch: Vec::new(),
            vdomainname: None,
            // Ethernet MTU
            if_mtu: 0,
            // Ethernet MRU
            if_mru: 0,
            disable_host_loopback: false,
            enable_emu: false,
            outbound_addr: None,
            outbound_addr6: None,
            disable_dns: false,
            disable_dhcp: false,
            mfr_id: 0,
            oob_eth_addr: [0; 6usize],
            http_proxy_on: false,
            host_dns: Vec::new(),
        }
    }
}

// Struct to hold a "C" SlirpConfig and the Rust storage that is
// referenced by SlirpConfig.
#[allow(dead_code)]
pub struct SlirpConfigs {
    pub c_slirp_config: libslirp_sys::SlirpConfig,

    // fields that hold the managed storage for "C" struct.
    c_bootfile: Option<CString>,
    c_tftp_server_name: Option<CString>,
    c_vdomainname: Option<CString>,
    c_vhostname: Option<CString>,
    c_tftp_path: Option<CString>,
    c_host_dns: [libslirp_sys::sockaddr_storage; MAX_DNS_SERVERS],
    // TODO: add other fields
}

pub async fn lookup_host_dns(host_dns: &str) -> io::Result<Vec<SocketAddr>> {
    let mut set = tokio::task::JoinSet::new();
    if host_dns.is_empty() {
        return Ok(Vec::new());
    }

    for addr in host_dns.split(",") {
        set.spawn(tokio::net::lookup_host(format!("{addr}:0")));
    }

    let mut addrs = Vec::new();
    while let Some(result) = set.join_next().await {
        addrs.push(result??.next().ok_or(io::Error::from(io::ErrorKind::NotFound))?);
    }
    Ok(addrs)
}

fn to_socketaddr_storage(dns: &[SocketAddr]) -> [libslirp_sys::sockaddr_storage; MAX_DNS_SERVERS] {
    let mut result = [libslirp_sys::sockaddr_storage::default(); MAX_DNS_SERVERS];
    if dns.len() > MAX_DNS_SERVERS {
        warn!("Too many DNS servers, only keeping the first {} ones", MAX_DNS_SERVERS);
    }
    for i in 0..usize::min(dns.len(), MAX_DNS_SERVERS) {
        result[i] = dns[i].into();
    }
    result
}

impl SlirpConfigs {
    pub fn new(config: &SlirpConfig) -> SlirpConfigs {
        let as_cstring =
            |s: &Option<String>| s.as_ref().and_then(|s| CString::new(s.as_bytes()).ok());
        let c_tftp_path = config
            .tftp_path
            .as_ref()
            .and_then(|s| CString::new(s.to_string_lossy().into_owned()).ok());
        let c_vhostname = as_cstring(&config.vhostname);
        let c_tftp_server_name = as_cstring(&config.tftp_server_name);
        let c_bootfile = as_cstring(&config.bootfile);
        let c_vdomainname = as_cstring(&config.vdomainname);

        let c_host_dns = to_socketaddr_storage(&config.host_dns);

        // Convert to a ptr::null() or a raw ptr to managed
        // memory. Whenever storing a ptr in "C" Struct using `as_ptr`
        // this code must have a Rust member is `SlirpConfigs` that
        // holds the storage.
        let as_ptr = |p: &Option<CString>| p.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());

        let c_slirp_config = libslirp_sys::SlirpConfig {
            version: config.version,
            restricted: config.restricted,
            in_enabled: config.in_enabled,
            vnetwork: config.vnetwork.into(),
            vnetmask: config.vnetmask.into(),
            vhost: config.vhost.into(),
            in6_enabled: config.in6_enabled,
            vprefix_addr6: config.vprefix_addr6.into(),
            vprefix_len: config.vprefix_len,
            vhost6: config.vhost6.into(),
            vhostname: as_ptr(&c_vhostname),
            tftp_server_name: as_ptr(&c_tftp_server_name),
            tftp_path: as_ptr(&c_tftp_path),
            bootfile: as_ptr(&c_bootfile),
            vdhcp_start: config.vdhcp_start.into(),
            vnameserver: config.vnameserver.into(),
            vnameserver6: config.vnameserver6.into(),
            // TODO: add field
            vdnssearch: std::ptr::null_mut(),
            vdomainname: as_ptr(&c_vdomainname),
            if_mtu: config.if_mtu,
            if_mru: config.if_mru,
            disable_host_loopback: config.disable_host_loopback,
            enable_emu: config.enable_emu,
            // TODO: add field
            outbound_addr: std::ptr::null_mut(),
            // TODO: add field
            outbound_addr6: std::ptr::null_mut(),
            disable_dns: config.disable_dns,
            disable_dhcp: config.disable_dhcp,
            mfr_id: config.mfr_id,
            oob_eth_addr: config.oob_eth_addr,
            http_proxy_on: config.http_proxy_on,
            host_dns_count: config.host_dns.len(),
            host_dns: c_host_dns,
        };

        // Return the "C" struct and Rust members holding the storage
        // referenced by the "C" struct.
        SlirpConfigs {
            c_slirp_config,
            c_vhostname,
            c_tftp_server_name,
            c_bootfile,
            c_vdomainname,
            c_tftp_path,
            c_host_dns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_slirp_config_default() {
        let config = SlirpConfig::default();

        assert_eq!(config.version, 5);
        assert_eq!(config.restricted, 0);
        assert!(config.in_enabled);
        assert_eq!(config.vnetwork, Ipv4Addr::new(10, 0, 2, 0));
        assert_eq!(config.vnetmask, Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(config.vhost, Ipv4Addr::new(10, 0, 2, 2));
        assert!(config.in6_enabled);
        assert_eq!(config.vprefix_addr6, "fec0::".parse::<Ipv6Addr>().unwrap());
        assert_eq!(config.vprefix_len, 64);
        assert_eq!(config.vhost6, "fec0::2".parse::<Ipv6Addr>().unwrap());
        assert_eq!(config.vhostname, None);
        assert_eq!(config.tftp_server_name, None);
        assert_eq!(config.tftp_path, None);
        assert_eq!(config.bootfile, None);
        assert_eq!(config.vdhcp_start, Ipv4Addr::new(10, 0, 2, 16));
        assert_eq!(config.vnameserver, Ipv4Addr::new(10, 0, 2, 3));
        assert_eq!(config.vnameserver6, "fec0::3".parse::<Ipv6Addr>().unwrap());
        assert!(config.vdnssearch.is_empty());
        assert_eq!(config.vdomainname, None);
        assert_eq!(config.if_mtu, 0);
        assert_eq!(config.if_mru, 0);
        assert!(!config.disable_host_loopback);
        assert!(!config.enable_emu);
        assert_eq!(config.outbound_addr, None);
        assert_eq!(config.outbound_addr6, None);
        assert!(!config.disable_dns);
        assert!(!config.disable_dhcp);
        assert_eq!(config.mfr_id, 0);
        assert_eq!(config.oob_eth_addr, [0; 6]);
        assert!(!config.http_proxy_on);
        assert_eq!(config.host_dns.len(), 0);
    }

    #[test]
    fn test_slirp_configs_new() {
        let rust_config = SlirpConfig::default();
        let c_configs = SlirpConfigs::new(&rust_config);

        // Check basic field conversions
        assert_eq!(c_configs.c_slirp_config.version, rust_config.version);
        assert_eq!(c_configs.c_slirp_config.restricted, rust_config.restricted);
        assert_eq!(c_configs.c_slirp_config.in_enabled as i32, rust_config.in_enabled as i32);

        // Check string conversions and null pointers
        assert_eq!(c_configs.c_slirp_config.vhostname, std::ptr::null());
        assert_eq!(c_configs.c_slirp_config.tftp_server_name, std::ptr::null());
    }

    #[test]
    fn test_lookup_host_dns() -> io::Result<()> {
        let rt = Runtime::new().unwrap();
        let results = rt.block_on(lookup_host_dns(""))?;
        assert_eq!(results.len(), 0);

        let results = rt.block_on(lookup_host_dns("localhost"))?;
        assert_eq!(results.len(), 1);

        let results = rt.block_on(lookup_host_dns("example.com"))?;
        assert_eq!(results.len(), 1);

        let results = rt.block_on(lookup_host_dns("localhost,example.com"))?;
        assert_eq!(results.len(), 2);
        Ok(())
    }

    #[test]
    fn test_to_socketaddr_storage_empty_input() {
        let dns: [SocketAddr; 0] = [];
        let result = to_socketaddr_storage(&dns);
        assert_eq!(result.len(), MAX_DNS_SERVERS);
        for entry in result {
            // Assuming `sockaddr_storage::default()` initializes all fields to 0
            assert_eq!(entry.ss_family, 0);
        }
    }

    #[test]
    fn test_to_socketaddr_storage() {
        let dns = ["1.1.1.1:53".parse().unwrap(), "8.8.8.8:53".parse().unwrap()];
        let result = to_socketaddr_storage(&dns);
        assert_eq!(result.len(), MAX_DNS_SERVERS);
        for i in 0..dns.len() {
            assert_ne!(result[i].ss_family, 0); // Converted addresses should have a non-zero family
        }
        for i in dns.len()..MAX_DNS_SERVERS {
            assert_eq!(result[i].ss_family, 0); // Remaining entries should be default
        }
    }

    #[test]
    fn test_to_socketaddr_storage_valid_input_at_max() {
        let dns = [
            "1.1.1.1:53".parse().unwrap(),
            "8.8.8.8:53".parse().unwrap(),
            "9.9.9.9:53".parse().unwrap(),
            "1.0.0.1:53".parse().unwrap(),
        ];
        let result = to_socketaddr_storage(&dns);
        assert_eq!(result.len(), MAX_DNS_SERVERS);
        for i in 0..dns.len() {
            assert_ne!(result[i].ss_family, 0);
        }
    }

    #[test]
    fn test_to_socketaddr_storage_input_exceeds_max() {
        let dns = [
            "1.1.1.1:53".parse().unwrap(),
            "8.8.8.8:53".parse().unwrap(),
            "9.9.9.9:53".parse().unwrap(),
            "1.0.0.1:53".parse().unwrap(),
            "1.2.3.4:53".parse().unwrap(), // Extra address
        ];
        let result = to_socketaddr_storage(&dns);
        assert_eq!(result.len(), MAX_DNS_SERVERS);
        for i in 0..MAX_DNS_SERVERS {
            assert_ne!(result[i].ss_family, 0);
        }
    }
}
