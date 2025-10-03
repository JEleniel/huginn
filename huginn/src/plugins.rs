// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Plugin system for Huginn
//!
//! This module defines the plugin trait and provides a framework for implementing
//! different types of scanning plugins.

pub mod ping;
pub mod tcp_connect;
pub mod tcp_syn;
pub mod udp;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Scan result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
	/// Target that was scanned
	pub target: String,
	/// Type of scan performed
	pub scan_type: String,
	/// Status or result of the scan
	pub status: String,
	/// Additional details
	pub details: Option<String>,
}

/// Plugin trait that all scanning plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
	/// Get the name of the plugin
	fn name(&self) -> String;

	/// Get the scan type identifier
	fn scan_type(&self) -> String;

	/// Perform the scan on the target
	async fn scan(&self, target: &str) -> Result<Vec<ScanResult>, Box<dyn Error>>;
}
