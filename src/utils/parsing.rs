//! Parsing utilities

use crate::core::{IpError, Result};

/// Parse a comma-separated list of integers
#[allow(dead_code)]
pub fn parse_comma_separated(s: &str) -> Result<Vec<u32>> {
    s.split(',')
        .map(|part| {
            part.trim()
                .parse::<u32>()
                .map_err(|_| IpError::ParseError(format!("Invalid number: {}", part)))
        })
        .collect()
}

/// Parse multiple values that can be comma-separated or space-separated
#[allow(dead_code)]
pub fn parse_multi_value(s: &str) -> Vec<String> {
    s.split(|c| c == ',' || c == ' ')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
