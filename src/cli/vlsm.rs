//! VLSM command - Variable Length Subnet Mask allocation

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlsmAllocation {
    pub network: String,
    pub size: u32,
    pub hosts: u32,
    pub range: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VlsmResult {
    pub network: String,
    pub available_hosts: u32,
    pub allocations: Vec<VlsmAllocation>,
    pub total_allocated: u32,
    pub total_available: u32,
    pub remaining: u32,
}

pub fn run(cli: &Cli, cidr: String, requirements: Vec<String>) -> Result<()> {
    let network = Ipv4Network::from_cidr(&cidr).map_err(|e| anyhow::anyhow!(e))?;
    let available = network.usable_hosts();
    
    // Parse requirements - supports both comma-separated and space-separated
    let mut parsed_reqs: Vec<u32> = Vec::new();
    for req in requirements {
        // Split by comma and/or space
        for part in req.split(|c| c == ',' || c == ' ') {
            let part = part.trim();
            if !part.is_empty() {
                let num: u32 = part.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number: '{}'", part))?;
                parsed_reqs.push(num);
            }
        }
    }
    
    if parsed_reqs.is_empty() {
        anyhow::bail!("No host requirements provided");
    }
    
    // Sort requirements in descending order (largest first)
    parsed_reqs.sort_by(|a, b| b.cmp(a));
    
    let mut allocations = Vec::new();
    let mut current_addr = network.address();
    let mut total_allocated = 0u32;
    
    for (_i, hosts_needed) in parsed_reqs.iter().enumerate() {
        // Calculate required prefix for this many hosts
        let prefix = Ipv4Network::prefix_for_hosts(*hosts_needed);
        let subnet = Ipv4Network::new(current_addr, prefix).unwrap();
        
        if !network.contains(subnet.broadcast()) {
            anyhow::bail!("Insufficient address space for {} hosts (need {} more addresses)",
                hosts_needed, hosts_needed + 2);
        }
        
        let allocation = VlsmAllocation {
            network: subnet.to_string(),
            size: subnet.total_addrs(),
            hosts: subnet.usable_hosts(),
            range: format!("{} - {}", subnet.address(), subnet.broadcast()),
        };
        
        allocations.push(allocation);
        total_allocated += subnet.usable_hosts();
        current_addr = subnet.broadcast() + 1;
    }
    
    let result = VlsmResult {
        network: cidr.clone(),
        available_hosts: available,
        allocations: allocations.clone(),
        total_allocated,
        total_available: available,
        remaining: available - total_allocated,
    };

    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("Network:      {}", cidr);
            println!("Available:    {} hosts", available);
            println!();
            println!("Allocation Table:");
            println!("  {:<3} {:<18} {:<6} {:<6} {}", 
                "#", "Network", "Size", "Hosts", "Range");
            for (i, alloc) in allocations.iter().enumerate() {
                println!("  {:<3} {:<18} {:<6} {:<6} {}", 
                    i + 1, alloc.network, alloc.size, alloc.hosts, alloc.range);
            }
            println!();
            println!("Summary:");
            println!("  Total Allocated: {} hosts", total_allocated);
            println!("  Total Available: {} hosts", available);
            println!("  Remaining: {} hosts", result.remaining);
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }

    Ok(())
}
