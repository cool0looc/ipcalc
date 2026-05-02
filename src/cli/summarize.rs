//! Summarize command - Aggregate multiple CIDRs into supernets

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizeResult {
    pub input_cidrs: Vec<String>,
    pub aggregated: Vec<String>,
}

pub fn run(cli: &Cli, cidrs: Vec<String>) -> Result<()> {
    let mut networks: Vec<Ipv4Network> = Vec::new();
    
    for cidr in &cidrs {
        match Ipv4Network::from_cidr(cidr) {
            Ok(network) => networks.push(network),
            Err(e) => {
                if cli.is_strict() {
                    return Err(anyhow::anyhow!("Invalid CIDR '{}': {}", cidr, e));
                }
            }
        }
    }
    
    // Sort networks by address
    networks.sort_by_key(|n| n.address());
    
    // Aggregate networks
    let aggregated = aggregate_networks(networks);
    
    let result = SummarizeResult {
        input_cidrs: cidrs,
        aggregated: aggregated.iter().map(|n| n.to_string()).collect(),
    };

    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("Input CIDRs:");
            for cidr in &result.input_cidrs {
                println!("  - {}", cidr);
            }
            println!();
            println!("Aggregated:");
            for cidr in &result.aggregated {
                println!("  - {}", cidr);
            }
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }

    Ok(())
}

/// Aggregate a list of networks into supernets
fn aggregate_networks(networks: Vec<Ipv4Network>) -> Vec<Ipv4Network> {
    if networks.is_empty() {
        return Vec::new();
    }
    
    let mut result = Vec::new();
    let mut current = networks[0];
    
    for network in networks.iter().skip(1) {
        // Try to merge current and network
        if let Some(merged) = try_merge(current, *network) {
            current = merged;
        } else {
            result.push(current);
            current = *network;
        }
    }
    
    result.push(current);
    result
}

/// Try to merge two networks into a supernet
fn try_merge(a: Ipv4Network, b: Ipv4Network) -> Option<Ipv4Network> {
    // Networks must have the same prefix length and be adjacent
    if a.prefix_len() != b.prefix_len() {
        // Try with one less prefix
        if a.prefix_len() > 0 {
            let parent_prefix = a.prefix_len() - 1;
            if let Ok(parent) = Ipv4Network::new(a.address(), parent_prefix) {
                if parent.contains(b.address()) && parent.contains(b.broadcast()) {
                    return Some(parent);
                }
            }
        }
        return None;
    }
    
    // Check if they are adjacent
    let a_end = a.broadcast();
    let b_start = b.address();
    
    if a_end + 1 == b_start {
        // Can merge if prefix > 0
        if a.prefix_len() > 0 {
            let new_prefix = a.prefix_len() - 1;
            if let Ok(merged) = Ipv4Network::new(a.address(), new_prefix) {
                return Some(merged);
            }
        }
    }
    
    None
}
