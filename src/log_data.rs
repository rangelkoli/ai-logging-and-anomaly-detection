// Placeholder for log data structures (e.g., ParsedLogEntry, LogLevel)
// Will be implemented in Phase 1
use std::collections::HashMap; // We need HashMap for the 'fields' part

#[derive(Debug, PartialEq, Clone)] // Add Debug for printing, PartialEq for comparisons (useful in tests)
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Trace,
    Critical, // Example of adding another level
    Unknown,  // For cases where the level can't be parsed
}

#[derive(Debug)]
pub struct ParsedLogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub fields: HashMap<String, String>,

}

// Helper function to convert string to LogLevel
fn string_to_log_level(level_str: &str) -> LogLevel {
    match level_str.to_uppercase().as_str() { // Case-insensitive matching
        "INFO" => LogLevel::Info,
        "WARNING" | "WARN" => LogLevel::Warning, // Allow variations
        "ERROR" | "ERR" => LogLevel::Error,
        "DEBUG" | "DBG" => LogLevel::Debug,
        "TRACE" | "TRC" => LogLevel::Trace,
        "CRITICAL" | "CRIT" | "FATAL" => LogLevel::Critical,
        _ => LogLevel::Unknown, // Default to Unknown if not matched
    }
}