//! IPv4 Classification - IP address class detection
//! 
//! Determines the class (A/B/C/D/E) of an IPv4 address.

use super::address::Ipv4Addr;
use serde::{Deserialize, Serialize};

/// IP address class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpClass {
    /// Class A (0.0.0.0 - 127.255.255.255)
    A,
    /// Class B (128.0.0.0 - 191.255.255.255)
    B,
    /// Class C (192.0.0.0 - 223.255.255.255)
    C,
    /// Class D - Multicast (224.0.0.0 - 239.255.255.255)
    D,
    /// Class E - Reserved (240.0.0.0 - 255.255.255.255)
    E,
}

impl IpClass {
    /// Get the default mask for this class
    pub fn default_mask(&self) -> u8 {
        match self {
            IpClass::A => 8,
            IpClass::B => 16,
            IpClass::C => 24,
            IpClass::D | IpClass::E => 32,
        }
    }

    /// Get the range string for this class
    pub fn range(&self) -> &'static str {
        match self {
            IpClass::A => "0.0.0.0 - 127.255.255.255",
            IpClass::B => "128.0.0.0 - 191.255.255.255",
            IpClass::C => "192.0.0.0 - 223.255.255.255",
            IpClass::D => "224.0.0.0 - 239.255.255.255",
            IpClass::E => "240.0.0.0 - 255.255.255.255",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            IpClass::A => "Large networks (up to 16M hosts)",
            IpClass::B => "Medium networks (up to 65K hosts)",
            IpClass::C => "Small networks (up to 254 hosts)",
            IpClass::D => "Multicast",
            IpClass::E => "Reserved for future use / research",
        }
    }

    /// Classify an IP address
    pub fn classify(ip: Ipv4Addr) -> Self {
        let first_octet = ip.first_octet();
        
        match first_octet {
            0..=127 => IpClass::A,
            128..=191 => IpClass::B,
            192..=223 => IpClass::C,
            224..=239 => IpClass::D,
            240..=255 => IpClass::E,
        }
    }

    /// Get the network portion for this class
    #[allow(dead_code)]
    pub fn network_bits(&self) -> u8 {
        match self {
            IpClass::A => 8,
            IpClass::B => 16,
            IpClass::C => 24,
            IpClass::D | IpClass::E => 0, // No network portion
        }
    }

    /// Get the host portion for this class
    #[allow(dead_code)]
    pub fn host_bits(&self) -> u8 {
        match self {
            IpClass::A => 24,
            IpClass::B => 16,
            IpClass::C => 8,
            IpClass::D | IpClass::E => 0, // No host portion
        }
    }
}

impl std::fmt::Display for IpClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IpClass::A => "A",
            IpClass::B => "B",
            IpClass::C => "C",
            IpClass::D => "D",
            IpClass::E => "E",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify() {
        assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("10.0.0.1").unwrap()), IpClass::A);
        assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("172.16.0.1").unwrap()), IpClass::B);
        assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("192.168.1.1").unwrap()), IpClass::C);
        assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("224.0.0.1").unwrap()), IpClass::D);
        assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("240.0.0.1").unwrap()), IpClass::E);
    }

    #[test]
    fn test_default_mask() {
        assert_eq!(IpClass::A.default_mask(), 8);
        assert_eq!(IpClass::B.default_mask(), 16);
        assert_eq!(IpClass::C.default_mask(), 24);
    }
}
