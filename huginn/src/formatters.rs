// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Output formatting for scan results
//!
//! This module provides different output formats for scan results including
//! text, JSON, and CSV formats.

use crate::plugins::ScanResult;
use colored::*;
use serde::Serialize;
use std::error::Error;

/// Output formatter trait
pub trait OutputFormatter {
	/// Format scan results as a string
	fn format(&self, results: &[ScanResult]) -> Result<String, Box<dyn Error>>;
}

/// Plain text formatter with human-readable output
pub struct TextFormatter {
	colored: bool,
}

impl TextFormatter {
	/// Create a new text formatter
	pub fn new(colored: bool) -> Self {
		Self { colored }
	}
}

impl OutputFormatter for TextFormatter {
	fn format(&self, results: &[ScanResult]) -> Result<String, Box<dyn Error>> {
		let mut output = String::new();

		if results.is_empty() {
			output.push_str("No scan results\n");
			return Ok(output);
		}

		// Summary header
		if self.colored {
			output.push_str(&format!("{}\n", "Scan Results".bold().underline()));
			output.push_str(&format!(
				"{} {}\n\n",
				"Total results:".bold(),
				results.len()
			));
		} else {
			output.push_str("Scan Results\n");
			output.push_str(&format!("Total results: {}\n\n", results.len()));
		}

		// Results by target
		let mut current_target = String::new();
		for result in results {
			if result.target != current_target {
				current_target = result.target.clone();
				if self.colored {
					output.push_str(&format!("\n{} {}\n", "Target:".bold(), current_target));
				} else {
					output.push_str(&format!("\nTarget: {}\n", current_target));
				}
			}

			// Format status with color
			let status_str = if self.colored {
				match result.status.as_str() {
					"open" | "up" | "alive" => result.status.green().to_string(),
					"closed" | "down" | "dead" => result.status.red().to_string(),
					"filtered" => result.status.yellow().to_string(),
					_ => result.status.normal().to_string(),
				}
			} else {
				result.status.clone()
			};

			output.push_str(&format!(
				"  [{}] {}\n",
				result.scan_type, status_str
			));

			if let Some(details) = &result.details {
				output.push_str(&format!("    {}\n", details));
			}
		}

		Ok(output)
	}
}

/// JSON formatter for machine-readable output
pub struct JsonFormatter {
	pretty: bool,
}

impl JsonFormatter {
	/// Create a new JSON formatter
	pub fn new(pretty: bool) -> Self {
		Self { pretty }
	}
}

/// Wrapper for JSON output with metadata
#[derive(Serialize)]
struct JsonOutput<'a> {
	total_results: usize,
	results: &'a [ScanResult],
}

impl OutputFormatter for JsonFormatter {
	fn format(&self, results: &[ScanResult]) -> Result<String, Box<dyn Error>> {
		let output = JsonOutput {
			total_results: results.len(),
			results,
		};

		let json = if self.pretty {
			serde_json::to_string_pretty(&output)?
		} else {
			serde_json::to_string(&output)?
		};

		Ok(json)
	}
}

/// CSV formatter for spreadsheet-compatible output
pub struct CsvFormatter;

impl CsvFormatter {
	/// Create a new CSV formatter
	pub fn new() -> Self {
		Self
	}
}

impl Default for CsvFormatter {
	fn default() -> Self {
		Self::new()
	}
}

impl OutputFormatter for CsvFormatter {
	fn format(&self, results: &[ScanResult]) -> Result<String, Box<dyn Error>> {
		let mut output = String::new();

		// CSV header
		output.push_str("target,scan_type,status,details\n");

		// CSV rows
		for result in results {
			let details = result
				.details
				.as_ref()
				.map(|d| d.replace(',', ";").replace('\n', " "))
				.unwrap_or_default();

			output.push_str(&format!(
				"{},{},{},{}\n",
				result.target, result.scan_type, result.status, details
			));
		}

		Ok(output)
	}
}

/// Get the appropriate formatter based on format string
pub fn get_formatter(format: &str, colored: bool) -> Box<dyn OutputFormatter> {
	match format {
		"json" => Box::new(JsonFormatter::new(true)),
		"csv" => Box::new(CsvFormatter::new()),
		_ => Box::new(TextFormatter::new(colored)),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_test_results() -> Vec<ScanResult> {
		vec![
			ScanResult {
				target: "192.168.1.1".to_string(),
				scan_type: "ping".to_string(),
				status: "up".to_string(),
				details: Some("latency: 5ms".to_string()),
			},
			ScanResult {
				target: "192.168.1.1".to_string(),
				scan_type: "tcp_connect".to_string(),
				status: "open".to_string(),
				details: Some("port 80".to_string()),
			},
			ScanResult {
				target: "192.168.1.2".to_string(),
				scan_type: "ping".to_string(),
				status: "down".to_string(),
				details: None,
			},
		]
	}

	#[test]
	fn test_text_formatter() {
		let formatter = TextFormatter::new(false);
		let results = create_test_results();
		let output = formatter.format(&results).unwrap();

		assert!(output.contains("Scan Results"));
		assert!(output.contains("Total results: 3"));
		assert!(output.contains("192.168.1.1"));
		assert!(output.contains("192.168.1.2"));
		assert!(output.contains("ping"));
		assert!(output.contains("tcp_connect"));
	}

	#[test]
	fn test_text_formatter_empty() {
		let formatter = TextFormatter::new(false);
		let results = vec![];
		let output = formatter.format(&results).unwrap();

		assert!(output.contains("No scan results"));
	}

	#[test]
	fn test_json_formatter() {
		let formatter = JsonFormatter::new(true);
		let results = create_test_results();
		let output = formatter.format(&results).unwrap();

		assert!(output.contains("\"total_results\": 3"));
		assert!(output.contains("\"target\": \"192.168.1.1\""));
		assert!(output.contains("\"scan_type\": \"ping\""));
		assert!(output.contains("\"status\": \"up\""));
	}

	#[test]
	fn test_csv_formatter() {
		let formatter = CsvFormatter::new();
		let results = create_test_results();
		let output = formatter.format(&results).unwrap();

		assert!(output.contains("target,scan_type,status,details"));
		assert!(output.contains("192.168.1.1,ping,up,latency: 5ms"));
		assert!(output.contains("192.168.1.2,ping,down,"));
	}

	#[test]
	fn test_get_formatter() {
		let _ = get_formatter("text", false);
		let _ = get_formatter("json", false);
		let _ = get_formatter("csv", false);
		let _ = get_formatter("unknown", false);
	}

	#[test]
	fn test_csv_escape_commas() {
		let formatter = CsvFormatter::new();
		let results = vec![ScanResult {
			target: "test.com".to_string(),
			scan_type: "test".to_string(),
			status: "ok".to_string(),
			details: Some("data,with,commas".to_string()),
		}];
		let output = formatter.format(&results).unwrap();

		// Commas should be replaced with semicolons
		assert!(output.contains("data;with;commas"));
	}
}
