// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Huginn - Cyber Threat Scanning Toolkit
//!
//! The raven of Odin searches the world for knowledge and threats.
//! This is the main entry point for the Huginn executable.

mod config;
mod logging;
mod plugins;
mod scanner;

use log::{error, info};

#[tokio::main]
async fn main() {
	// Initialize logging
	if let Err(e) = logging::init() {
		eprintln!("Failed to initialize logging: {}", e);
		std::process::exit(1);
	}

	info!("Starting Huginn cyber threat scanning toolkit");

	// Load configuration
	let config = match config::load() {
		Ok(cfg) => cfg,
		Err(e) => {
			error!("Failed to load configuration: {}", e);
			std::process::exit(1);
		},
	};

	info!("Configuration loaded successfully");

	// Initialize scanner
	let scanner = scanner::Scanner::new(config);

	// Run the scanner
	if let Err(e) = scanner.run().await {
		error!("Scanner error: {}", e);
		std::process::exit(1);
	}

	info!("Huginn completed successfully");
}
