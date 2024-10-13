use std::net::Ipv4Addr;
use crate::sys;

#[derive(Clone)]
pub struct IPAddress {
    inner: sys::SteamIPAddress_t,
}

impl IPAddress {
    pub fn new(inner: sys::SteamIPAddress_t) -> Self {
        Self { inner }
    }

    pub fn get_ip_type(&self) -> sys::ESteamIPType {
        self.inner.m_eType
    }

    pub fn get_ipv4_address(&self) -> Ipv4Addr {
        unsafe {
            Ipv4Addr::from_bits(self.inner.__bindgen_anon_1.m_unIPv4)
        }
    }
}
