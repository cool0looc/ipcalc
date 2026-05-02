//! YAML output formatter

use serde::Serialize;
use std::io;

#[allow(dead_code)]
pub struct YamlFormatter;

#[allow(dead_code)]
impl YamlFormatter {
    pub fn format<T: Serialize>(&self, data: &T) -> io::Result<String> {
        serde_yaml::to_string(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
