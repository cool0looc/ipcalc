//! Convert command - Convert IP between different formats

use crate::cli::{Cli, ConvertFormat};
use crate::core::ipv4::address::Ipv4Addr;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertResult {
    pub input: String,
    pub dotted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

pub fn run(cli: &Cli, ip: String, to_format: ConvertFormat) -> Result<()> {
    let addr = Ipv4Addr::from_dotted(&ip).map_err(|e| anyhow::anyhow!(e))?;

    let mut result = ConvertResult {
        input: ip.clone(),
        dotted: addr.to_string(),
        integer: None,
        binary: None,
        hex: None,
    };

    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("Address:     {}", result.dotted);
            
            match to_format {
                ConvertFormat::Dotted => println!("Dotted:      {}", result.dotted),
                ConvertFormat::Integer => println!("Integer:     {}", addr.to_int()),
                ConvertFormat::Binary => println!("Binary:      {}", addr.to_binary()),
                ConvertFormat::Hex => println!("Hex:         {}", addr.to_hex()),
            }
        }
        _ => {
            result.integer = Some(addr.to_int().to_string());
            result.binary = Some(addr.to_binary());
            result.hex = Some(addr.to_hex());
            cli.outputter().output(&result)?;
        }
    }

    Ok(())
}
