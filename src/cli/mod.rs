//! CLI module - Command-line interface definitions and implementations

pub mod validate;
pub mod subnet;
pub mod range;
pub mod expand;
pub mod summarize;
pub mod divide;
pub mod vlsm;
pub mod classify;
pub mod private;
pub mod convert;
pub mod lookup;
pub mod visualize;
pub mod conflict;
pub mod plan;
pub mod dhcp;

use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;
use crate::formats::{OutputFormat, Outputter};

/// IP Calculator - A comprehensive IP address calculator
#[derive(Parser, Debug)]
#[command(
    name = "ipcalc",
    about = "IP address calculator for IPv4 and IPv6",
    version,
    author,
    long_about = None,
    next_help_heading = "Options"
)]
pub struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Human)]
    pub format: OutputFormat,

    /// Strict mode: fail on any invalid input
    #[arg(long, global = true)]
    pub strict: bool,

    /// Input file (for batch processing)
    #[arg(short = 'i', long, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// The subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

/// All available subcommands
#[derive(Parser, Debug)]
pub enum Commands {
    /// Validate IP addresses or CIDR notation
    Validate {
        /// IP addresses or CIDRs to validate
        #[arg(required = true, value_name = "IP_OR_CIDR")]
        addresses: Vec<String>,
    },

    /// Calculate subnet information
    Subnet {
        /// CIDR notation networks
        #[arg(required = true, value_name = "CIDR")]
        cidr: Vec<String>,

        /// Show all subnet details
        #[arg(short, long)]
        show_all: bool,
    },

    /// Show IP range for CIDR notation
    Range {
        /// CIDR networks
        #[arg(required = true, value_name = "CIDR")]
        cidrs: Vec<String>,
    },

    /// Convert IP range to CIDR notation
    Expand {
        /// IP range in START-END format
        #[arg(value_name = "START_IP-END_IP", required = true)]
        range: String,

        /// Minimum prefix length (smallest network, default 8)
        #[arg(short = 'n', long, default_value_t = 8)]
        min_prefix: u8,

        /// Maximum prefix length (largest network, default 32)
        #[arg(short = 'm', long, default_value_t = 32)]
        max_prefix: u8,
    },

    /// Aggregate multiple CIDRs into supernets
    Summarize {
        /// CIDR networks to aggregate
        #[arg(required = true, value_name = "CIDR")]
        cidrs: Vec<String>,
    },

    /// Divide a network into subnets
    Divide {
        /// Network to divide (CIDR notation)
        #[arg(value_name = "CIDR", required = true)]
        cidr: String,

        /// Number of subnets
        #[arg(value_name = "NUM_SUBNETS", required = true)]
        num_subnets: u32,
    },

    /// VLSM allocation calculator
    Vlsm {
        /// Parent network (CIDR notation)
        #[arg(value_name = "CIDR", required = true)]
        cidr: String,

        /// Host requirements (comma-separated or space-separated)
        #[arg(required = true, value_name = "REQUIREMENTS", num_args = 1..)]
        requirements: Vec<String>,
    },

    /// Classify IP address (A/B/C/D/E class)
    Classify {
        /// IP addresses to classify
        #[arg(required = true, value_name = "IP")]
        addresses: Vec<String>,
    },

    /// Check if IP is private or public
    Private {
        /// IP addresses to check
        #[arg(required = true, value_name = "IP")]
        addresses: Vec<String>,
    },

    /// Convert IP between different formats
    Convert {
        /// IP address to convert
        #[arg(value_name = "IP", required = true)]
        ip: String,

        /// Output format
        #[arg(long, value_enum, default_value_t = ConvertFormat::Integer)]
        to_format: ConvertFormat,
    },

    /// Lookup well-known network ranges
    Lookup {
        /// Network name to lookup
        #[arg(value_name = "NETWORK_NAME")]
        network: Option<String>,

        /// List all available networks
        #[arg(short, long)]
        list: bool,
    },

    /// Visualize network hierarchy as ASCII tree
    Visualize {
        /// Network to visualize (CIDR notation)
        #[arg(value_name = "CIDR", required = true)]
        cidr: String,

        /// Maximum tree depth
        #[arg(short, long)]
        depth: Option<u8>,
    },

    /// Detect IP address conflicts
    Conflict {
        /// CIDR networks to check for conflicts
        #[arg(required = true, value_name = "CIDR")]
        cidrs: Vec<String>,
    },

    /// Subnet planning and recommendations
    Plan {
        /// Network to plan (CIDR notation)
        #[arg(value_name = "CIDR", required = true)]
        cidr: String,

        /// Expected number of hosts per subnet
        #[arg(value_name = "HOST_COUNT")]
        requirements: Vec<String>,
    },

    /// DHCP scope planning
    Dhcp {
        /// Network for DHCP scope (CIDR notation)
        #[arg(value_name = "CIDR", required = true)]
        cidr: String,

        /// Number of static reservations to plan
        #[arg(short, long)]
        reservations: Option<u32>,

        /// Number of exclusion ranges
        #[arg(short, long)]
        exclusions: Option<u32>,
    },
}

/// Convert command format options
#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize)]
pub enum ConvertFormat {
    /// Dotted decimal notation
    Dotted,
    /// Integer representation
    Integer,
    /// Binary representation
    Binary,
    /// Hexadecimal representation
    Hex,
}

impl Cli {
    /// Get outputter based on format selection
    pub fn outputter(&self) -> Outputter {
        Outputter::new(self.format)
    }

    /// Check if strict mode is enabled
    pub fn is_strict(&self) -> bool {
        self.strict
    }
}
