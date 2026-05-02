//! Classify command - Classify IP address (A/B/C/D/E)

use crate::cli::Cli;
use crate::core::ipv4::address::Ipv4Addr;
use crate::core::ipv4::classify::IpClass;
use crate::core::ipv4::private::is_private;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifyResult {
    pub address: String,
    pub class: String,
    pub default_mask: u8,
    pub range: String,
    pub description: String,
    pub ip_type: String,
}

pub fn run(cli: &Cli, addresses: Vec<String>) -> Result<()> {
    let mut results = Vec::new();

    for addr_str in addresses {
        match Ipv4Addr::from_dotted(&addr_str) {
            Ok(addr) => {
                let class = IpClass::classify(addr);
                
                let result = ClassifyResult {
                    address: addr.to_string(),
                    class: class.to_string(),
                    default_mask: class.default_mask(),
                    range: class.range().to_string(),
                    description: class.description().to_string(),
                    ip_type: if is_private(addr) { "Private".to_string() } else { "Public".to_string() },
                };
                results.push(result);
            }
            Err(e) => {
                if cli.is_strict() {
                    return Err(anyhow::anyhow!("Invalid IP '{}': {}", addr_str, e));
                }
            }
        }
    }

    match cli.format {
        crate::cli::OutputFormat::Human => {
            for result in &results {
                println!("Address:     {}", result.address);
                println!("Class:       {}", result.class);
                println!("Default Mask: /{}", result.default_mask);
                println!("Range:       {}", result.range);
                println!("Type:        {}", result.ip_type);
                println!();
            }
        }
        _ => {
            cli.outputter().output(&results)?;
        }
    }

    Ok(())
}
