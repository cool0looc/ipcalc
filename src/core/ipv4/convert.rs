//! IPv4 Format Conversion
//! 
//! Handles conversion between different IPv4 address formats.

use super::address::Ipv4Addr;
use serde::{Deserialize, Serialize};

/// IPv4 address in different formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Ipv4Formats {
    pub dotted: String,
    pub integer: u32,
    pub binary: String,
    pub hex: String,
}

#[allow(dead_code)]
impl Ipv4Formats {
    /// Create from an IPv4 address
    pub fn from_addr(addr: Ipv4Addr) -> Self {
        Self {
            dotted: addr.to_string(),
            integer: addr.to_int(),
            binary: addr.to_binary(),
            hex: addr.to_hex(),
        }
    }

    /// Create from an integer
    pub fn from_int(value: u32) -> Self {
        let addr = Ipv4Addr::from_int(value);
        Self::from_addr(addr)
    }
}

impl std::fmt::Display for Ipv4Formats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dotted:    {}\nInteger:   {}\nBinary:    {}\nHex:       {}",
            self.dotted, self.integer, self.binary, self.hex
        )
    }
}
