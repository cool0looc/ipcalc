//! Plan command - Subnet planning suggestions and recommendations

use crate::cli::Cli;
use crate::core::ipv4::network::Ipv4Network;
use crate::core::ipv4::private;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanResult {
    pub network: String,
    pub suggestions: Vec<PlanningSuggestion>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningSuggestion {
    pub title: String,
    pub description: String,
    pub suggested_prefix: u8,
    pub estimated_subnets: u32,
    pub hosts_per_subnet: u32,
    pub suitability: String,
}

pub fn run(cli: &Cli, cidr: String, requirements: Vec<String>) -> Result<()> {
    let network = Ipv4Network::from_cidr(&cidr).map_err(|e| anyhow::anyhow!(e))?;
    
    let suggestions = generate_suggestions(&network, &requirements);
    let recommendations = generate_recommendations(&network, &suggestions);
    
    let result = PlanResult {
        network: cidr.clone(),
        suggestions: suggestions.clone(),
        recommendations,
    };
    
    match cli.format {
        crate::cli::OutputFormat::Human => {
            println!("\nSubnet Planning Analysis for {}", cidr);
            println!("{}", "=".repeat(60));
            
            // Network info
            let total_hosts = network.total_addrs();
            let usable_hosts = if total_hosts > 2 { total_hosts - 2 } else { total_hosts };
            
            println!("\nNetwork Information:");
            println!("  Address:    {}", network.address());
            println!("  Netmask:    {}", network.netmask());
            println!("  Broadcast:  {}", network.broadcast());
            println!("  Total IPs: {}", total_hosts);
            println!("  Usable:     {}", usable_hosts);
            
            // Check if private
            let is_private = private::is_private(network.address());
            println!("  Type:       {}", if is_private { "Private" } else { "Public" });
            
            // Suggestions
            println!("\nSubnetting Suggestions:");
            println!("{}", "-".repeat(60));
            
            for (i, suggestion) in suggestions.iter().enumerate() {
                println!("\n{}. {} [Suitability: {}]", i + 1, suggestion.title, suggestion.suggested_prefix);
                println!("   {}", suggestion.description);
                println!("   Subnets: {}, Hosts/subnet: ~{}", 
                    suggestion.estimated_subnets, suggestion.hosts_per_subnet);
                println!("   Rating: {}", suggestion.suitability);
            }
            
            // Recommendations
            println!("\n{}", "=".repeat(60));
            println!("Recommendations:");
            for rec in &result.recommendations {
                println!("  • {}", rec);
            }
        }
        _ => {
            cli.outputter().output(&result)?;
        }
    }
    
    Ok(())
}

fn generate_suggestions(network: &Ipv4Network, requirements: &[String]) -> Vec<PlanningSuggestion> {
    let mut suggestions = Vec::new();
    let current_prefix = network.prefix_len();
    let _available_bits = 32 - current_prefix; // Reserved for future calculations
    
    // Analyze requirements
    let mut max_hosts_needed = 0u32;
    for req in requirements {
        if let Ok(num) = req.parse::<u32>() {
            max_hosts_needed = max_hosts_needed.max(num);
        }
    }
    
    // Generate suggestions for different prefix lengths
    let mut tried_prefixes = std::collections::HashSet::new();
    
    for new_prefix in (current_prefix + 1)..=32 {
        if tried_prefixes.contains(&new_prefix) {
            continue;
        }
        tried_prefixes.insert(new_prefix);
        
        let subnet_bits = new_prefix - current_prefix;
        let num_subnets = 1u32 << subnet_bits;
        let hosts_per_subnet = (1u32 << (32 - new_prefix)).saturating_sub(2);
        
        // Calculate suitability
        let suitability = if hosts_per_subnet >= max_hosts_needed && max_hosts_needed > 0 {
            if hosts_per_subnet <= max_hosts_needed * 2 {
                "Excellent"
            } else if hosts_per_subnet <= max_hosts_needed * 4 {
                "Good"
            } else {
                "Adequate"
            }
        } else if hosts_per_subnet >= 10 {
            "Good for small networks"
        } else if hosts_per_subnet >= 2 {
            "Good for point-to-point"
        } else {
            "Too small"
        };
        
        suggestions.push(PlanningSuggestion {
            title: format!("Divide into {}{}", num_subnets, 
                if num_subnets == 1 { " subnet" } else { " subnets" }),
            description: format!(
                "Using /{} creates {} subnets with ~{} usable hosts each",
                new_prefix, num_subnets, hosts_per_subnet
            ),
            suggested_prefix: new_prefix,
            estimated_subnets: num_subnets,
            hosts_per_subnet,
            suitability: suitability.to_string(),
        });
    }
    
    // Sort by suitability
    suggestions.sort_by(|a, b| {
        let order = |s: &str| {
            match s {
                "Excellent" => 0,
                "Good" | "Good for small networks" => 1,
                "Adequate" => 2,
                "Good for point-to-point" => 3,
                _ => 4,
            }
        };
        order(&a.suitability).cmp(&order(&b.suitability))
    });
    
    // Return top suggestions
    suggestions.into_iter().take(5).collect()
}

fn generate_recommendations(network: &Ipv4Network, suggestions: &[PlanningSuggestion]) -> Vec<String> {
    let mut recs = Vec::new();
    let usable_hosts = network.total_addrs().saturating_sub(2);
    
    // Basic recommendations
    recs.push(format!(
        "Reserve first IP ({}) for network address",
        network.address()
    ));
    
    recs.push(format!(
        "Reserve last IP ({}) for broadcast address",
        network.broadcast()
    ));
    
    recs.push(format!(
        "Recommended gateway: {}",
        increment_ip(&network.address().to_string())
    ));
    
    // DHCP recommendation
    if usable_hosts > 50 {
        recs.push(format!(
            "For DHCP, consider reserving {} - {} for static assignments",
            increment_ip(&network.address().to_string()),
            increment_ip_by(&network.address().to_string(), 10)
        ));
    }
    
    // VLSM recommendation
    if suggestions.len() > 1 {
        recs.push(format!(
            "Consider VLSM for more efficient address allocation (see: ipcalc vlsm)"
        ));
    }
    
    recs
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

fn increment_ip_by(ip: &str, offset: u32) -> String {
    use crate::core::ipv4::address::Ipv4Addr;
    if let Ok(addr) = Ipv4Addr::from_dotted(ip) {
        let next = addr.to_int() + offset;
        Ipv4Addr::from_int(next).to_string()
    } else {
        ip.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_suggestions() {
        let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
        let suggestions = generate_suggestions(&network, &["50".to_string()]);
        
        assert!(!suggestions.is_empty());
    }
}
