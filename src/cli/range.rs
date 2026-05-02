//! Range command - Show IP range for CIDR

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeInfo {
    pub cidr: String,
    pub start: String,
    pub end: String,
    pub total_ips: u32,
}

pub fn run(cli: &Cli, cidrs: Vec<String>) -> Result<()> {
    let mut results = Vec::new();

    for cidr in cidrs {
        match Ipv4Network::from_cidr(&cidr) {
            Ok(network) => {
                let (start, end) = network.range();
                let info = RangeInfo {
                    cidr: network.to_string(),
                    start: start.to_string(),
                    end: end.to_string(),
                    total_ips: network.total_addrs(),
                };
                results.push(info);
            }
            Err(e) => {
                if cli.is_strict() {
                    return Err(anyhow::anyhow!("Invalid CIDR '{}': {}", cidr, e));
                }
            }
        }
    }

    match cli.format {
        crate::cli::OutputFormat::Human => {
            for info in &results {
                println!("CIDR:        {}", info.cidr);
                println!("Start:       {}", info.start);
                println!("End:         {}", info.end);
                println!("Total IPs:  {}", info.total_ips);
                println!();
            }
        }
        _ => {
            cli.outputter().output(&results)?;
        }
    }

    Ok(())
}
