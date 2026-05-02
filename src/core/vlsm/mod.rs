//! VLSM module - Variable Length Subnet Mask allocation
//! 
//! Calculates optimal subnet allocation based on host requirements.

use crate::core::ipv4::network::Ipv4Network;
use crate::core::IpError;
use serde::{Deserialize, Serialize};

/// A VLSM allocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct VlsmAllocation {
    /// Required number of hosts
    pub hosts: u32,
    /// Allocated network
    pub network: Ipv4Network,
    /// Size of the allocation (including network/broadcast)
    pub size: u32,
    /// Usable hosts
    pub usable: u32,
}

#[allow(dead_code)]
impl VlsmAllocation {
    /// Create a new allocation
    pub fn new(hosts: u32, network: Ipv4Network) -> Self {
        Self {
            hosts,
            network,
            size: network.total_addrs(),
            usable: network.usable_hosts(),
        }
    }
}

/// Calculate VLSM allocations for given host requirements
#[allow(dead_code)]
pub fn calculate_vlsm(
    parent_network: Ipv4Network,
    host_requirements: &[u32],
) -> Result<Vec<VlsmAllocation>, IpError> {
    // Sort requirements in descending order
    let mut sorted = host_requirements.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    
    let mut allocations = Vec::new();
    let mut current_addr = parent_network.address();
    
    for &hosts_needed in &sorted {
        // Calculate required prefix
        let prefix = Ipv4Network::prefix_for_hosts(hosts_needed);
        let network = Ipv4Network::new(current_addr, prefix)?;
        
        // Check if network fits in parent
        if !parent_network.contains(network.broadcast()) {
            return Err(IpError::InsufficientSpace(prefix, 32 - parent_network.prefix_len() as u8));
        }
        
        allocations.push(VlsmAllocation::new(hosts_needed, network));
        
        // Move to next address
        current_addr = network.broadcast() + 1;
    }
    
    Ok(allocations)
}

/// Check if VLSM allocation is possible
#[allow(dead_code)]
pub fn can_allocate(parent_network: Ipv4Network, host_requirements: &[u32]) -> bool {
    calculate_vlsm(parent_network, host_requirements).is_ok()
}

/// Calculate wasted addresses
#[allow(dead_code)]
pub fn calculate_waste(allocations: &[VlsmAllocation], parent_network: Ipv4Network) -> u32 {
    let total_used: u32 = allocations.iter().map(|a| a.size).sum();
    let parent_size = parent_network.total_addrs();
    
    parent_size - total_used
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlsm_calculation() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        let requirements = [50, 25, 10, 5];
        
        let allocations = calculate_vlsm(network, &requirements).unwrap();
        
        assert_eq!(allocations.len(), 4);
        // First allocation should be for 50 hosts (/26)
        assert!(allocations[0].network.prefix_len() <= 26);
    }
}
