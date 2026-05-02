//! Private IP Detection - RFC1918 and other special ranges
//! 
//! Identifies private, public, and special-purpose IP addresses.

use super::address::Ipv4Addr;
use super::network::Ipv4Network;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// IP address type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpType {
    /// Private IP address (RFC1918)
    Private,
    /// Public IP address
    Public,
    /// Loopback (127.0.0.0/8)
    Loopback,
    /// Link-local (169.254.0.0/16)
    LinkLocal,
    /// Multicast (224.0.0.0/4)
    Multicast,
    /// Broadcast (255.255.255.255)
    Broadcast,
    /// Unspecified (0.0.0.0)
    Unspecified,
    /// Reserved for documentation (192.0.2.0/24, etc.)
    Documentation,
    /// Carrier-grade NAT (100.64.0.0/10)
    Cgnat,
}

impl IpType {
    /// Get RFC reference
    pub fn rfc(&self) -> &'static str {
        match self {
            IpType::Private => "RFC1918",
            IpType::Public => "",  // Public IPs don't have a specific RFC
            IpType::Loopback => "RFC1122",
            IpType::LinkLocal => "RFC3927",
            IpType::Multicast => "RFC5771",
            IpType::Broadcast => "RFC919",
            IpType::Unspecified => "RFC1122",
            IpType::Documentation => "RFC5737",
            IpType::Cgnat => "RFC6598",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            IpType::Private => "Private IP address",
            IpType::Public => "Public IP address",
            IpType::Loopback => "Loopback address",
            IpType::LinkLocal => "Link-local address",
            IpType::Multicast => "Multicast address",
            IpType::Broadcast => "Broadcast address",
            IpType::Unspecified => "Unspecified address",
            IpType::Documentation => "Documentation range (TEST-NET)",
            IpType::Cgnat => "Carrier-grade NAT (shared space)",
        }
    }
}

impl std::fmt::Display for IpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description(), self.rfc())
    }
}

// Well-known network ranges defined below (lazy_static macro)
lazy_static! {
    pub static ref KNOWN_NETWORKS: HashMap<&'static str, Vec<Ipv4Network>> = {
        let mut m = HashMap::new();
        
        // RFC1918 - Private
        m.insert("RFC1918", vec![
            Ipv4Network::from_cidr("10.0.0.0/8").unwrap(),
            Ipv4Network::from_cidr("172.16.0.0/12").unwrap(),
            Ipv4Network::from_cidr("192.168.0.0/16").unwrap(),
        ]);
        
        // Loopback
        m.insert("LOOPBACK", vec![
            Ipv4Network::from_cidr("127.0.0.0/8").unwrap(),
        ]);
        
        // Link-local
        m.insert("LINKLOCAL", vec![
            Ipv4Network::from_cidr("169.254.0.0/16").unwrap(),
        ]);
        
        // Multicast
        m.insert("MULTICAST", vec![
            Ipv4Network::from_cidr("224.0.0.0/4").unwrap(),
        ]);
        
        // Documentation ranges
        m.insert("TEST-NET-1", vec![
            Ipv4Network::from_cidr("192.0.2.0/24").unwrap(),
        ]);
        m.insert("TEST-NET-2", vec![
            Ipv4Network::from_cidr("198.51.100.0/24").unwrap(),
        ]);
        m.insert("TEST-NET-3", vec![
            Ipv4Network::from_cidr("203.0.113.0/24").unwrap(),
        ]);
        m.insert("DOCUMENTATION", vec![
            Ipv4Network::from_cidr("192.0.2.0/24").unwrap(),
            Ipv4Network::from_cidr("198.51.100.0/24").unwrap(),
            Ipv4Network::from_cidr("203.0.113.0/24").unwrap(),
        ]);
        
        // CGNAT
        m.insert("CGNAT", vec![
            Ipv4Network::from_cidr("100.64.0.0/10").unwrap(),
        ]);
        
        // 6to4 relay
        m.insert("6TO4", vec![
            Ipv4Network::from_cidr("192.88.99.0/24").unwrap(),
        ]);
        
        // Benchmark testing
        m.insert("BENCHMARK", vec![
            Ipv4Network::from_cidr("198.18.0.0/15").unwrap(),
        ]);
        
        m
    };
}

/// Detect the type of an IP address
pub fn detect_type(ip: Ipv4Addr) -> IpType {
    // Check special addresses first
    if ip.is_unspecified() {
        return IpType::Unspecified;
    }
    if ip.is_broadcast() {
        return IpType::Broadcast;
    }
    if ip.is_loopback() {
        return IpType::Loopback;
    }

    // Check against known networks
    for (name, networks) in KNOWN_NETWORKS.iter() {
        for network in networks {
            if network.contains(ip) {
                return match *name {
                    "RFC1918" => IpType::Private,
                    "LINKLOCAL" => IpType::LinkLocal,
                    "MULTICAST" => IpType::Multicast,
                    "DOCUMENTATION" => IpType::Documentation,
                    "CGNAT" => IpType::Cgnat,
                    "LOOPBACK" => IpType::Loopback,
                    _ => IpType::Public,
                };
            }
        }
    }

    IpType::Public
}

/// Check if an IP is private
pub fn is_private(ip: Ipv4Addr) -> bool {
    detect_type(ip) == IpType::Private
}

/// Check if an IP is public
#[allow(dead_code)]
pub fn is_public(ip: Ipv4Addr) -> bool {
    detect_type(ip) == IpType::Public
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private() {
        assert!(is_private(Ipv4Addr::from_dotted("10.0.0.1").unwrap()));
        assert!(is_private(Ipv4Addr::from_dotted("172.16.0.1").unwrap()));
        assert!(is_private(Ipv4Addr::from_dotted("192.168.1.1").unwrap()));
    }

    #[test]
    fn test_public() {
        assert!(is_public(Ipv4Addr::from_dotted("8.8.8.8").unwrap()));
        assert!(is_public(Ipv4Addr::from_dotted("1.1.1.1").unwrap()));
    }

    #[test]
    fn test_loopback() {
        assert!(!is_public(Ipv4Addr::from_dotted("127.0.0.1").unwrap()));
    }

    #[test]
    fn test_detect_type() {
        assert_eq!(detect_type(Ipv4Addr::from_dotted("10.0.0.1").unwrap()), IpType::Private);
        assert_eq!(detect_type(Ipv4Addr::from_dotted("8.8.8.8").unwrap()), IpType::Public);
        assert_eq!(detect_type(Ipv4Addr::from_dotted("127.0.0.1").unwrap()), IpType::Loopback);
        assert_eq!(detect_type(Ipv4Addr::from_dotted("169.254.0.1").unwrap()), IpType::LinkLocal);
    }
}
