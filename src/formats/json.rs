//! JSON output formatter

use serde::Serialize;
use std::io;

#[allow(dead_code)]
pub struct JsonFormatter;

#[allow(dead_code)]
impl JsonFormatter {
    pub fn format<T: Serialize>(&self, data: &T) -> io::Result<String> {
        serde_json::to_string_pretty(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
