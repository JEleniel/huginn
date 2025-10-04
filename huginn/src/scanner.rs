// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Scanner core functionality
//!
//! This module implements the main scanner logic that orchestrates different scan types.

use crate::config::Config;
use crate::plugins::{Plugin, ScanResult};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn};
use std::error::Error;

/// Main scanner structure
pub struct Scanner {
	config: Config,
	plugins: Vec<Box<dyn Plugin>>,
}

impl Scanner {
	/// Create a new scanner instance
	pub fn new(config: Config) -> Self {
		Self {
			config,
			plugins: Vec::new(),
		}
	}

	/// Register a plugin
	#[allow(dead_code)]
	pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
		info!("Registering plugin: {}", plugin.name());
		self.plugins.push(plugin);
	}

	/// Run all configured scans and return results
	pub async fn run(&self) -> Result<Vec<ScanResult>, Box<dyn Error>> {
		info!("Starting scan execution");

		let mut all_results = Vec::new();

		if self.config.targets.is_empty() {
			warn!("No targets configured for scanning");
			return Ok(all_results);
		}

		// Calculate total operations for progress bar
		let total_operations = self.config.targets.len()
			* self
				.plugins
				.iter()
				.filter(|p| self.config.scan_types.contains(&p.scan_type()))
				.count();

		// Create progress bar if we have operations to perform
		let progress_bar = if total_operations > 0 && self.config.verbose == 0 {
			let pb = ProgressBar::new(total_operations as u64);
			pb.set_style(
				ProgressStyle::default_bar()
					.template(
						"{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
					)?
					.progress_chars("#>-"),
			);
			Some(pb)
		} else {
			None
		};

		for target in &self.config.targets {
			info!("Scanning target: {}", target);

			for plugin in &self.plugins {
				if self.config.scan_types.contains(&plugin.scan_type()) {
					if let Some(pb) = &progress_bar {
						pb.set_message(format!("{} on {}", plugin.scan_type(), target));
					}

					info!("Running {} scan on {}", plugin.scan_type(), target);
					match plugin.scan(target).await {
						Ok(results) => {
							info!("Scan completed: {} results found", results.len());
							all_results.extend(results);
						}
						Err(e) => {
							error!("Scan failed: {}", e);
						}
					}

					if let Some(pb) = &progress_bar {
						pb.inc(1);
					}
				}
			}
		}

		if let Some(pb) = progress_bar {
			pb.finish_with_message("Scan completed");
		}

		info!("Scan execution completed");
		Ok(all_results)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_scanner_creation() {
		let config = Config::default();
		let scanner = Scanner::new(config);
		assert_eq!(scanner.plugins.len(), 0);
	}
}
