//! Core module - IP address calculation logic
//! 
//! This module contains all the core IP calculation functions including
//! IPv4 and IPv6 address parsing, validation, and various network operations.

pub mod ipv4;
pub mod ipv6;
pub mod cidr;
pub mod vlsm;

use thiserror::Error;

/// Core error types for IP calculations
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum IpError {
    #[error("Invalid IP address format: {0}")]
    InvalidFormat(String),
    
    #[error("Octet out of range: {0} (valid: 0-255)")]
    OctetOutOfRange(u32),
    
    #[error("Invalid prefix length: {0} (valid: 0-{1})")]
    InvalidPrefixLength(u8, u8),
    
    #[error("Invalid CIDR notation: {0}")]
    InvalidCidr(String),
    
    #[error("IP range error: {0}")]
    RangeError(String),
    
    #[error("Insufficient address space: need {0} bits, have {1} bits")]
    InsufficientSpace(u8, u8),
    
    #[error("Invalid IP range: start {0} > end {1}")]
    InvalidRange(String, String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type for IP operations
pub type Result<T> = std::result::Result<T, IpError>;

/// IP version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IpVersion {
    /// IPv4
    V4,
    /// IPv6
    V6,
}

impl std::fmt::Display for IpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpVersion::V4 => write!(f, "IPv4"),
            IpVersion::V6 => write!(f, "IPv6"),
        }
    }
}
