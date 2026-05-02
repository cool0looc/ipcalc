//! Validate command - Check if IP addresses or CIDRs are valid

use crate::cli::Cli;
use crate::core::ipv4::address::Ipv4Addr;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub input: String,
    pub valid: bool,
    pub ip: Option<String>,
    pub cidr: Option<String>,
    pub error: Option<String>,
}

pub fn run(cli: &Cli, addresses: Vec<String>) -> Result<()> {
    let mut results = Vec::new();

    for addr in addresses {
        let result = validate_address(&addr, cli.is_strict());
        results.push(result);
    }

    // Output results
    match cli.format {
        crate::cli::OutputFormat::Human => {
            for result in &results {
                if result.valid {
                    println!("✓ {} is valid", result.input);
                } else {
                    println!("✗ {} is invalid: {}", result.input, result.error.as_ref().unwrap());
                }
            }
        }
        _ => {
            cli.outputter().output(&results)?;
        }
    }

    // In strict mode, fail if any input is invalid
    if cli.is_strict() && results.iter().any(|r| !r.valid) {
        anyhow::bail!("Validation failed in strict mode");
    }

    Ok(())
}

fn validate_address(input: &str, _strict: bool) -> ValidationResult {
    let mut result = ValidationResult {
        input: input.to_string(),
        valid: false,
        ip: None,
        cidr: None,
        error: None,
    };

    // Try parsing as CIDR first
    if let Ok(network) = Ipv4Network::from_cidr(input) {
        result.valid = true;
        result.ip = Some(network.address().to_string());
        result.cidr = Some(network.prefix_len().to_string());
        return result;
    }

    // Try parsing as plain IP
    if let Ok(ip) = Ipv4Addr::from_dotted(input) {
        result.valid = true;
        result.ip = Some(ip.to_string());
        return result;
    }

    // Invalid format
    result.error = Some(format!("Invalid IP address or CIDR notation: {}", input));
    result
}
