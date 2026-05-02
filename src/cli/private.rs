//! Private command - Check if IP is private or public

use crate::cli::Cli;
use crate::core::ipv4::address::Ipv4Addr;
use crate::core::ipv4::private::{detect_type, is_private};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateResult {
    pub address: String,
    pub ip_type: String,
    pub rfc: String,
    pub description: String,
    pub is_private: bool,
}

pub fn run(cli: &Cli, addresses: Vec<String>) -> Result<()> {
    let mut results = Vec::new();

    for addr_str in addresses {
        match Ipv4Addr::from_dotted(&addr_str) {
            Ok(addr) => {
                let ip_type = detect_type(addr);
                
                let result = PrivateResult {
                    address: addr.to_string(),
                    ip_type: ip_type.description().to_string(),
                    rfc: ip_type.rfc().to_string(),
                    description: ip_type.description().to_string(),
                    is_private: is_private(addr),
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
                println!("Type:        {}", result.ip_type);
                println!("Reference:   {}", result.rfc);
                if result.is_private {
                    println!("Status:      Private");
                } else {
                    println!("Status:      Public");
                }
                println!();
            }
        }
        _ => {
            cli.outputter().output(&results)?;
        }
    }

    Ok(())
}
