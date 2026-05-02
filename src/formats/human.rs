//! Human-readable output formatter

use serde::Serialize;
use std::io;

#[allow(dead_code)]
pub struct HumanFormatter;

#[allow(dead_code)]
impl HumanFormatter {
    pub fn format<T: Serialize>(&self, _data: &T) -> io::Result<String> {
        // For human format, we rely on commands to print directly
        // This is a no-op fallback
        Ok(String::new())
    }
}
