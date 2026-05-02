//! IPv4 Address - Parsing and validation
//! 
//! Handles IPv4 address representation, parsing from various formats,
//! and basic validation.

use crate::core::{IpError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::Ipv4Addr as StdIpv4Addr;
use std::str::FromStr;

/// IPv4 address representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ipv4Addr {
    /// The four octets of the IPv4 address
    octets: [u8; 4],
}

impl Ipv4Addr {
    /// Create a new IPv4 address from four octets
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            octets: [a, b, c, d],
        }
    }

    /// Create from a byte array
    #[allow(dead_code)]
    pub fn from_octets(octets: [u8; 4]) -> Self {
        Self { octets }
    }

    /// Get the octets as a slice
    #[allow(dead_code)]
    pub fn octets(&self) -> [u8; 4] {
        self.octets
    }

    /// Get the octets as a slice
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[u8; 4] {
        &self.octets
    }

    /// Convert to standard library Ipv4Addr
    #[allow(dead_code)]
    pub fn to_std(&self) -> StdIpv4Addr {
        StdIpv4Addr::from(self.octets)
    }

    /// Create from standard library Ipv4Addr
    #[allow(dead_code)]
    pub fn from_std(addr: StdIpv4Addr) -> Self {
        Self {
            octets: addr.octets(),
        }
    }

    /// Parse from dotted decimal string
    pub fn from_dotted(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        
        if parts.len() != 4 {
            return Err(IpError::InvalidFormat(format!(
                "Expected 4 octets, got {}",
                parts.len()
            )));
        }

        let mut octets = [0u8; 4];
        for (i, part) in parts.iter().enumerate() {
            let value: u32 = part
                .parse()
                .map_err(|_| IpError::InvalidFormat(format!(
                    "Invalid octet value: {}",
                    part
                )))?;
            
            if value > 255 {
                return Err(IpError::OctetOutOfRange(value));
            }
            
            octets[i] = value as u8;
        }

        Ok(Self { octets })
    }

    /// Parse from integer
    pub fn from_int(value: u32) -> Self {
        Self {
            octets: [
                (value >> 24) as u8,
                (value >> 16) as u8,
                (value >> 8) as u8,
                value as u8,
            ],
        }
    }

    /// Convert to integer
    pub fn to_int(&self) -> u32 {
        ((self.octets[0] as u32) << 24)
            | ((self.octets[1] as u32) << 16)
            | ((self.octets[2] as u32) << 8)
            | (self.octets[3] as u32)
    }

    /// Convert to binary string (dotted binary)
    pub fn to_binary(&self) -> String {
        self.octets
            .iter()
            .map(|o| format!("{:08b}", o))
            .collect::<Vec<_>>()
            .join(".")
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}{:02X}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }

    /// Validate the address
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        // All zeros or all ones are technically valid but reserved
        // For most purposes, we consider them valid but might want to warn
        true
    }

    /// Check if this is a loopback address (127.0.0.0/8)
    pub fn is_loopback(&self) -> bool {
        self.octets[0] == 127
    }

    /// Check if this is an unspecified address (0.0.0.0)
    pub fn is_unspecified(&self) -> bool {
        self.octets == [0, 0, 0, 0]
    }

    /// Check if this is a broadcast address (255.255.255.255)
    pub fn is_broadcast(&self) -> bool {
        self.octets == [255, 255, 255, 255]
    }

    /// Get the first octet
    pub fn first_octet(&self) -> u8 {
        self.octets[0]
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }
}

impl FromStr for Ipv4Addr {
    type Err = IpError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_dotted(s)
    }
}

impl PartialEq<u32> for Ipv4Addr {
    fn eq(&self, other: &u32) -> bool {
        self.to_int() == *other
    }
}

impl PartialOrd for Ipv4Addr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ipv4Addr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dotted() {
        let addr = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        assert_eq!(addr.octets, [192, 168, 1, 1]);
    }

    #[test]
    fn test_to_int() {
        let addr = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        assert_eq!(addr.to_int(), 3232235777);
    }

    #[test]
    fn test_from_int() {
        let addr = Ipv4Addr::from_int(3232235777);
        assert_eq!(addr.to_string(), "192.168.1.1");
    }

    #[test]
    fn test_to_binary() {
        let addr = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        assert_eq!(addr.to_binary(), "11000000.10101000.00000001.00000001");
    }

    #[test]
    fn test_invalid_octet() {
        assert!(Ipv4Addr::from_dotted("192.168.1.256").is_err());
        assert!(Ipv4Addr::from_dotted("192.168.1.-1").is_err());
    }

    #[test]
    fn test_comparison() {
        let a = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        let b = Ipv4Addr::from_dotted("192.168.1.2").unwrap();
        assert!(a < b);
    }
}
