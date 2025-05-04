// Placeholder for log parsing logic (e.g., parse_line function)
// Will be implemented in Phase 1
use crate::log_data::{LogLevel, ParsedLogEntry, ParseError};


pub fn parse_line(line: &str) -> Result<ParsedLogEntry, ParseError> {
    // Trim whitespace from the line
    let trimmed_line = line.trim();
    if trimmed_line.is_empty() {
        return Err(ParseError::InvalidFormat); // Handle empty lines
    }

    // Find the first space (after timestamp) and the start/end of the level indicator ([LEVEL])
    let first_space_pos = trimmed_line.find(' ');
    let level_start_pos = trimmed_line.find('[');
    let level_end_pos = trimmed_line.find(']');

    // Basic validation based on positions
    if !(first_space_pos.is_some() && level_start_pos.is_some() && level_end_pos.is_some() &&
         level_start_pos.unwrap() > first_space_pos.unwrap() && level_end_pos.unwrap() > level_start_pos.unwrap()) {
        return Err(ParseError::InvalidFormat);
    }

    let ts_end = first_space_pos.unwrap();
    let level_start = level_start_pos.unwrap();
    let level_end = level_end_pos.unwrap();

    // Extract parts based on positions
    let timestamp = trimmed_line[0..ts_end].to_string();
    // Extract level string, trimming '[' and ']' and whitespace
    let level_str = trimmed_line[level_start + 1..level_end].trim();
    // The rest of the line after '] ' is the message
    let message = trimmed_line[level_end + 1..].trim().to_string();

    if message.is_empty() {
        // Consider if a log entry must have a message
        return Err(ParseError::InvalidFormat);
    }

    // Convert the level string to the LogLevel enum
    let level = string_to_log_level(level_str);


    // Create and return the ParsedLogEntry
    Ok(ParsedLogEntry {
        timestamp,
        level,
        message,
    })
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