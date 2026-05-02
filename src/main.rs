//! IP Calculator - A comprehensive IP address calculator for IPv4 and IPv6
//! 
//! This tool provides various IP address calculations including subnetting,
//! CIDR operations, VLSM allocation, and conversions between different formats.

mod cli;
mod core;
mod formats;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::process::exit;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(&cli) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run(cli: &Cli) -> Result<()> {
    match &cli.command {
        Commands::Validate { addresses } => cli::validate::run(cli, addresses.clone()),
        Commands::Subnet { cidr, show_all } => cli::subnet::run(cli, cidr.clone(), *show_all),
        Commands::Range { cidrs } => cli::range::run(cli, cidrs.clone()),
        Commands::Expand { range, min_prefix, max_prefix } => {
            cli::expand::run(cli, range.clone(), *min_prefix, *max_prefix)
        }
        Commands::Summarize { cidrs } => cli::summarize::run(cli, cidrs.clone()),
        Commands::Divide { cidr, num_subnets } => cli::divide::run(cli, cidr.clone(), *num_subnets),
        Commands::Vlsm { cidr, requirements } => cli::vlsm::run(cli, cidr.clone(), requirements.clone()),
        Commands::Classify { addresses } => cli::classify::run(cli, addresses.clone()),
        Commands::Private { addresses } => cli::private::run(cli, addresses.clone()),
        Commands::Convert { ip, to_format } => cli::convert::run(cli, ip.clone(), *to_format),
        Commands::Lookup { network, list } => cli::lookup::run(cli, network.clone(), *list),
        Commands::Visualize { cidr, depth } => cli::visualize::run(cli, cidr.clone(), *depth),
        Commands::Conflict { cidrs } => cli::conflict::run(cli, cidrs.clone()),
        Commands::Plan { cidr, requirements } => cli::plan::run(cli, cidr.clone(), requirements.clone()),
        Commands::Dhcp { cidr, reservations, exclusions } => {
            cli::dhcp::run(cli, cidr.clone(), *reservations, *exclusions)
        }
    }
}
