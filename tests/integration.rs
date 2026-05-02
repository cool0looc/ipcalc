//! Integration tests

use ipcalc::{Ipv4Addr, Ipv4Network, IpClass, is_private};

#[test]
fn test_ipv4_validation() {
    assert!(Ipv4Addr::from_dotted("192.168.1.1").is_ok());
    assert!(Ipv4Addr::from_dotted("192.168.1.256").is_err());
    assert!(Ipv4Addr::from_dotted("192.168.1").is_err());
}

#[test]
fn test_cidr_validation() {
    assert!(Ipv4Network::from_cidr("192.168.1.0/24").is_ok());
    assert!(Ipv4Network::from_cidr("192.168.1.0/33").is_err());
    assert!(Ipv4Network::from_cidr("192.168.1.0").is_err());
}

#[test]
fn test_subnet_calculation() {
    let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
    
    assert_eq!(network.netmask().to_string(), "255.255.255.0");
    assert_eq!(network.broadcast().to_string(), "192.168.1.255");
    assert_eq!(network.usable_hosts(), 254);
}

#[test]
fn test_ip_classification() {
    assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("10.0.0.1").unwrap()), IpClass::A);
    assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("172.16.0.1").unwrap()), IpClass::B);
    assert_eq!(IpClass::classify(Ipv4Addr::from_dotted("192.168.1.1").unwrap()), IpClass::C);
}

#[test]
fn test_private_ip_detection() {
    assert!(is_private(Ipv4Addr::from_dotted("10.0.0.1").unwrap()));
    assert!(is_private(Ipv4Addr::from_dotted("172.16.0.1").unwrap()));
    assert!(is_private(Ipv4Addr::from_dotted("192.168.1.1").unwrap()));
    assert!(!is_private(Ipv4Addr::from_dotted("8.8.8.8").unwrap()));
}

#[test]
fn test_network_contains() {
    let network = Ipv4Network::from_cidr("192.168.1.0/24").unwrap();
    let inside = Ipv4Addr::from_dotted("192.168.1.100").unwrap();
    let outside = Ipv4Addr::from_dotted("192.168.2.1").unwrap();
    
    assert!(network.contains(inside));
    assert!(!network.contains(outside));
}

#[test]
fn test_prefix_for_hosts() {
    assert_eq!(Ipv4Network::prefix_for_hosts(254), 24);
    assert_eq!(Ipv4Network::prefix_for_hosts(126), 25);
    assert_eq!(Ipv4Network::prefix_for_hosts(62), 26);
}

#[test]
fn test_integer_conversion() {
    let addr = Ipv4Addr::from_dotted("192.168.1.1").unwrap();
    assert_eq!(addr.to_int(), 3232235777);
    
    let addr2 = Ipv4Addr::from_int(3232235777);
    assert_eq!(addr2.to_string(), "192.168.1.1");
}
