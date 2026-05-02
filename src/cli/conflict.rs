//! Conflict command - Detect IP address conflicts

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictResult {
    pub networks: Vec<NetworkInfo>,
    pub conflicts: Vec<ConflictInfo>,
    pub total_networks: usize,
    pub has_conflicts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub cidr: String,
    pub network: String,
    pub broadcast: String,
    pub size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub networks: Vec<String>,
    pub overlap_type: String,
    pub overlap_range: String,
}

pub fn run(cli: &Cli, cidrs: Vec<String>) -> Result<()> {
    let mut networks = Vec::new();
    let mut network_ranges: Vec<(Ipv4Network, NetworkInfo)> = Vec::new();
    
    for cidr in &cidrs {
        let network = Ipv4Network::from_cidr(cidr).map_err(|e| anyhow::anyhow!(e))?;
        let info = NetworkInfo {
            cidr: cidr.clone(),
            network: network.address().to_string(),
            broadcast: network.broadcast().to_string(),
            size: network.total_addrs(),
        };
        network_ranges.push((network, info.clone()));
        networks.push(info);
    }
    
    // Detect conflicts
    let conflicts = detect_conflicts(&network_ranges);
    
    let result = ConflictResult {
        networks: networks.clone(),
        conflicts: conflicts.clone(),
        total_networks: networks.len(),
        has_conflicts: !conflicts.is_empty(),
    };
    
    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("\nChecking {} networks for conflicts...\n", networks.len());
            
            if conflicts.is_empty() {
                println!("✓ No conflicts detected!");
                println!("\nNetworks:");
                for network in &networks {
                    println!("  {}  ({} - {}) [{} IPs]", 
                        network.cidr, network.network, network.broadcast, network.size);
                }
            } else {
                println!("✗ Found {} conflict(s)!", conflicts.len());
                println!("\nConflicts:");
                for (i, conflict) in conflicts.iter().enumerate() {
                    println!("\n  Conflict #{}: {}", i + 1, conflict.overlap_type);
                    println!("  Networks: {}", conflict.networks.join(", "));
                    println!("  Overlap: {}", conflict.overlap_range);
                }
                
                println!("\nAll Networks:");
                for network in &networks {
                    println!("  {}  ({} - {}) [{} IPs]", 
                        network.cidr, network.network, network.broadcast, network.size);
                }
            }
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }
    
    Ok(())
}

fn detect_conflicts(networks: &[(Ipv4Network, NetworkInfo)]) -> Vec<ConflictInfo> {
    let mut conflicts = Vec::new();
    
    for i in 0..networks.len() {
        for j in (i + 1)..networks.len() {
            if let Some(conflict) = check_overlap(&networks[i].0, &networks[j].0, 
                                                   &networks[i].1, &networks[j].1) {
                conflicts.push(conflict);
            }
        }
    }
    
    conflicts
}

fn check_overlap(a: &Ipv4Network, b: &Ipv4Network, 
                 a_info: &NetworkInfo, b_info: &NetworkInfo) -> Option<ConflictInfo> {
    let a_start = a.address().to_int();
    let a_end = a.broadcast().to_int();
    let b_start = b.address().to_int();
    let b_end = b.broadcast().to_int();
    
    // Check if ranges overlap
    if a_start <= b_end && b_start <= a_end {
        let overlap_start = std::cmp::max(a_start, b_start);
        let overlap_end = std::cmp::min(a_end, b_end);
        
        let overlap_type = if a_start == b_start && a_end == b_end {
            "Exact Match"
        } else if a.contains(b.address()) || a.contains(b.broadcast()) {
            "A contains B"
        } else if b.contains(a.address()) || b.contains(a.broadcast()) {
            "B contains A"
        } else {
            "Partial Overlap"
        };
        
        let overlap_range = format!(
            "{} - {}",
            crate::core::ipv4::address::Ipv4Addr::from_int(overlap_start),
            crate::core::ipv4::address::Ipv4Addr::from_int(overlap_end)
        );
        
        Some(ConflictInfo {
            networks: vec![a_info.cidr.clone(), b_info.cidr.clone()],
            overlap_type: overlap_type.to_string(),
            overlap_range,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_conflict() {
        let result = detect_conflicts(&vec![
            (Ipv4Network::from_cidr("192.168.1.0/24").unwrap(), 
             NetworkInfo { cidr: "192.168.1.0/24".to_string(), network: "192.168.1.0".to_string(), broadcast: "192.168.1.255".to_string(), size: 256 }),
            (Ipv4Network::from_cidr("192.168.2.0/24").unwrap(), 
             NetworkInfo { cidr: "192.168.2.0/24".to_string(), network: "192.168.2.0".to_string(), broadcast: "192.168.2.255".to_string(), size: 256 }),
        ]);
        
        assert!(result.is_empty());
    }

    #[test]
    fn test_conflict() {
        let result = detect_conflicts(&vec![
            (Ipv4Network::from_cidr("192.168.1.0/24").unwrap(), 
             NetworkInfo { cidr: "192.168.1.0/24".to_string(), network: "192.168.1.0".to_string(), broadcast: "192.168.1.255".to_string(), size: 256 }),
            (Ipv4Network::from_cidr("192.168.1.128/25").unwrap(), 
             NetworkInfo { cidr: "192.168.1.128/25".to_string(), network: "192.168.1.128".to_string(), broadcast: "192.168.1.255".to_string(), size: 128 }),
        ]);
        
        assert!(!result.is_empty());
    }
}
