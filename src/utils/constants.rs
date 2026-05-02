//! Constants for well-known networks

use lazy_static::lazy_static;
use std::collections::HashMap;

/// Well-known network information
pub struct NetworkInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub rfc: &'static str,
}

lazy_static! {
    pub static ref KNOWN_NETWORK_INFO: HashMap<&'static str, NetworkInfo> = {
        let mut m = HashMap::new();
        
        m.insert("RFC1918", NetworkInfo {
            name: "Private Addresses",
            description: "Private IP addresses for use in internal networks",
            rfc: "RFC1918",
        });
        
        m.insert("LOOPBACK", NetworkInfo {
            name: "Loopback",
            description: "Loopback addresses (127.0.0.0/8)",
            rfc: "RFC1122",
        });
        
        m.insert("LINKLOCAL", NetworkInfo {
            name: "Link-Local",
            description: "Link-local addresses (169.254.0.0/16)",
            rfc: "RFC3927",
        });
        
        m.insert("MULTICAST", NetworkInfo {
            name: "Multicast",
            description: "Multicast addresses (224.0.0.0/4)",
            rfc: "RFC5771",
        });
        
        m.insert("TEST-NET-1", NetworkInfo {
            name: "TEST-NET-1",
            description: "Documentation network (192.0.2.0/24)",
            rfc: "RFC5737",
        });
        
        m.insert("TEST-NET-2", NetworkInfo {
            name: "TEST-NET-2",
            description: "Documentation network (198.51.100.0/24)",
            rfc: "RFC5737",
        });
        
        m.insert("TEST-NET-3", NetworkInfo {
            name: "TEST-NET-3",
            description: "Documentation network (203.0.113.0/24)",
            rfc: "RFC5737",
        });
        
        m.insert("DOCUMENTATION", NetworkInfo {
            name: "Documentation",
            description: "Documentation networks",
            rfc: "RFC5737",
        });
        
        m.insert("CGNAT", NetworkInfo {
            name: "Shared Address Space",
            description: "Carrier-grade NAT (100.64.0.0/10)",
            rfc: "RFC6598",
        });
        
        m.insert("6TO4", NetworkInfo {
            name: "6to4 Relay",
            description: "6to4 relay anycast addresses (192.88.99.0/24)",
            rfc: "RFC3068",
        });
        
        m.insert("BENCHMARK", NetworkInfo {
            name: "Benchmark Testing",
            description: "Benchmark testing (198.18.0.0/15)",
            rfc: "RFC2544",
        });
        
        m
    };
}
