//! Utility functions

pub mod constants;
pub mod validation;
pub mod parsing;

use crate::core::ipv4::address::Ipv4Addr;
use crate::core::ipv4::network::Ipv4Network;
use crate::core::{IpError, Result};

/// Parse an IP address or CIDR
#[allow(dead_code)]
pub fn parse_ip_or_cidr(input: &str) -> Result<(Ipv4Addr, Option<u8>)> {
    if input.contains('/') {
        let network = Ipv4Network::from_cidr(input)?;
        Ok((network.address(), Some(network.prefix_len())))
    } else {
        let addr = Ipv4Addr::from_dotted(input)?;
        Ok((addr, None))
    }
}

/// Parse an IP range (START-END format)
pub fn parse_ip_range(range: &str) -> Result<(Ipv4Addr, Ipv4Addr)> {
    let parts: Vec<&str> = range.split('-').collect();
    
    if parts.len() != 2 {
        return Err(IpError::InvalidFormat(format!(
            "Invalid IP range format: {} (expected 'START-END')",
            range
        )));
    }

    let start = Ipv4Addr::from_dotted(parts[0].trim())?;
    let end = Ipv4Addr::from_dotted(parts[1].trim())?;

    if start > end {
        return Err(IpError::InvalidRange(
            start.to_string(),
            end.to_string()
        ));
    }

    Ok((start, end))
}
