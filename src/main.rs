mod log_data;

use std::collections::HashMap; // We need HashMap for the 'fields' part
use log_data::{ParsedLogEntry, LogLevel};

fn main() {
    let mut extra_fields = HashMap::new();
    extra_fields.insert("user_id".to_string(), "user123".to_string());


    let log_entry = ParsedLogEntry {
        timestamp: "2025-05-04T15:30:00Z".to_string(),
        level: LogLevel::Info, // Use the imported enum variant
        message: "User logged in successfully".to_string(),
        fields: extra_fields,
    };

    println!("Log Entry from main: {:?}", log_entry);
    println!("Level: {:?}", log_entry.level);
    // Accessing public fields directly:
    println!("Message: {}", log_entry.message);
}
