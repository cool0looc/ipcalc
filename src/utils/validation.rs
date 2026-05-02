//! Validation utilities

use crate::core::ipv4::address::Ipv4Addr;
use crate::core::ipv4::network::Ipv4Network;

/// Check if a string is a valid IPv4 address
#[allow(dead_code)]
pub fn is_valid_ipv4(s: &str) -> bool {
    Ipv4Addr::from_dotted(s).is_ok()
}

/// Check if a string is a valid CIDR notation
#[allow(dead_code)]
pub fn is_valid_cidr(s: &str) -> bool {
    Ipv4Network::from_cidr(s).is_ok()
}

/// Check if a string is a valid IP range (START-END)
#[allow(dead_code)]
pub fn is_valid_range(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let start_ok = is_valid_ipv4(parts[0].trim());
    let end_ok = is_valid_ipv4(parts[1].trim());
    
    if !start_ok || !end_ok {
        return false;
    }
    
    let start = Ipv4Addr::from_dotted(parts[0].trim()).unwrap();
    let end = Ipv4Addr::from_dotted(parts[1].trim()).unwrap();
    
    start <= end
}
