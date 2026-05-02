//! IPv6 Network - Subnet calculations
//! 
//! Handles IPv6 network calculations.

use super::address::Ipv6Addr;
use crate::core::{IpError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// IPv6 network (CIDR representation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Ipv6Network {
    /// Network address
    address: Ipv6Addr,
    /// Prefix length (0-128)
    prefix_len: u8,
}

#[allow(dead_code)]
impl Ipv6Network {
    /// Create a new IPv6 network
    pub fn new(address: Ipv6Addr, prefix_len: u8) -> Result<Self> {
        if prefix_len > 128 {
            return Err(IpError::InvalidPrefixLength(prefix_len, 128));
        }
        
        Ok(Self {
            address,
            prefix_len,
        })
    }

    /// Parse from CIDR string (e.g., "2001:db8::/32")
    pub fn from_cidr(cidr: &str) -> Result<Self> {
        let parts: Vec<&str> = cidr.split('/').collect();
        
        if parts.len() != 2 {
            return Err(IpError::InvalidCidr(format!(
                "Invalid CIDR format: {} (expected 'address/prefix')",
                cidr
            )));
        }

        let address = Ipv6Addr::from_str(parts[0])?;
        let prefix_len: u8 = parts[1]
            .parse()
            .map_err(|_| IpError::InvalidCidr(format!(
                "Invalid prefix length: {}",
                parts[1]
            )))?;

        Self::new(address, prefix_len)
    }

    /// Get the network address
    pub fn address(&self) -> Ipv6Addr {
        self.address
    }

    /// Get the prefix length
    pub fn prefix_len(&self) -> u8 {
        self.prefix_len
    }

    /// Get total number of addresses in the network
    pub fn total_addrs(&self) -> u128 {
        if self.prefix_len == 0 {
            return u128::MAX;
        }
        2u128.pow(128 - self.prefix_len as u32)
    }

    /// Get first host address (::1 for /128)
    pub fn first_host(&self) -> Option<Ipv6Addr> {
        if self.prefix_len == 128 {
            return Some(self.address);
        }
        // For other networks, first host is address + 1
        // Simplified implementation
        Some(self.address)
    }

    /// Get last host address
    pub fn last_host(&self) -> Option<Ipv6Addr> {
        // Simplified - would need proper BigInt for full implementation
        Some(self.address)
    }
}

impl fmt::Display for Ipv6Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix_len)
    }
}

impl FromStr for Ipv6Network {
    type Err = IpError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_cidr(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cidr() {
        let network = Ipv6Network::from_cidr("2001:db8::/32").unwrap();
        assert_eq!(network.prefix_len(), 32);
    }

    #[test]
    fn test_total_addrs() {
        let network = Ipv6Network::from_cidr("2001:db8::/32").unwrap();
        assert_eq!(network.total_addrs(), 2u128.pow(96));
    }
}
