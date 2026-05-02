//! CIDR aggregation
//! 
//! Aggregates multiple CIDR blocks into supernets.

use crate::core::ipv4::network::Ipv4Network;

/// Aggregate a list of IPv4 networks into supernets
#[allow(dead_code)]
pub fn aggregate_ipv4_networks(networks: Vec<Ipv4Network>) -> Vec<Ipv4Network> {
    if networks.is_empty() {
        return Vec::new();
    }
    
    // Sort by address
    let mut sorted = networks;
    sorted.sort_by_key(|n| n.address());
    
    let mut result = Vec::new();
    let mut current = sorted[0];
    
    for network in sorted.iter().skip(1) {
        // Try to merge
        if let Some(merged) = try_merge_ipv4(current, *network) {
            current = merged;
        } else {
            result.push(current);
            current = *network;
        }
    }
    
    result.push(current);
    result
}

/// Try to merge two IPv4 networks
#[allow(dead_code)]
fn try_merge_ipv4(a: Ipv4Network, b: Ipv4Network) -> Option<Ipv4Network> {
    // Basic merge logic
    if a.prefix_len() == b.prefix_len() {
        // Check if they are adjacent
        if a.broadcast().to_int() + 1 == b.address().to_int() {
            if a.prefix_len() > 0 {
                if let Ok(merged) = Ipv4Network::new(a.address(), a.prefix_len() - 1) {
                    return Some(merged);
                }
            }
        }
    }
    
    // Try to find common parent
    if a.prefix_len() > 0 {
        if let Ok(parent) = Ipv4Network::new(a.address(), a.prefix_len() - 1) {
            if parent.contains(b.address()) {
                return Some(parent);
            }
        }
    }
    
    None
}

/// Helper trait for network operations
#[allow(dead_code)]
pub trait NetworkOps {
    fn address_int(&self) -> u32;
    fn prefix_len(&self) -> u8;
}

#[allow(dead_code)]
impl NetworkOps for Ipv4Network {
    fn address_int(&self) -> u32 {
        self.address().to_int()
    }
    
    fn prefix_len(&self) -> u8 {
        Ipv4Network::prefix_len(self)
    }
}
