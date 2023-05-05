#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{c_char, c_int, c_short, c_uchar, c_ulong, c_ushort};
use std::{ffi::CStr, mem, ptr, str};

const IFNAMSIZ: u32 = 16;

#[repr(C)]
#[derive(Copy, Clone)]
/// Network interface ioctl request
///
/// See [man netdevice(7)](https://man7.org/linux/man-pages/man7/netdevice.7.html)
pub struct ifreq {
    pub ifr_ifrn: ifreq_ifrn,
    pub ifr_ifru: ifreq_ifru,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_ifreq {
    pub ifr6_addr: in6_addr,
    pub ifr6_prefixlen: ::std::os::raw::c_uint,
    pub ifr6_ifindex: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq_ifrn {
    pub ifrn_name: [c_char; 16usize],
    align: [u8; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq_ifru {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: ::std::os::raw::c_short,
    pub ifru_ifindex: ::std::os::raw::c_int,
    pub ifru_ivalue: ::std::os::raw::c_int,
    pub ifru_mtu: ::std::os::raw::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [c_char; 16usize],
    pub ifru_newname: [c_char; 16usize],
    pub ifru_data: *mut c_char,
    align: [u64; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Linux kernel IPv4 socket address representation
///
/// See [man ip(7)](https://man7.org/linux/man-pages/man7/ip.7.html)
pub struct sockaddr {
    // see https://man7.org/linux/man-pages/man7/ip.7.html for docs
    pub sa_family: ::std::os::raw::c_ushort,
    pub sa_data: [::std::os::raw::c_char; 14usize],
}

// See https://man7.org/linux/man-pages/man7/ipv6.7.html
pub type in6_addr = libc::in6_addr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ifmap {
    pub mem_start: c_ulong,
    pub mem_end: c_ulong,
    pub base_addr: c_ushort,
    pub irq: c_uchar,
    pub dma: c_uchar,
    pub port: c_uchar,
}

impl ifreq {
    pub fn new(name: &str) -> Self {
        let mut req: ifreq = unsafe { mem::zeroed() };
        if !name.is_empty() {
            let len = name.len().min(IFNAMSIZ as usize - 1);
            // Done just to make sure we don't truncate
            // on an UTF-8 code point boundary.
            let name = &name[..len];
            unsafe {
                ptr::copy_nonoverlapping(
                    name.as_ptr().cast::<c_char>(),
                    req.ifr_ifrn.ifrn_name.as_mut_ptr(),
                    len,
                );
            }
        }
        req
    }

    pub fn name(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(CStr::from_ptr(self.ifr_ifrn.ifrn_name.as_ptr()).to_bytes())
        }
    }
}
