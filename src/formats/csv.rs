//! CSV output formatter

use serde::Serialize;
use std::io;

#[allow(dead_code)]
pub struct CsvFormatter;

#[allow(dead_code)]
impl CsvFormatter {
    pub fn format<T: Serialize>(&self, data: &T) -> io::Result<String> {
        // CSV formatting is typically handled by commands directly
        // This is a basic implementation
        let json = serde_json::to_value(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        Ok(json.to_string())
    }
}
