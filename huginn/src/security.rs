// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Security utilities for Huginn
//!
//! This module provides security-related functionality including secret masking
//! and file permission validation.

use log::warn;
use std::path::Path;

/// Mask sensitive values in strings for logging
///
/// This function replaces sensitive patterns with masked versions to prevent
/// secrets from being logged in clear text.
pub fn mask_sensitive(input: &str) -> String {
	let mut masked = input.to_string();

	// Mask common secret patterns
	// API keys and tokens (typically alphanumeric strings)
	let api_key_patterns = [
		r#"api[_-]?key[=:]\s*['"]?([a-zA-Z0-9_-]{8,})['"]?"#,
		r#"token[=:]\s*['"]?([a-zA-Z0-9_-]{8,})['"]?"#,
		r#"secret[=:]\s*['"]?([a-zA-Z0-9_-]{8,})['"]?"#,
		r#"password[=:]\s*['"]?([^\s'"]{8,})['"]?"#,
	];

	for pattern in &api_key_patterns {
		if let Ok(re) = regex::Regex::new(pattern) {
			masked = re
				.replace_all(&masked, |caps: &regex::Captures| {
					let key_name = caps.get(0).unwrap().as_str().split(['=', ':']).next().unwrap();
					format!("{}=***REDACTED***", key_name)
				})
				.to_string();
		}
	}

	masked
}

/// Check file permissions and warn if overly permissive
///
/// This function checks if a file has secure permissions (0600 on Unix).
/// Returns true if permissions are secure, false otherwise.
#[cfg(unix)]
pub fn check_file_permissions(path: &Path) -> bool {
	use std::os::unix::fs::PermissionsExt;

	if let Ok(metadata) = std::fs::metadata(path) {
		let permissions = metadata.permissions();
		let mode = permissions.mode();

		// Check if file is readable/writable by owner only (0600)
		let secure_mode = 0o600;
		let actual_mode = mode & 0o777;

		if actual_mode != secure_mode {
			warn!(
				"File {} has permissive permissions: {:o} (expected: {:o})",
				path.display(),
				actual_mode,
				secure_mode
			);
			return false;
		}
		true
	} else {
		// If we can't read metadata, assume it's not secure
		false
	}
}

/// Check file permissions (Windows - always returns true as Windows uses different security model)
#[cfg(not(unix))]
pub fn check_file_permissions(_path: &Path) -> bool {
	// On Windows, we rely on NTFS permissions which are more complex
	// For now, we assume files are secure and rely on Windows security
	true
}

/// Validate that a configuration file has secure permissions
///
/// Issues warnings if the file has overly permissive permissions.
pub fn validate_config_file_security(path: &Path) {
	if !check_file_permissions(path) {
		warn!(
			"Configuration file {} may contain sensitive data but has permissive permissions. \
			 Consider restricting access with: chmod 600 {}",
			path.display(),
			path.display()
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mask_sensitive_api_key() {
		let input = "api_key=sk_test_abcdef1234567890";
		let masked = mask_sensitive(input);
		assert!(!masked.contains("sk_test_abcdef1234567890"));
		assert!(masked.contains("***REDACTED***"));
	}

	#[test]
	fn test_mask_sensitive_token() {
		let input = "token: ghp_1234567890abcdefghij";
		let masked = mask_sensitive(input);
		assert!(!masked.contains("ghp_1234567890abcdefghij"));
		assert!(masked.contains("***REDACTED***"));
	}

	#[test]
	fn test_mask_sensitive_password() {
		let input = "password=mySecretPass123";
		let masked = mask_sensitive(input);
		assert!(!masked.contains("mySecretPass123"));
		assert!(masked.contains("***REDACTED***"));
	}

	#[test]
	fn test_mask_sensitive_no_secrets() {
		let input = "This is just normal text";
		let masked = mask_sensitive(input);
		assert_eq!(input, masked);
	}

	#[test]
	fn test_mask_sensitive_multiple_secrets() {
		let input = "api_key=secret123 token=token456";
		let masked = mask_sensitive(input);
		assert!(!masked.contains("secret123"));
		assert!(!masked.contains("token456"));
		assert!(masked.contains("***REDACTED***"));
	}

	#[cfg(unix)]
	#[test]
	fn test_check_file_permissions_nonexistent() {
		let result = check_file_permissions(Path::new("/nonexistent/file"));
		assert!(!result);
	}
}
