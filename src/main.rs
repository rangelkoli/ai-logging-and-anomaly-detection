mod log_parser;
mod log_data;

use crate::log_data::{ParsedLogEntry};
use crate::log_parser::parse_line;
use std::fs::read_to_string;
fn main() {
let log_file = "test.log";
let contents = read_to_string(log_file);
let mut parsed_entries: Vec<ParsedLogEntry> = Vec::new();

match contents {
    Ok(data) => {
        for line in data.lines() {
            // Here you would call your log parsing function
            // For example: parse_line(line);
            match parse_line(line) {
                Ok(parsed_entry) => {
                    parsed_entries.push(parsed_entry);
                },
                Err(e) => {
                    eprintln!("Error parsing line '{}': {:?}", line, e);
                }
            }

        }
    },
    Err(e) => {
        eprintln!("Error reading file {}: {}", log_file, e);
    }
}

println!("Parsed {} log entries.", parsed_entries.len());
println!("Parsed entries: {:?}", parsed_entries);

}
