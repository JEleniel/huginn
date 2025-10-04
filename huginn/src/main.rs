// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Huginn - Cyber Threat Scanning Toolkit
//!
//! The raven of Odin searches the world for knowledge and threats.
//! This is the main entry point for the Huginn executable.

mod config;
mod formatters;
mod logging;
mod plugins;
mod scanner;

use clap::Parser;
use config::{Cli, Commands};
use log::{error, info};

#[tokio::main]
async fn main() {
	// Parse command-line arguments
	let cli = Cli::parse();

	// Handle version command
	if let Some(Commands::Version) = cli.command {
		println!("Huginn version {}", env!("CARGO_PKG_VERSION"));
		return;
	}

	// Initialize logging
	if let Err(e) = logging::init() {
		eprintln!("Failed to initialize logging: {}", e);
		std::process::exit(1);
	}

	info!("Starting Huginn cyber threat scanning toolkit");

	// Load configuration with CLI arguments
	let config = match config::load(&cli) {
		Ok(cfg) => cfg,
		Err(e) => {
			error!("Failed to load configuration: {}", e);
			eprintln!("Error: {}", e);
			std::process::exit(1);
		}
	};

	info!("Configuration loaded successfully");
	info!("Targets: {:?}", config.targets);
	info!("Scan types: {:?}", config.scan_types);

	// Initialize scanner
	let scanner = scanner::Scanner::new(config.clone());

	// Run the scanner
	let results = match scanner.run().await {
		Ok(res) => res,
		Err(e) => {
			error!("Scanner error: {}", e);
			std::process::exit(1);
		}
	};

	info!("Scan completed with {} results", results.len());

	// Format and output results
	let colored_output = config.output_file.is_none() && atty::is(atty::Stream::Stdout);
	let formatter = formatters::get_formatter(&config.output_format, colored_output);

	let formatted_output = match formatter.format(&results) {
		Ok(output) => output,
		Err(e) => {
			error!("Failed to format output: {}", e);
			std::process::exit(1);
		}
	};

	// Write to file or stdout
	if let Some(output_path) = &config.output_file {
		match std::fs::write(output_path, &formatted_output) {
			Ok(_) => {
				info!("Results written to {}", output_path.display());
				println!("Results written to {}", output_path.display());
			}
			Err(e) => {
				error!("Failed to write output file: {}", e);
				eprintln!("Error writing output file: {}", e);
				std::process::exit(1);
			}
		}
	} else {
		println!("{}", formatted_output);
	}

	info!("Huginn completed successfully");
}
