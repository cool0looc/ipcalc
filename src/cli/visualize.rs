//! Visualize command - ASCII tree visualization of network hierarchy

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VisualizeResult {
    pub network: String,
    pub tree: String,
}

pub fn run(cli: &Cli, cidr: String, depth: Option<u8>) -> Result<()> {
    let network = Ipv4Network::from_cidr(&cidr).map_err(|e| anyhow::anyhow!(e))?;
    let max_depth = depth.unwrap_or(3);
    
    let tree = build_tree(&network, 0, max_depth);
    
    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("\nNetwork: {}", cidr);
            println!("\n{}", tree);
        }
        _ => {
            let result = VisualizeResult {
                network: cidr,
                tree: tree.clone(),
            };
            cli.outputter().output(&result)?;
        }
    }
    
    Ok(())
}

fn build_tree(network: &Ipv4Network, current_depth: u8, max_depth: u8) -> String {
    if current_depth > max_depth {
        return String::new();
    }
    
    let prefix = network.prefix_len();
    let mut output = String::new();
    
    // Calculate subnet info
    let total_hosts = network.total_addrs();
    let usable_hosts = if total_hosts > 2 { total_hosts - 2 } else { total_hosts };
    
    // Build tree lines
    let indent = "  ".repeat(current_depth as usize);
    let branch = if current_depth == 0 { "┌─" } else { "├─" };
    
    output.push_str(&format!(
        "{}{}[{}/{}] {} ({} hosts, {} usable)\n",
        indent,
        branch,
        network.address(),
        prefix,
        if current_depth == 0 { "ROOT" } else { "" },
        total_hosts,
        usable_hosts
    ));
    
    // Add children if we haven't reached max depth
    if current_depth < max_depth && prefix < 30 {
        let child_prefix = std::cmp::min(prefix + 1, 30);
        
        if let Ok(child1) = Ipv4Network::new(network.address(), child_prefix) {
            let broadcast = network.broadcast();
            let mid_addr = if prefix < 31 {
                let block_size = 1u32 << (32 - child_prefix);
                crate::core::ipv4::address::Ipv4Addr::from_int(network.address().to_int() + block_size / 2)
            } else {
                network.address()
            };
            
            if let Ok(child2) = Ipv4Network::new(mid_addr, child_prefix) {
                // Only show children that fit within parent
                if child1.broadcast() <= broadcast {
                    output.push_str(&build_tree(&child1, current_depth + 1, max_depth));
                }
                if child2.broadcast() <= broadcast && child2.address() > child1.address() {
                    output.push_str(&build_tree(&child2, current_depth + 1, max_depth));
                }
            }
        }
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_generation() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        let tree = build_tree(&network, 0, 2);
        
        assert!(!tree.is_empty());
        assert!(tree.contains("192.168.1.0/24"));
    }
}
