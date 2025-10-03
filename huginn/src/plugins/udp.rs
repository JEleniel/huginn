// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! UDP scan plugin
//!
//! Scans for open UDP ports, which do not require a handshake.

use crate::plugins::{Plugin, ScanResult};
use async_trait::async_trait;
use std::error::Error;

/// UDP scan plugin
pub struct UdpScanPlugin;

#[async_trait]
impl Plugin for UdpScanPlugin {
	fn name(&self) -> String {
		"UDP Scanner".to_string()
	}

	fn scan_type(&self) -> String {
		"udp".to_string()
	}

	async fn scan(&self, target: &str) -> Result<Vec<ScanResult>, Box<dyn Error>> {
		// Placeholder implementation
		Ok(vec![ScanResult {
			target: target.to_string(),
			scan_type: self.scan_type(),
			status: "not_implemented".to_string(),
			details: Some("UDP scan not yet implemented".to_string()),
		}])
	}
}
