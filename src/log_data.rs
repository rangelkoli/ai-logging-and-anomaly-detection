// Placeholder for log data structures (e.g., ParsedLogEntry, LogLevel)
// Will be implemented in Phase 1
use std::collections::HashMap; // We need HashMap for the 'fields' part

/// Represents errors that can occur during log line parsing.
#[derive(Debug, PartialEq)] // Allow printing and comparison
pub enum ParseError {
    InvalidFormat,
    UnknownLevel(String),

}

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
}

