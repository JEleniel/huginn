# Requirement 3: Configuration Management

## Overview

Huginn must provide a flexible and secure configuration system that supports multiple configuration sources with clear precedence rules. Configuration should be loadable from files, environment variables, and command-line arguments while maintaining type safety and validation.

## User Story

As a system administrator, I need to configure Huginn's behavior through configuration files for standard deployments, environment variables for containerized deployments, and command-line arguments for ad-hoc scans. I want configuration to be validated early and provide clear error messages when misconfigured.

## Features

### 1. Configuration Structure (Priority: Critical)

A well-defined configuration structure that covers all aspects of scanner operation.

**Feature Detail:**

- **Core Configuration Fields**:
	- `targets`: List of hosts/IPs to scan (required)
	- `scan_types`: List of enabled scan types (default: ["ping"])
	- `port_ranges`: Port ranges for port scanning (default: common ports)
	- `timeout`: Global timeout in seconds (default: 30)
	- `concurrency`: Max concurrent scans (default: 10)
	- `log_level`: Logging verbosity (default: "info")
	- `output_format`: Result output format (default: "text")
	- `output_file`: Optional output file path
- **Plugin-Specific Configuration**: Nested configuration per plugin
- **Type Safety**: Use Rust types with serde for deserialization
- **Defaults**: All optional fields have sensible defaults

**Acceptance Criteria:**

- Configuration structure is documented with rustdoc
- All fields are strongly typed
- Default values are provided for optional fields
- Configuration is serializable and deserializable
- Field validation is performed during deserialization

### 2. Configuration Sources (Priority: Critical)

Support multiple configuration sources with defined precedence.

**Feature Detail:**

- **Configuration Priority** (highest to lowest):
	1. Command-line arguments
	2. Environment variables (prefix: `HUGINN_`)
	3. Configuration file (JSON/TOML)
	4. Built-in defaults
- **File Formats**: Support JSON and TOML configuration files
- **File Discovery**: Search for config in:
	1. Path specified by `--config` argument
	2. `./config.json` or `./config.toml`
	3. `~/.config/huginn/config.json`
	4. `/etc/huginn/config.json` (Unix systems)
- **Merging Strategy**: Higher priority sources override lower priority

**Acceptance Criteria:**

- Configuration loads from all supported sources
- Precedence rules are correctly applied
- Missing config files (if not required) don't cause errors
- JSON and TOML parsing errors provide clear messages
- Environment variable names follow consistent naming (HUGINN_FIELD_NAME)

### 3. Configuration Validation (Priority: High)

Comprehensive validation of configuration before scanner starts.

**Feature Detail:**

- **Field Validation**:
	- Targets: Non-empty list, valid IP/hostname format
	- Scan types: Recognized scan type names
	- Port ranges: Valid port numbers (1-65535)
	- Timeout: Positive value, reasonable range
	- Concurrency: Positive value, reasonable limit (1-1000)
	- Paths: Valid file paths, write permissions for output
- **Semantic Validation**:
	- Scan types match available plugins
	- Port ranges specified for port-based scans
	- Output directory exists and is writable
- **Validation Errors**: Clear, actionable error messages

**Acceptance Criteria:**

- Invalid configuration prevents scanner startup
- Validation errors specify field name and problem
- Validation errors suggest corrections
- All validation rules are documented
- Validation is tested with invalid inputs

### 4. Environment Variable Support (Priority: High)

Full support for configuration via environment variables.

**Feature Detail:**

- **Variable Naming**: Convert config fields to environment variables:
	- `targets` → `HUGINN_TARGETS` (comma-separated)
	- `scan_types` → `HUGINN_SCAN_TYPES` (comma-separated)
	- `log_level` → `HUGINN_LOG_LEVEL`
	- Nested: `plugin.tcp.timeout` → `HUGINN_PLUGIN_TCP_TIMEOUT`
- **Type Conversion**: Parse strings to appropriate types
- **List Handling**: Support comma-separated lists
- **Documentation**: Document all environment variables

**Acceptance Criteria:**

- All config fields can be set via environment variables
- Lists are properly parsed from comma-separated strings
- Type conversion errors provide clear messages
- Environment variable names are documented
- Empty environment variables are handled correctly

### 5. Configuration File Examples (Priority: Medium)

Provide comprehensive example configuration files.

**Feature Detail:**

- **Basic Example**: Minimal working configuration
- **Advanced Example**: All options with explanations
- **Use Case Examples**:
	- Single host quick scan
	- Network range comprehensive scan
	- Service-specific scanning
	- Daemon mode configuration
- **Comments**: Extensive comments explaining each option

**Acceptance Criteria:**

- At least 3 example configurations provided
- Examples are valid and tested
- Examples include inline comments
- Examples demonstrate common use cases
- Examples are kept up-to-date with code

### 6. Configuration Secrets Management (Priority: High)

Secure handling of sensitive configuration values.

**Feature Detail:**

- **Secret Fields**: Identify sensitive fields:
	- API keys
	- Authentication tokens
	- Passwords
- **Secret Storage**: Support for:
	- Environment variables (preferred)
	- Files with restricted permissions
	- System keychain/credential store (future)
- **Secret Masking**: Redact secrets in logs and error messages
- **Validation**: Check file permissions on secret files

**Acceptance Criteria:**

- Secrets can be loaded from environment variables
- Secrets are never logged or printed
- Secret files must have restrictive permissions (0600)
- Configuration validation checks secret availability
- Documentation warns about secret handling

### 7. Configuration Schema Documentation (Priority: Medium)

Comprehensive documentation of configuration schema.

**Feature Detail:**

- **Schema Documentation**: Document in README and docs:
	- All configuration fields
	- Field types and formats
	- Valid values and ranges
	- Default values
	- Required vs optional fields
	- Example values
- **JSON Schema**: Generate JSON schema for validation tools
- **Inline Help**: CLI help text includes configuration info

**Acceptance Criteria:**

- All config fields are documented
- Documentation includes examples
- JSON schema is valid and complete
- CLI help matches documentation
- Documentation is kept in sync with code

### 8. Configuration Override System (Priority: Medium)

Allow runtime configuration overrides for specific operations.

**Feature Detail:**

- **Override Mechanism**: Programmatic configuration changes
- **Scoped Overrides**: Override config for specific scan operations
- **Override Validation**: Validate overrides same as base config
- **Override Logging**: Log configuration overrides

**Acceptance Criteria:**

- Configuration can be cloned and modified
- Overrides don't affect original configuration
- Override validation uses same rules
- Overrides are clearly logged
- API is documented with examples

### 9. Configuration Migration (Priority: Low)

Support for migrating configurations between versions.

**Feature Detail:**

- **Version Tracking**: Include schema version in config
- **Migration Scripts**: Scripts to update old configs
- **Deprecation Warnings**: Warn about deprecated fields
- **Backward Compatibility**: Support old configs when possible

**Acceptance Criteria:**

- Configuration version is tracked
- Deprecated fields generate warnings
- Migration path is documented
- Old configs work or provide upgrade instructions

### 10. Dynamic Configuration Reload (Priority: Low)

Support for reloading configuration without restarting (daemon mode).

**Feature Detail:**

- **Reload Signal**: Respond to SIGHUP (Unix) or config API call
- **Hot Reload**: Reload config without stopping active scans
- **Validation**: Validate new config before applying
- **Rollback**: Revert to old config if validation fails

**Acceptance Criteria:**

- Config reload doesn't interrupt active scans
- Invalid new config doesn't break running scanner
- Reload is logged clearly
- Changed values take effect immediately for new scans
- API for triggering reload exists

## Dependencies

- `config` crate for configuration loading
- `serde` for serialization/deserialization
- `serde_json` for JSON support
- `toml` for TOML support (optional)
- File system access for config file reading

## Testing Requirements

- Unit tests for configuration loading from each source
- Unit tests for precedence rules
- Unit tests for validation (valid and invalid inputs)
- Unit tests for environment variable parsing
- Integration tests for configuration merging
- Tests for secret handling
- Tests for configuration errors

## Documentation Requirements

- Configuration reference document
- Example configuration files with comments
- Environment variable documentation
- Configuration migration guide
- Secret handling best practices
- Troubleshooting guide for config issues

## Related Requirements

- Requirement 1: Core Scanning System
- Requirement 2: Plugin Architecture
- Requirement 4: CLI Interface
- Requirement 5: Security & Privilege Management
