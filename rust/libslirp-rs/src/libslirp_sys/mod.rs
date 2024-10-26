//  Copyright 2024 Google, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at:
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

//! This module provides a safe and convenient wrapper around libslirp_sys,
//! allowing for easy integration of user-mode networking into Rust applications.
//!
//! It offers functionality for:
//!
//! - Converting C sockaddr_in and sockaddr_in6 to Rust types per OS
//! - Converting C sockaddr_storage type into the IPv6 and IPv4 variants
//!
//! # Example
//!
//! ```
//! use libslirp_rs::{Slirp, Config};
//!
//! let sockaddr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8080);
//! let storage: sockaddr_storage = sockaddr.into();
//!
//! // Interact with the Slirp instance
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// TODO(b/203002625) - since rustc 1.53, bindgen causes UB warnings
// Remove this once bindgen figures out how to do this correctly
#![allow(deref_nullptr)]

use std::convert::From;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg(target_os = "linux")]
include!("linux/bindings.rs");

#[cfg(target_os = "macos")]
include!("macos/bindings.rs");

#[cfg(target_os = "windows")]
include!("windows/bindings.rs");

impl Default for sockaddr_storage {
    /// Returns a zeroed `sockaddr_storage`.
    ///
    /// This is useful for uninitialied libslirp_config fields.
    ///
    /// This is safe because `sockaddr_storage` is a plain old data
    /// type with no padding or invariants, and a zeroed
    /// `sockaddr_storage` is a valid representation of "no address".
    fn default() -> Self {
        // Safety:
        //  * sockaddr_storage is repr(C) and has no uninitialized padding bytes.
        //  * Zeroing a sockaddr_storage is a valid initialization.
        unsafe { std::mem::zeroed() }
    }
}

fn v4_ref(storage: &sockaddr_storage) -> &sockaddr_in {
    // SAFETY: `sockaddr_storage` has size and alignment that is at least that of `sockaddr_in`.
    // Neither types have any padding.
    unsafe { &*(storage as *const sockaddr_storage as *const sockaddr_in) }
}

fn v6_ref(storage: &sockaddr_storage) -> &sockaddr_in6 {
    // SAFETY: `sockaddr_storage` has size and alignment that is at least that of `sockaddr_in6`.
    // Neither types have any padding.
    unsafe { &*(storage as *const sockaddr_storage as *const sockaddr_in6) }
}

fn v4_mut(storage: &mut sockaddr_storage) -> &mut sockaddr_in {
    // SAFETY: `sockaddr_storage` has size and alignment that is at least that of `sockaddr_in`.
    // Neither types have any padding.
    unsafe { &mut *(storage as *mut sockaddr_storage as *mut sockaddr_in) }
}

fn v6_mut(storage: &mut sockaddr_storage) -> &mut sockaddr_in6 {
    // SAFETY: `sockaddr_storage` has size and alignment that is at least that of `sockaddr_in6`.
    // Neither types have any padding.
    unsafe { &mut *(storage as *mut sockaddr_storage as *mut sockaddr_in6) }
}

// Type for libslirp poll bitfield mask SLIRP_POLL_nnn

#[cfg(target_os = "linux")]
pub type SlirpPollType = _bindgen_ty_7;

#[cfg(target_os = "macos")]
pub type SlirpPollType = _bindgen_ty_1;

#[cfg(target_os = "windows")]
pub type SlirpPollType = _bindgen_ty_5;

impl From<sockaddr_storage> for SocketAddr {
    /// Converts a `sockaddr_storage` to a `SocketAddr`.
    ///
    /// This function safely converts a `sockaddr_storage` from the
    /// `libslirp_sys` crate into a `std::net::SocketAddr`. It handles
    /// both IPv4 and IPv6 addresses by checking the `ss_family` field
    /// and casting the `sockaddr_storage` to the appropriate address
    /// type (`sockaddr_in` or `sockaddr_in6`).
    ///
    /// # Panics
    ///
    /// This function will panic if the `ss_family` field of the
    /// `sockaddr_storage` is not `AF_INET` or `AF_INET6`.
    fn from(storage: sockaddr_storage) -> Self {
        match storage.ss_family as u32 {
            AF_INET => SocketAddr::V4((*v4_ref(&storage)).into()),
            AF_INET6 => SocketAddr::V6((*v6_ref(&storage)).into()),
            _ => panic!("Unsupported address family"),
        }
    }
}

impl From<SocketAddr> for sockaddr_storage {
    /// Converts a `SocketAddr` to a `sockaddr_storage`.
    ///
    /// This function safely converts a `std::net::SocketAddr` into a
    /// `libslirp_sys::sockaddr_storage`. It handles both IPv4 and
    /// IPv6 addresses by writing the appropriate data into the
    /// `sockaddr_storage` structure.
    ///
    /// This conversion is useful when interacting with
    /// libslirp_config that expect a `sockaddr_storage` type.
    fn from(sockaddr: SocketAddr) -> Self {
        let mut storage = sockaddr_storage::default();

        match sockaddr {
            SocketAddr::V4(addr) => *v4_mut(&mut storage) = addr.into(),
            SocketAddr::V6(addr) => *v6_mut(&mut storage) = addr.into(),
        }
        storage
    }
}

impl Into<u32> for in_addr {
    fn into(self) -> u32 {
        #[cfg(target_os = "windows")]
        // SAFETY: This is safe because we are accessing a union field and
        // all fields in the union have the same size.
        unsafe {
            self.S_un.S_addr
        }

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        self.s_addr
    }
}

mod net {
    /// Converts a value from host byte order to network byte order.
    #[inline]
    pub fn htonl(hostlong: u32) -> u32 {
        hostlong.to_be()
    }

    /// Converts a value from network byte order to host byte order.
    #[inline]
    pub fn ntohl(netlong: u32) -> u32 {
        u32::from_be(netlong)
    }

    /// Converts a value from host byte order to network byte order.
    #[inline]
    pub fn htons(hostshort: u16) -> u16 {
        hostshort.to_be()
    }

    /// Converts a value from network byte order to host byte order.
    #[inline]
    pub fn ntohs(netshort: u16) -> u16 {
        u16::from_be(netshort)
    }
}

impl From<Ipv4Addr> for in_addr {
    fn from(item: Ipv4Addr) -> Self {
        #[cfg(target_os = "windows")]
        return in_addr {
            S_un: in_addr__bindgen_ty_1 { S_addr: std::os::raw::c_ulong::to_be(item.into()) },
        };

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        return in_addr { s_addr: net::htonl(item.into()) };
    }
}

impl From<in6_addr> for Ipv6Addr {
    fn from(item: in6_addr) -> Self {
        // SAFETY: Access union field. This is safe because we are
        // accessing the underlying byte array representation of the
        // `in6_addr` struct on macOS and all variants have the same
        // size.
        #[cfg(target_os = "macos")]
        return Ipv6Addr::from(unsafe { item.__u6_addr.__u6_addr8 });

        // SAFETY: Access union field. This is safe because we are
        // accessing the underlying byte array representation of the
        // `in6_addr` struct on Linux and all variants have the same
        // size.
        #[cfg(target_os = "linux")]
        return Ipv6Addr::from(unsafe { item.__in6_u.__u6_addr8 });

        // SAFETY: Access union field. This is safe because we are
        // accessing the underlying byte array representation of the
        // `in6_addr` struct on Windows and all variants have the same
        // size.
        #[cfg(target_os = "windows")]
        return Ipv6Addr::from(unsafe { item.u.Byte });
    }
}

impl From<Ipv6Addr> for in6_addr {
    fn from(item: Ipv6Addr) -> Self {
        #[cfg(target_os = "macos")]
        return in6_addr { __u6_addr: in6_addr__bindgen_ty_1 { __u6_addr8: item.octets() } };

        #[cfg(target_os = "linux")]
        return in6_addr { __in6_u: in6_addr__bindgen_ty_1 { __u6_addr8: item.octets() } };

        #[cfg(target_os = "windows")]
        return in6_addr { u: in6_addr__bindgen_ty_1 { Byte: item.octets() } };
    }
}

impl From<SocketAddrV4> for sockaddr_in {
    fn from(item: SocketAddrV4) -> Self {
        #[cfg(target_os = "macos")]
        return sockaddr_in {
            sin_len: 16u8,
            sin_family: AF_INET as u8,
            sin_port: net::htons(item.port()),
            sin_addr: (*item.ip()).into(),
            sin_zero: [0; 8],
        };

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        return sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: net::htons(item.port()),
            sin_addr: (*item.ip()).into(),
            sin_zero: [0; 8],
        };
    }
}

impl From<sockaddr_in> for SocketAddrV4 {
    fn from(item: sockaddr_in) -> Self {
        SocketAddrV4::new(
            Ipv4Addr::from(net::ntohl(item.sin_addr.into())),
            net::ntohs(item.sin_port),
        )
    }
}

impl From<sockaddr_in6> for SocketAddrV6 {
    fn from(item: sockaddr_in6) -> Self {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        return SocketAddrV6::new(
            Ipv6Addr::from(item.sin6_addr),
            item.sin6_port,
            item.sin6_flowinfo,
            item.sin6_scope_id,
        );

        #[cfg(target_os = "windows")]
        return SocketAddrV6::new(
            Ipv6Addr::from(item.sin6_addr),
            item.sin6_port,
            item.sin6_flowinfo,
            // SAFETY: This is safe because we are accessing a union
            // field where all fields have the same size.
            unsafe { item.__bindgen_anon_1.sin6_scope_id },
        );
    }
}

impl From<SocketAddrV6> for sockaddr_in6 {
    fn from(item: SocketAddrV6) -> Self {
        #[cfg(target_os = "windows")]
        return sockaddr_in6 {
            sin6_addr: (*item.ip()).into(),
            sin6_family: AF_INET6 as u16,
            sin6_port: item.port(),
            sin6_flowinfo: item.flowinfo(),
            __bindgen_anon_1: sockaddr_in6__bindgen_ty_1 { sin6_scope_id: item.scope_id() },
        };

        #[cfg(target_os = "macos")]
        return sockaddr_in6 {
            sin6_addr: (*item.ip()).into(),
            sin6_family: AF_INET6 as u8,
            sin6_port: item.port(),
            sin6_flowinfo: item.flowinfo(),
            sin6_scope_id: item.scope_id(),
            sin6_len: 16,
        };

        #[cfg(target_os = "linux")]
        return sockaddr_in6 {
            sin6_addr: (*item.ip()).into(),
            sin6_family: AF_INET6 as u16,
            sin6_port: item.port(),
            sin6_flowinfo: item.flowinfo(),
            sin6_scope_id: item.scope_id(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This tests a bidirectional conversion between sockaddr_storage
    // and SocketAddr
    #[test]
    fn test_sockaddr_storage() {
        let sockaddr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8080);
        let storage: sockaddr_storage = sockaddr.into();

        let sockaddr_from_storage: SocketAddr = storage.into();

        assert_eq!(sockaddr, sockaddr_from_storage);
    }

    #[test]
    fn test_sockaddr_storage_v6() {
        let sockaddr = SocketAddr::new(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8).into(), 8080);
        let storage: sockaddr_storage = sockaddr.into();

        let sockaddr_from_storage: SocketAddr = storage.into();

        assert_eq!(sockaddr, sockaddr_from_storage);
    }
}
