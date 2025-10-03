// Copyright (c) 2025 JEleniel
// Licensed under the Apache License, Version 2.0 or the MIT License

//! Logging configuration for Huginn
//!
//! This module sets up structured logging using the fern crate.

use log::LevelFilter;
use std::io;

/// Initialize the logging system
pub fn init() -> Result<(), fern::InitError> {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"[{} {} {}] {}",
				chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
				record.level(),
				record.target(),
				message
			))
		})
		.level(LevelFilter::Info)
		.level_for("huginn", LevelFilter::Debug)
		.chain(io::stdout())
		.apply()?;

	Ok(())
}
