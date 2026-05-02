//! IP range to CIDR conversion
//! 
//! Converts IP ranges to minimal CIDR blocks.

use crate::core::ipv4::address::Ipv4Addr;
use crate::core::ipv4::network::Ipv4Network;
use crate::core::IpError;

/// Convert an IP range to a list of CIDR blocks
#[allow(dead_code)]
pub fn range_to_cidrs(start: Ipv4Addr, end: Ipv4Addr) -> Result<Vec<Ipv4Network>, IpError> {
    let mut cidrs = Vec::new();
    let mut current = start;
    
    while current <= end {
        // Find the largest prefix that fits
        let prefix = find_largest_prefix(current, end);
        let network = Ipv4Network::new(current, prefix)?;
        
        cidrs.push(network);
        
        // Move to the next network
        let total = network.total_addrs();
        current = current + total;
    }
    
    Ok(cidrs)
}

/// Find the largest prefix length for which the network fits in the range
#[allow(dead_code)]
fn find_largest_prefix(current: Ipv4Addr, end: Ipv4Addr) -> u8 {
    let range_size = (end - current) as u32 + 1;
    let _bits_needed = 32 - range_size.ilog2() as u8;
    
    // Start from /32 and work down
    for prefix in (0..=32).rev() {
        if let Ok(network) = Ipv4Network::new(current, prefix) {
            let network_end = network.broadcast();
            if network_end <= end {
                return prefix;
            }
        }
    }
    
    32 // Default to /32
}

/// Convert a list of CIDR blocks to an IP range
#[allow(dead_code)]
pub fn cidrs_to_range(networks: &[Ipv4Network]) -> Option<(Ipv4Addr, Ipv4Addr)> {
    if networks.is_empty() {
        return None;
    }
    
    let mut sorted = networks.to_vec();
    sorted.sort_by_key(|n| n.address());
    
    let first = sorted.first()?.address();
    let last = sorted.last()?.broadcast();
    
    Some((first, last))
}
