// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Configuration management for Huginn
//!
//! This module handles loading and parsing configuration from files, environment variables,
//! and command-line arguments with proper precedence: CLI > Env > File > Defaults.

use clap::Parser;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

/// Main configuration structure for Huginn
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
	/// Target hosts to scan
	pub targets: Vec<String>,
	/// Enabled scan types
	pub scan_types: Vec<String>,
	/// Optional server port for daemon mode
	pub port: Option<u16>,
	/// Output format: json, text, or csv
	pub output_format: String,
	/// Log level: debug, info, warn, error
	pub log_level: String,
	/// Optional output file path
	#[serde(skip)]
	pub output_file: Option<PathBuf>,
	/// Configuration file path
	#[serde(skip)]
	pub config_file: Option<PathBuf>,
	/// Enable verbose output
	#[serde(skip)]
	pub verbose: u8,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			targets: Vec::new(),
			scan_types: vec!["ping".to_string()],
			port: None,
			output_format: "text".to_string(),
			log_level: "info".to_string(),
			output_file: None,
			config_file: None,
			verbose: 0,
		}
	}
}

/// Command-line arguments for Huginn
#[derive(Parser, Debug)]
#[command(
	name = "huginn",
	version,
	about = "Huginn - Cyber Threat Scanning Toolkit\nThe raven of Odin searches the world for knowledge and threats.",
	long_about = None
)]
pub struct Cli {
	/// Subcommands
	#[command(subcommand)]
	pub command: Option<Commands>,

	/// Configuration file path
	#[arg(short, long, value_name = "FILE", env = "HUGINN_CONFIG")]
	pub config: Option<PathBuf>,

	/// Set log level
	#[arg(long, value_name = "LEVEL", default_value = "info")]
	pub log_level: Option<String>,

	/// Enable verbose output (can be used multiple times: -v, -vv, -vvv)
	#[arg(short, long, action = clap::ArgAction::Count)]
	pub verbose: u8,
}

/// Available subcommands
#[derive(Parser, Debug)]
pub enum Commands {
	/// Perform scanning operations
	Scan(ScanArgs),
	/// Show version information
	Version,
}

/// Arguments for the scan command
#[derive(Parser, Debug)]
pub struct ScanArgs {
	/// Target hosts to scan (IP addresses, hostnames, or CIDR ranges)
	#[arg(short = 't', long, value_name = "TARGET", required = true)]
	pub target: Vec<String>,

	/// Scan types to perform (comma-separated: ping, tcp_connect, tcp_syn, udp)
	#[arg(short = 's', long, value_name = "TYPE", default_value = "ping")]
	pub scan_type: String,

	/// Port specification (e.g., "80,443,8000-9000")
	#[arg(short, long, value_name = "PORTS")]
	pub ports: Option<String>,

	/// Output file path (stdout if not specified)
	#[arg(short, long, value_name = "FILE")]
	pub output: Option<PathBuf>,

	/// Output format: text, json, csv
	#[arg(short = 'f', long, value_name = "FORMAT", default_value = "text")]
	pub format: String,
}

/// Load configuration from file, environment variables, and CLI arguments
///
/// Configuration precedence (highest to lowest):
/// 1. Command-line arguments
/// 2. Environment variables (prefix: HUGINN_)
/// 3. Configuration file
/// 4. Default values
pub fn load(cli: &Cli) -> Result<Config, ConfigError> {
	// Start with default configuration
	let mut config = Config::default();

	// Determine config file path
	let config_file = cli
		.config
		.clone()
		.unwrap_or_else(|| PathBuf::from("config.json"));

	// Load from file and environment
	let builder = ConfigBuilder::builder()
		.add_source(File::from(config_file.clone()).required(false))
		.add_source(Environment::with_prefix("HUGINN").separator("__"))
		.set_default("targets", Vec::<String>::new())?
		.set_default("scan_types", vec!["ping"])?
		.set_default("output_format", "text")?
		.set_default("log_level", "info")?;

	if let Ok(built_config) = builder.build()
		&& let Ok(loaded_config) = built_config.try_deserialize::<Config>()
	{
		// Merge file/env config with defaults
		if !loaded_config.targets.is_empty() {
			config.targets = loaded_config.targets;
		}
		if !loaded_config.scan_types.is_empty() {
			config.scan_types = loaded_config.scan_types;
		}
		config.port = loaded_config.port;
		config.output_format = loaded_config.output_format;
		config.log_level = loaded_config.log_level;
	}

	// Apply CLI overrides
	if let Some(log_level) = &cli.log_level {
		config.log_level = log_level.clone();
	}
	config.config_file = Some(config_file);
	config.verbose = cli.verbose;

	// Apply scan command arguments if present
	if let Some(Commands::Scan(scan_args)) = &cli.command {
		config.targets = scan_args.target.clone();
		config.scan_types = scan_args
			.scan_type
			.split(',')
			.map(|s| s.trim().to_string())
			.collect();
		config.output_format = scan_args.format.clone();
		config.output_file = scan_args.output.clone();
	}

	// Validate configuration
	validate(&config)?;

	Ok(config)
}

/// Validate configuration for semantic correctness
fn validate(config: &Config) -> Result<(), ConfigError> {
	// Validate targets are non-empty when scanning
	if config.targets.is_empty() {
		return Err(ConfigError::Message(
			"At least one target must be specified".to_string(),
		));
	}

	// Validate scan types
	let valid_scan_types = ["ping", "tcp_connect", "tcp_syn", "udp"];
	for scan_type in &config.scan_types {
		if !valid_scan_types.contains(&scan_type.as_str()) {
			return Err(ConfigError::Message(format!(
				"Invalid scan type '{}'. Valid types: {}",
				scan_type,
				valid_scan_types.join(", ")
			)));
		}
	}

	// Validate output format
	let valid_formats = ["text", "json", "csv"];
	if !valid_formats.contains(&config.output_format.as_str()) {
		return Err(ConfigError::Message(format!(
			"Invalid output format '{}'. Valid formats: {}",
			config.output_format,
			valid_formats.join(", ")
		)));
	}

	// Validate log level
	let valid_levels = ["debug", "info", "warn", "error"];
	if !valid_levels.contains(&config.log_level.as_str()) {
		return Err(ConfigError::Message(format!(
			"Invalid log level '{}'. Valid levels: {}",
			config.log_level,
			valid_levels.join(", ")
		)));
	}

	// Validate port if specified
	if let Some(port) = config.port
		&& port == 0
	{
		return Err(ConfigError::Message(
			"Port must be between 1 and 65535".to_string(),
		));
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_default_config() {
		let config = Config::default();
		assert_eq!(config.targets.len(), 0);
		assert_eq!(config.scan_types, vec!["ping".to_string()]);
		assert_eq!(config.output_format, "text");
		assert_eq!(config.log_level, "info");
		assert_eq!(config.verbose, 0);
	}

	#[test]
	fn test_validate_empty_targets() {
		let config = Config::default();
		let result = validate(&config);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("At least one target")
		);
	}

	#[test]
	fn test_validate_invalid_scan_type() {
		let mut config = Config::default();
		config.targets = vec!["192.168.1.1".to_string()];
		config.scan_types = vec!["invalid_scan".to_string()];
		let result = validate(&config);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("Invalid scan type")
		);
	}

	#[test]
	fn test_validate_valid_config() {
		let mut config = Config::default();
		config.targets = vec!["192.168.1.1".to_string()];
		config.scan_types = vec!["ping".to_string(), "tcp_connect".to_string()];
		let result = validate(&config);
		assert!(result.is_ok());
	}

	#[test]
	fn test_validate_invalid_output_format() {
		let mut config = Config::default();
		config.targets = vec!["192.168.1.1".to_string()];
		config.output_format = "invalid".to_string();
		let result = validate(&config);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("Invalid output format")
		);
	}

	#[test]
	fn test_validate_invalid_log_level() {
		let mut config = Config::default();
		config.targets = vec!["192.168.1.1".to_string()];
		config.log_level = "invalid".to_string();
		let result = validate(&config);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("Invalid log level")
		);
	}

	#[test]
	fn test_validate_invalid_port() {
		let mut config = Config::default();
		config.targets = vec!["192.168.1.1".to_string()];
		config.port = Some(0);
		let result = validate(&config);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("Port must be between")
		);
	}
}
