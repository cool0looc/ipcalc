//! Subnet command - Calculate subnet information

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetInfo {
    pub network: String,
    pub address: String,
    pub netmask: String,
    pub wildcard: String,
    pub broadcast: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_host: Option<String>,
    pub total_hosts: u32,
}

pub fn run(cli: &Cli, cidrs: Vec<String>, show_all: bool) -> Result<()> {
    let mut results = Vec::new();

    for cidr in cidrs {
        match Ipv4Network::from_cidr(&cidr) {
            Ok(network) => {
                let info = calculate_subnet_info(&network, show_all);
                results.push(info);
            }
            Err(e) => {
                if cli.is_strict() {
                    return Err(anyhow::anyhow!("Invalid CIDR '{}': {}", cidr, e));
                }
                // In non-strict mode, just skip invalid CIDRs
                continue;
            }
        }
    }

    // Output results
    match cli.format {
        crate::cli::OutputFormat::Human => {
            for info in &results {
                print_subnet_info_human(info, show_all);
            }
        }
        _ => {
            cli.outputter().output(&results)?;
        }
    }

    Ok(())
}

fn calculate_subnet_info(network: &Ipv4Network, show_all: bool) -> SubnetInfo {
    let info = SubnetInfo {
        network: network.to_string(),
        address: network.address().to_string(),
        netmask: network.netmask().to_string(),
        wildcard: network.wildcard().to_string(),
        broadcast: network.broadcast().to_string(),
        first_host: if show_all { network.first_host().map(|h| h.to_string()) } else { None },
        last_host: if show_all { network.last_host().map(|h| h.to_string()) } else { None },
        total_hosts: network.usable_hosts(),
    };
    info
}

fn print_subnet_info_human(info: &SubnetInfo, show_all: bool) {
    println!("Network:      {}", info.network);
    println!("Address:      {}", info.address);
    println!("Netmask:      {}", info.netmask);
    println!("Wildcard:     {}", info.wildcard);
    println!("Broadcast:   {}", info.broadcast);
    
    if show_all {
        if let Some(first) = &info.first_host {
            println!("First Host:  {}", first);
        }
        if let Some(last) = &info.last_host {
            println!("Last Host:   {}", last);
        }
    }
    
    println!("Total Hosts: {}", info.total_hosts);
    println!();
}
