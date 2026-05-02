//! IP Calculator Library
//! 
//! This library provides IP address calculation functionality for IPv4 and IPv6.

pub mod cli;
pub mod core;
pub mod formats;
pub mod utils;

// Re-export commonly used types
pub use core::ipv4::address::Ipv4Addr;
pub use core::ipv4::network::Ipv4Network;
pub use core::ipv4::classify::IpClass;
pub use core::ipv4::private::{is_private, is_public, detect_type};
pub use core::ipv6::address::Ipv6Addr;
pub use core::ipv6::network::Ipv6Network;
pub use core::cidr::collapse::{range_to_cidrs, cidrs_to_range};
pub use core::vlsm::{calculate_vlsm, can_allocate};
