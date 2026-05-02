//! Expand command - Convert IP range to CIDR

use crate::cli::Cli;
use crate::core::ipv4::address::Ipv4Addr;
use crate::utils::parse_ip_range;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandResult {
    pub ip_range: String,
    pub start_ip: String,
    pub end_ip: String,
    pub total_ips: u32,
    pub cidrs: Vec<CidrBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CidrBlock {
    pub cidr: String,
    pub start: String,
    pub end: String,
    pub size: u32,
    pub usable: u32,
}

pub fn run(cli: &Cli, range: String, min_prefix: u8, max_prefix: u8) -> Result<()> {
    let (start, end) = parse_ip_range(&range).map_err(|e| anyhow::anyhow!(e))?;
    
    let result = expand_range_to_cidrs(start, end, min_prefix, max_prefix)?;

    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("\nIP Range:     {}", range);
            println!("Total IPs:   {} (+2 reserved)", result.total_ips);
            println!("\nCIDR Blocks:");
            for cidr in &result.cidrs {
                println!("  {}  ({} - {})  [{} IPs, {} usable]", 
                    cidr.cidr, cidr.start, cidr.end, cidr.size, cidr.usable);
            }
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }

    Ok(())
}

/// Find the minimal CIDR that contains the entire IP range
/// This finds the smallest possible CIDR that covers from start to end
fn expand_range_to_cidrs(start: Ipv4Addr, end: Ipv4Addr, _min_prefix: u8, _max_prefix: u8) -> Result<ExpandResult> {
    let start_int = start.to_int();
    let end_int = end.to_int();
    let total_ips = end_int - start_int + 1;
    
    // Find the smallest CIDR that contains the entire range
    let (network_addr, prefix) = find_minimal_cidr(start_int, end_int);
    
    let network = Ipv4Addr::from_int(network_addr);
    let network_cidr = format!("{}/{}", network, prefix);
    
    // Calculate usable hosts (excluding network and broadcast for /31 and /32)
    let total_size = 1u64 << (32 - prefix) as u64;
    let usable = if total_size <= 2 {
        total_size as u32 // For /31 and /32, all addresses are usable
    } else {
        total_size as u32 - 2 // Exclude network and broadcast
    };
    
    Ok(ExpandResult {
        ip_range: format!("{}-{}", start, end),
        start_ip: start.to_string(),
        end_ip: end.to_string(),
        total_ips,
        cidrs: vec![CidrBlock {
            cidr: network_cidr,
            start: start.to_string(),
            end: end.to_string(),
            size: total_size as u32,
            usable,
        }],
    })
}

/// Find the minimal CIDR that contains the range from start to end
/// Uses XOR-based common prefix calculation to find the true network boundary
fn find_minimal_cidr(start: u32, end: u32) -> (u32, u8) {
    // Find the common prefix length between start and end
    let xor_result = start ^ end;
    
    // Count leading zeros in XOR result - this tells us how many bits are the same
    let common_bits = xor_result.leading_zeros();
    
    // The prefix is the number of common bits
    let prefix = common_bits as u8;
    
    // Calculate the network address by masking with the appropriate netmask
    let netmask = if prefix == 0 {
        0u32
    } else {
        !0u32 << (32 - prefix)
    };
    let network_addr = start & netmask;
    
    (network_addr, prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_full_network() {
        let start = Ipv4Addr::from_dotted("192.168.1.0").unwrap();
        let end = Ipv4Addr::from_dotted("192.168.1.255").unwrap();
        let result = expand_range_to_cidrs(start, end, 8, 32).unwrap();
        
        assert_eq!(result.cidrs.len(), 1);
        assert_eq!(result.cidrs[0].cidr, "192.168.1.0/24");
    }

    #[test]
    fn test_expand_nearly_full_network() {
        let start = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        let end = Ipv4Addr::from_dotted("192.168.1.254").unwrap();
        let result = expand_range_to_cidrs(start, end, 8, 32).unwrap();
        
        // Should return the containing /24
        assert_eq!(result.cidrs.len(), 1);
        assert_eq!(result.cidrs[0].cidr, "192.168.1.0/24");
        assert_eq!(result.cidrs[0].start, "192.168.1.1");
        assert_eq!(result.cidrs[0].end, "192.168.1.254");
    }

    #[test]
    fn test_expand_single_ip() {
        let start = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        let end = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        let result = expand_range_to_cidrs(start, end, 8, 32).unwrap();
        
        assert_eq!(result.cidrs.len(), 1);
        assert_eq!(result.cidrs[0].cidr, "192.168.1.1/32");
    }

    #[test]
    fn test_expand_two_ips() {
        let start = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
        let end = Ipv4Addr::from_dotted("192.168.1.2").unwrap();
        let result = expand_range_to_cidrs(start, end, 8, 32).unwrap();
        
        assert_eq!(result.cidrs.len(), 1);
        // /30 for two non-aligned IPs (network at 192.168.1.0 spans 0-3)
        // Note: /31 is for point-to-point links with exact alignment at .0/.2
        // 192.168.1.1 and 192.168.1.2 don't align to /31 boundaries
        assert_eq!(result.cidrs[0].cidr, "192.168.1.0/30");
    }

    #[test]
    fn test_expand_small_network() {
        let start = Ipv4Addr::from_dotted("192.168.1.0").unwrap();
        let end = Ipv4Addr::from_dotted("192.168.1.63").unwrap();
        let result = expand_range_to_cidrs(start, end, 8, 32).unwrap();
        
        assert_eq!(result.cidrs.len(), 1);
        assert_eq!(result.cidrs[0].cidr, "192.168.1.0/26");
    }
    
    #[test]
    fn test_expand_across_subnet_boundary() {
        // Range that spans two /24 networks
        let start = Ipv4Addr::from_dotted("192.168.1.200").unwrap();
        let end = Ipv4Addr::from_dotted("192.168.2.55").unwrap();
        let result = expand_range_to_cidrs(start, end, 8, 32).unwrap();
        
        assert_eq!(result.cidrs.len(), 1);
        // XOR of 192.168.1.200 (0xC0A801C8) and 192.168.2.55 (0xC0A80237) = 0x000003FF
        // 200 ^ 55 = 255 (0xFF), so 1 ^ 2 = 3 for the third octet
        // Result: 0x000003FF = 1023, leading_zeros = 22
        // prefix = 22, network = 192.168.0.0/22 (spans 0.0-3.255)
        assert_eq!(result.cidrs[0].cidr, "192.168.0.0/22");
    }
}
