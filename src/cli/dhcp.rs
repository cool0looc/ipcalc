//! DHCP command - DHCP scope planning and calculation

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DhcpResult {
    pub network: String,
    pub scope: DhcpScope,
    pub reservations: Vec<Reservation>,
    pub exclusions: Vec<ExclusionRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpScope {
    pub start: String,
    pub end: String,
    pub total: u32,
    pub subnet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub ip: String,
    pub mac: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionRange {
    pub start: String,
    pub end: String,
    pub reason: String,
}

pub fn run(cli: &Cli, cidr: String, reservation_count: Option<u32>, exclusion_count: Option<u32>) -> Result<()> {
    let network = Ipv4Network::from_cidr(&cidr).map_err(|e| anyhow::anyhow!(e))?;
    
    let scope = calculate_dhcp_scope(&network);
    let reservations = generate_sample_reservations(reservation_count.unwrap_or(0));
    let exclusions = generate_recommended_exclusions(&network, exclusion_count.unwrap_or(3));
    
    let result = DhcpResult {
        network: cidr.clone(),
        scope,
        reservations,
        exclusions,
    };
    
    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("\nDHCP Scope Planning for {}", cidr);
            println!("{}", "=".repeat(60));
            
            // Scope information
            println!("\nDHCP Scope:");
            println!("  Network:    {}", cidr);
            println!("  Subnet:     {}", result.scope.subnet);
            println!("  Range:      {} - {}", result.scope.start, result.scope.end);
            println!("  Total IPs: {}", result.scope.total);
            
            // Static reservations
            println!("\n{}", "-".repeat(60));
            println!("Recommended Static Reservations:");
            println!("(First {} addresses reserved for infrastructure)", result.reservations.len());
            
            for (i, res) in result.reservations.iter().enumerate() {
                println!("  {}: {} ({})", i + 1, res.ip, res.purpose);
                println!("          MAC: {}", res.mac);
            }
            
            // Exclusions
            println!("\n{}", "-".repeat(60));
            println!("Recommended Exclusions:");
            println!("(Addresses excluded from DHCP pool for static assignment)");
            
            for (i, excl) in result.exclusions.iter().enumerate() {
                println!("  {}. {} - {} [{}]", 
                    i + 1, excl.start, excl.end, excl.reason);
            }
            
            // Summary
            println!("\n{}", "=".repeat(60));
            let available_for_dhcp = result.scope.total - 
                result.reservations.len() as u32 - 
                result.exclusions.iter().map(|e| {
                    let start: u32 = e.start.split('.').last().unwrap().parse().unwrap_or(0);
                    let end: u32 = e.end.split('.').last().unwrap().parse().unwrap_or(0);
                    end - start + 1
                }).sum::<u32>();
            
            println!("DHCP Pool Summary:");
            println!("  Total IPs in subnet: {}", result.scope.total);
            println!("  Static reservations:  {}", result.reservations.len());
            println!("  Excluded addresses:  {}", 
                result.exclusions.iter().map(|e| {
                    let start: u32 = e.start.split('.').last().unwrap().parse().unwrap_or(0);
                    let end: u32 = e.end.split('.').last().unwrap().parse().unwrap_or(0);
                    end - start + 1
                }).sum::<u32>());
            println!("  Available for DHCP:  {}", available_for_dhcp);
            
            // Example DHCP config
            println!("\n{}", "=".repeat(60));
            println!("Example isc-dhcp-server Configuration:");
            println!("  subnet {} netmask {} {{", 
                network.address(), network.netmask());
            println!("    range {} {};", result.scope.start, result.scope.end);
            println!("    option routers {};", increment_ip(&network.address().to_string()));
            println!("    # Add exclusions and reservations above");
            println!("  }}");
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }
    
    Ok(())
}

fn calculate_dhcp_scope(network: &Ipv4Network) -> DhcpScope {
    let start = increment_ip(&network.address().to_string());
    let end = decrement_ip(&network.broadcast().to_string());
    let total = network.total_addrs().saturating_sub(2);
    
    DhcpScope {
        start,
        end,
        total,
        subnet: network.netmask().to_string(),
    }
}

fn generate_sample_reservations(count: u32) -> Vec<Reservation> {
    let mut reservations = Vec::new();
    
    let purposes = vec![
        ("Gateway/Router", "00:11:22:33:44:01"),
        ("Primary DNS", "00:11:22:33:44:02"),
        ("Secondary DNS", "00:11:22:33:44:03"),
        ("Print Server", "00:11:22:33:44:04"),
        ("Network Storage", "00:11:22:33:44:05"),
        ("Domain Controller", "00:11:22:33:44:06"),
        ("VoIP Server", "00:11:22:33:44:07"),
        ("Camera NVR", "00:11:22:33:44:08"),
        ("Access Point 1", "00:11:22:33:44:09"),
        ("Access Point 2", "00:11:22:33:44:0A"),
    ];
    
    for (i, &(purpose, mac)) in purposes.iter().enumerate() {
        if i as u32 >= count {
            break;
        }
        reservations.push(Reservation {
            ip: format!("192.168.1.{}", i + 1),
            mac: mac.to_string(),
            purpose: purpose.to_string(),
        });
    }
    
    reservations
}

fn generate_recommended_exclusions(network: &Ipv4Network, count: u32) -> Vec<ExclusionRange> {
    let mut exclusions = Vec::new();
    
    // Common exclusions
    let _base_ip = network.address().to_int();
    
    exclusions.push(ExclusionRange {
        start: format!("192.168.1.1"),
        end: format!("192.168.1.10"),
        reason: "Infrastructure (gateway, DNS, servers)".to_string(),
    });
    
    if count >= 2 {
        exclusions.push(ExclusionRange {
            start: format!("192.168.1.11"),
            end: format!("192.168.1.30"),
            reason: "Static workstations/printers".to_string(),
        });
    }
    
    if count >= 3 {
        exclusions.push(ExclusionRange {
            start: format!("192.168.1.200"),
            end: format!("192.168.1.220"),
            reason: "Reserved for future infrastructure".to_string(),
        });
    }
    
    exclusions.truncate(count as usize);
    exclusions
}

fn increment_ip(ip: &str) -> String {
    use crate::core::ipv4::address::Ipv4Addr;
    if let Ok(addr) = Ipv4Addr::from_dotted(ip) {
        let next = addr.to_int() + 1;
        Ipv4Addr::from_int(next).to_string()
    } else {
        ip.to_string()
    }
}

fn decrement_ip(ip: &str) -> String {
    use crate::core::ipv4::address::Ipv4Addr;
    if let Ok(addr) = Ipv4Addr::from_dotted(ip) {
        let prev = addr.to_int().saturating_sub(1);
        Ipv4Addr::from_int(prev).to_string()
    } else {
        ip.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhcp_scope() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        let scope = calculate_dhcp_scope(&network);
        
        assert_eq!(scope.start, "192.168.1.1");
        assert_eq!(scope.end, "192.168.1.254");
        assert_eq!(scope.total, 254);
    }

    #[test]
    fn test_reservations() {
        let reservations = generate_sample_reservations(3);
        assert_eq!(reservations.len(), 3);
    }
}
