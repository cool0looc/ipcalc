//! Lookup command - Lookup well-known network ranges

use crate::cli::Cli;
use crate::core::ipv4::private::KNOWN_NETWORKS;
use crate::utils::constants::KNOWN_NETWORK_INFO;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NetworkLookupResult {
    pub name: String,
    pub description: String,
    pub rfc: String,
    pub networks: Vec<NetworkRange>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NetworkRange {
    pub cidr: String,
    pub range: String,
    pub total_ips: u32,
}

pub fn run(_cli: &Cli, network: Option<String>, list: bool) -> Result<()> {
    if list {
        // List all available networks
        list_all_networks()?;
    } else if let Some(name) = network {
        // Lookup specific network
        lookup_network(&name.to_uppercase())?;
    } else {
        // Show help
        println!("Use --list to see all available networks");
        println!("Or specify a network name: ipcalc lookup RFC1918");
    }
    
    Ok(())
}

fn list_all_networks() -> Result<()> {
    println!("Available networks:");
    println!();
    
    let unique_names = [
        ("RFC1918", "Private Addresses"),
        ("LOOPBACK", "Loopback"),
        ("LINKLOCAL", "Link-Local"),
        ("MULTICAST", "Multicast"),
        ("TEST-NET-1", "Documentation (192.0.2.0/24)"),
        ("TEST-NET-2", "Documentation (198.51.100.0/24)"),
        ("TEST-NET-3", "Documentation (203.0.113.0/24)"),
        ("DOCUMENTATION", "All documentation ranges"),
        ("CGNAT", "Shared Address Space"),
        ("6TO4", "6to4 Relay"),
        ("BENCHMARK", "Benchmark Testing"),
    ];
    
    for (name, desc) in unique_names.iter() {
        println!("  {:<15} - {}", name, desc);
    }
    
    Ok(())
}

fn lookup_network(name: &str) -> Result<()> {
    // Special case for DOCUMENTATION
    if name == "DOCUMENTATION" {
        println!("Documentation Networks:");
        println!();
        
        let networks = vec![
            ("192.0.2.0/24", "192.0.2.0 - 192.0.2.255", 256),
            ("198.51.100.0/24", "198.51.100.0 - 198.51.100.255", 256),
            ("203.0.113.0/24", "203.0.113.0 - 203.0.113.255", 256),
        ];
        
        for (cidr, range, total) in networks {
            println!("  {:<20} ({})  {:>10} IPs", cidr, range, total);
        }
        
        return Ok(());
    }
    
    // Get network info
    if let Some(info) = KNOWN_NETWORK_INFO.get(name) {
        println!("{}:", info.name);
        println!("  {}", info.description);
        println!("  RFC: {}", info.rfc);
        println!();
    }
    
    // Get network ranges
    if let Some(ranges) = KNOWN_NETWORKS.get(name) {
        println!("Ranges:");
        for network in ranges {
            let (start, end) = network.range();
            println!("  {:<20} ({} - {})  {:>10} IPs",
                network.to_string(),
                start,
                end,
                network.total_addrs()
            );
        }
    } else {
        println!("Network '{}' not found. Use --list to see all available networks.", name);
    }
    
    Ok(())
}
