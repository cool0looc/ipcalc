//! Output formats - JSON, YAML, CSV, and human-readable output
//! 
//! Provides multiple output format support for CLI commands.

pub mod json;
pub mod yaml;
pub mod csv;
pub mod human;

use serde::{Deserialize, Serialize};
use std::io;

/// Output format selection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// JSON output
    Json,
    /// YAML output
    Yaml,
    /// CSV output
    Csv,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Human
    }
}

/// Outputter - handles formatting and outputting data
pub struct Outputter {
    format: OutputFormat,
}

impl Outputter {
    /// Create a new outputter with the specified format
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Output data in the specified format
    pub fn output<T: Serialize>(&self, data: &T) -> io::Result<()> {
        match self.format {
            OutputFormat::Human => {
                // Human-readable output is handled by individual commands
                // This is a fallback
                let json = serde_json::to_string_pretty(data).unwrap_or_default();
                println!("{}", json);
            }
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(data)?;
                println!("{}", json);
            }
            OutputFormat::Yaml => {
                let yaml = serde_yaml::to_string(data)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                println!("{}", yaml);
            }
            OutputFormat::Csv => {
                // CSV is handled specially by commands
                // This is a fallback
                let json = serde_json::to_string_pretty(data).unwrap_or_default();
                println!("{}", json);
            }
        }
        Ok(())
    }

    /// Print a line
    #[allow(dead_code)]
    pub fn print(&self, line: &str) {
        println!("{}", line);
    }

    /// Print an error
    #[allow(dead_code)]
    pub fn error(&self, msg: &str) {
        eprintln!("Error: {}", msg);
    }
}
