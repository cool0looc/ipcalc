//! IPv6 Address - Parsing and validation
//! 
//! Handles IPv6 address representation, parsing from various formats,
//! and basic validation.

use crate::core::{IpError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::Ipv6Addr as StdIpv6Addr;

/// IPv6 address representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Ipv6Addr {
    /// The eight 16-bit segments of the IPv6 address
    segments: [u16; 8],
}

#[allow(dead_code)]
impl Ipv6Addr {
    /// Create a new IPv6 address from eight segments
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self {
            segments: [a, b, c, d, e, f, g, h],
        }
    }

    /// Create from a 16-byte array
    pub fn from_segments(segments: [u16; 8]) -> Self {
        Self { segments }
    }

    /// Get the segments as a slice
    pub fn segments(&self) -> [u16; 8] {
        self.segments
    }

    /// Convert to standard library Ipv6Addr
    pub fn to_std(&self) -> StdIpv6Addr {
        let bytes: [u8; 16] = [
            (self.segments[0] >> 8) as u8, self.segments[0] as u8,
            (self.segments[1] >> 8) as u8, self.segments[1] as u8,
            (self.segments[2] >> 8) as u8, self.segments[2] as u8,
            (self.segments[3] >> 8) as u8, self.segments[3] as u8,
            (self.segments[4] >> 8) as u8, self.segments[4] as u8,
            (self.segments[5] >> 8) as u8, self.segments[5] as u8,
            (self.segments[6] >> 8) as u8, self.segments[6] as u8,
            (self.segments[7] >> 8) as u8, self.segments[7] as u8,
        ];
        StdIpv6Addr::from(bytes)
    }

    /// Create from standard library Ipv6Addr
    pub fn from_std(addr: StdIpv6Addr) -> Self {
        let bytes = addr.octets();
        let mut segments = [0u16; 8];
        for i in 0..8 {
            segments[i] = ((bytes[i * 2] as u16) << 8) | (bytes[i * 2 + 1] as u16);
        }
        Self { segments }
    }

    /// Parse from string (full or compressed)
    pub fn from_str(s: &str) -> Result<Self> {
        // Try parsing as-is first
        if let Ok(std_addr) = s.parse::<StdIpv6Addr>() {
            return Ok(Self::from_std(std_addr));
        }
        
        // Handle special cases
        Err(IpError::InvalidFormat(format!("Invalid IPv6 address: {}", s)))
    }

    /// Expand compressed address to full form
    pub fn expand(&self) -> String {
        format!(
            "{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}",
            self.segments[0], self.segments[1], self.segments[2], self.segments[3],
            self.segments[4], self.segments[5], self.segments[6], self.segments[7]
        )
    }

    /// Compress address (using :: notation)
    pub fn compress(&self) -> String {
        self.to_std().to_string()
    }

    /// Check if this is a loopback address (::1)
    pub fn is_loopback(&self) -> bool {
        self.segments == [0, 0, 0, 0, 0, 0, 0, 1]
    }

    /// Check if this is an unspecified address (::)
    pub fn is_unspecified(&self) -> bool {
        self.segments == [0, 0, 0, 0, 0, 0, 0, 0]
    }

    /// Check if this is a link-local address (fe80::/10)
    pub fn is_link_local(&self) -> bool {
        self.segments[0] & 0xffc0 == 0xfe80
    }

    /// Check if this is a unique local address (fc00::/7)
    #[allow(dead_code)]
    pub fn is_unique_local(&self) -> bool {
        (self.segments[0] & 0xfe00) == 0xfc00
    }

    /// Check if this is a multicast address (ff00::/8)
    #[allow(dead_code)]
    pub fn is_multicast(&self) -> bool {
        self.segments[0] & 0xff00 == 0xff00
    }

    /// Get the first 64 bits (network portion for /64)
    #[allow(dead_code)]
    pub fn network_bits(&self) -> [u16; 4] {
        [self.segments[0], self.segments[1], self.segments[2], self.segments[3]]
    }

    /// Get the last 64 bits (interface identifier)
    #[allow(dead_code)]
    pub fn interface_id(&self) -> [u16; 4] {
        [self.segments[4], self.segments[5], self.segments[6], self.segments[7]]
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.compress())
    }
}

impl std::str::FromStr for Ipv6Addr {
    type Err = IpError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loopback() {
        let addr = Ipv6Addr::from_str("::1").unwrap();
        assert!(addr.is_loopback());
    }

    #[test]
    fn test_expand() {
        let addr = Ipv6Addr::from_str("2001:db8::1").unwrap();
        assert_eq!(addr.expand(), "2001:0db8:0000:0000:0000:0000:0000:0001");
    }

    #[test]
    fn test_compress() {
        let addr = Ipv6Addr::from_str("2001:0db8:0000:0000:0000:0000:0000:0001").unwrap();
        assert_eq!(addr.compress(), "2001:db8::1");
    }
}
