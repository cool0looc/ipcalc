//! Divide command - Divide a network into subnets

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetDivision {
    pub subnet: String,
    pub range: String,
    pub total_ips: u32,
    pub usable_hosts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DivideResult {
    pub network: String,
    pub num_subnets: u32,
    pub new_prefix: u8,
    pub subnets: Vec<SubnetDivision>,
}

pub fn run(cli: &Cli, cidr: String, num_subnets: u32) -> Result<()> {
    let network = Ipv4Network::from_cidr(&cidr).map_err(|e| anyhow::anyhow!(e))?;
    
    // Calculate new prefix length
    let additional_bits = (num_subnets as f64).log2().ceil() as u8;
    let new_prefix = network.prefix_len() + additional_bits;
    
    if new_prefix > 32 {
        anyhow::bail!("Cannot divide {} into {} subnets: insufficient address space", 
            cidr, num_subnets);
    }
    
    // Generate subnets
    let subnets = generate_subnets(network, new_prefix, num_subnets);
    
    let result = DivideResult {
        network: cidr,
        num_subnets,
        new_prefix,
        subnets: subnets.clone(),
    };

    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("Network:     {}", result.network);
            println!("Subnets:    {}", result.num_subnets);
            println!("New Prefix: /{}", result.new_prefix);
            println!();
            println!("Subnets:");
            for (i, subnet) in subnets.iter().enumerate() {
                println!("  {}. {:<18} ({})    [{} IPs, {} hosts]",
                    i + 1, subnet.subnet, subnet.range, subnet.total_ips, subnet.usable_hosts);
            }
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }

    Ok(())
}

fn generate_subnets(parent: Ipv4Network, new_prefix: u8, count: u32) -> Vec<SubnetDivision> {
    let mut subnets = Vec::new();
    let mut current = parent.address();
    let step = 2u32.pow((32 - new_prefix) as u32);
    
    for _i in 0..count {
        let network = Ipv4Network::new(current, new_prefix).unwrap();
        let subnet = SubnetDivision {
            subnet: network.to_string(),
            range: format!("{} - {}", network.address(), network.broadcast()),
            total_ips: network.total_addrs(),
            usable_hosts: network.usable_hosts(),
        };
        subnets.push(subnet);
        
        current = current + step;
    }
    
    subnets
}
