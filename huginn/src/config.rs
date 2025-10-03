// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Configuration management for Huginn
//!
//! This module handles loading and parsing configuration from files and environment variables.

use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

/// Main configuration structure for Huginn
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
	/// API key for authentication
	pub api_key: Option<String>,
	/// Enable debug mode
	pub debug_mode: bool,
	/// Server port number
	pub port: u16,
	/// Target hosts to scan
	pub targets: Vec<String>,
	/// Enabled scan types
	pub scan_types: Vec<String>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			api_key: None,
			debug_mode: false,
			port: 3000,
			targets: Vec::new(),
			scan_types: vec!["ping".to_string()],
		}
	}
}

/// Load configuration from file and environment variables
pub fn load() -> Result<Config, ConfigError> {
	let config_file = PathBuf::from("config.json");

	let builder = ConfigBuilder::builder()
		.add_source(File::from(config_file).required(false))
		.add_source(Environment::with_prefix("HUGINN"))
		.set_default("debug_mode", false)?
		.set_default("port", 3000)?;

	let config = builder.build()?;
	config.try_deserialize()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_default_config() {
		let config = Config::default();
		assert!(!config.debug_mode);
		assert_eq!(config.port, 3000);
		assert!(config.api_key.is_none());
	}
}
