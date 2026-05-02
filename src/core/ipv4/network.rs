//! IPv4 Network - Subnet calculations
//! 
//! Handles network address, broadcast address, and other subnet calculations.

use super::address::Ipv4Addr;
use crate::core::{IpError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// IPv4 network (CIDR representation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4Network {
    /// Network address
    address: Ipv4Addr,
    /// Prefix length (0-32)
    prefix_len: u8,
}

impl Ipv4Network {
    /// Create a new IPv4 network
    pub fn new(address: Ipv4Addr, prefix_len: u8) -> Result<Self> {
        if prefix_len > 32 {
            return Err(IpError::InvalidPrefixLength(prefix_len, 32));
        }
        
        // Ensure address is on network boundary
        let network_addr = address & Self::mask(prefix_len);
        Ok(Self {
            address: network_addr,
            prefix_len,
        })
    }

    /// Parse from CIDR string (e.g., "192.168.1.0/24")
    pub fn from_cidr(cidr: &str) -> Result<Self> {
        let parts: Vec<&str> = cidr.split('/').collect();
        
        if parts.len() != 2 {
            return Err(IpError::InvalidCidr(format!(
                "Invalid CIDR format: {} (expected 'address/prefix')",
                cidr
            )));
        }

        let address = Ipv4Addr::from_dotted(parts[0])?;
        let prefix_len: u8 = parts[1]
            .parse()
            .map_err(|_| IpError::InvalidCidr(format!(
                "Invalid prefix length: {}",
                parts[1]
            )))?;

        Self::new(address, prefix_len)
    }

    /// Get the network mask (e.g., 255.255.255.0 for /24)
    pub fn netmask(&self) -> Ipv4Addr {
        Self::mask(self.prefix_len)
    }

    /// Calculate netmask from prefix length
    pub fn mask(prefix_len: u8) -> Ipv4Addr {
        if prefix_len == 0 {
            return Ipv4Addr::new(0, 0, 0, 0);
        }
        
        let bits = u32::MAX << (32 - prefix_len);
        Ipv4Addr::from_int(bits)
    }

    /// Get the wildcard mask (inverse of netmask)
    pub fn wildcard(&self) -> Ipv4Addr {
        let wildcard_bits = u32::MAX >> self.prefix_len;
        Ipv4Addr::from_int(wildcard_bits)
    }

    /// Get the broadcast address
    pub fn broadcast(&self) -> Ipv4Addr {
        let host_bits = u32::MAX >> self.prefix_len;
        self.address + host_bits
    }

    /// Get the first usable host address
    pub fn first_host(&self) -> Option<Ipv4Addr> {
        if self.prefix_len >= 31 {
            // /31 and /32 are special cases
            if self.prefix_len == 31 {
                // Point-to-point links: 2 addresses, no broadcast
                let second = self.address + 1;
                return Some(second);
            }
            // /32: only one address, it's the host itself
            return Some(self.address);
        }
        
        Some(self.address + 1)
    }

    /// Get the last usable host address
    pub fn last_host(&self) -> Option<Ipv4Addr> {
        if self.prefix_len >= 31 {
            if self.prefix_len == 31 {
                return Some(self.address);
            }
            // /32: only one address
            return Some(self.address);
        }
        
        Some(self.broadcast() - 1)
    }

    /// Get total number of addresses in the network
    pub fn total_addrs(&self) -> u32 {
        2u32.pow(32 - self.prefix_len as u32)
    }

    /// Get total number of usable host addresses
    pub fn usable_hosts(&self) -> u32 {
        if self.prefix_len >= 31 {
            // /31: 2 addresses (used for point-to-point links)
            // /32: 1 address
            return if self.prefix_len == 31 { 2 } else { 1 };
        }
        
        let total = self.total_addrs();
        total - 2 // Subtract network and broadcast addresses
    }

    /// Check if an IP address is in this network
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        let network_int = self.address.to_int();
        let ip_int = ip.to_int();
        let mask_int = self.netmask().to_int();
        
        (ip_int & mask_int) == network_int
    }

    /// Get the network address
    pub fn address(&self) -> Ipv4Addr {
        self.address
    }

    /// Get the prefix length
    pub fn prefix_len(&self) -> u8 {
        self.prefix_len
    }

    /// Get IP range (start to end)
    pub fn range(&self) -> (Ipv4Addr, Ipv4Addr) {
        (self.address, self.broadcast())
    }

    /// Calculate required prefix length for a given number of hosts
    pub fn prefix_for_hosts(num_hosts: u32) -> u8 {
        if num_hosts <= 2 {
            if num_hosts == 1 {
                32
            } else {
                31
            }
        } else {
            // Need num_hosts + 2 addresses (network + broadcast)
            let needed = num_hosts + 2;
            let mut bits_needed = 32 - (needed.ilog2() as u8);
            // Ensure we have enough addresses
            if 2u32.pow(32 - bits_needed as u32) < needed {
                bits_needed -= 1;
            }
            bits_needed
        }
    }

    /// Get all IP addresses in the network (generator for memory efficiency)
    #[allow(dead_code)]
    pub fn iter(&self) -> Ipv4NetworkIter {
        Ipv4NetworkIter {
            current: self.address,
            end: self.broadcast(),
        }
    }
}

impl fmt::Display for Ipv4Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix_len)
    }
}

impl FromStr for Ipv4Network {
    type Err = IpError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_cidr(s)
    }
}

/// Iterator over IP addresses in a network
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ipv4NetworkIter {
    current: Ipv4Addr,
    end: Ipv4Addr,
}

impl Iterator for Ipv4NetworkIter {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let result = self.current;
            self.current = self.current + 1;
            Some(result)
        } else {
            None
        }
    }
}

// Implement addition for Ipv4Addr
impl std::ops::Add<u32> for Ipv4Addr {
    type Output = Ipv4Addr;

    fn add(self, rhs: u32) -> Self::Output {
        let result = self.to_int() + rhs;
        Ipv4Addr::from_int(result)
    }
}

impl std::ops::Sub<u32> for Ipv4Addr {
    type Output = Ipv4Addr;

    fn sub(self, rhs: u32) -> Self::Output {
        let result = self.to_int() - rhs;
        Ipv4Addr::from_int(result)
    }
}

impl std::ops::Sub<Ipv4Addr> for Ipv4Addr {
    type Output = u32;

    fn sub(self, rhs: Ipv4Addr) -> Self::Output {
        self.to_int() - rhs.to_int()
    }
}

impl std::ops::BitAnd for Ipv4Addr {
    type Output = Ipv4Addr;

    fn bitand(self, rhs: Self) -> Self::Output {
        let a = self.to_int();
        let b = rhs.to_int();
        Ipv4Addr::from_int(a & b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cidr() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        assert_eq!(network.address(), Ipv4Addr::from_dotted("192.168.1.0").unwrap());
        assert_eq!(network.prefix_len(), 24);
    }

    #[test]
    fn test_netmask() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        assert_eq!(network.netmask(), Ipv4Addr::from_dotted("255.255.255.0").unwrap());
    }

    #[test]
    fn test_broadcast() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        assert_eq!(network.broadcast(), Ipv4Addr::from_dotted("192.168.1.255").unwrap());
    }

    #[test]
    fn test_usable_hosts() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        assert_eq!(network.usable_hosts(), 254);
    }

    #[test]
    fn test_contains() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        let inside = Ipv4Addr::from_dotted("192.168.1.100").unwrap();
        let outside = Ipv4Addr::from_dotted("192.168.2.1").unwrap();
        
        assert!(network.contains(inside));
        assert!(!network.contains(outside));
    }

    #[test]
    fn test_prefix_for_hosts() {
        assert_eq!(Ipv4Network::prefix_for_hosts(254), 24);
        assert_eq!(Ipv4Network::prefix_for_hosts(126), 25);
        assert_eq!(Ipv4Network::prefix_for_hosts(62), 26);
    }
}
