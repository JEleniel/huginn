# Huginn Implementation Plan

## Overview

This document outlines the implementation plan for Huginn, a cyber threat scanning toolkit written in Rust. The project follows a modular, plugin-based architecture that allows for easy extension with new scanning techniques.

## Project Structure

```text
huginn/
├── Cargo.toml              # Workspace root configuration
├── rustfmt.toml            # Rust formatting configuration
├── huginn/                 # Main executable package
│   ├── Cargo.toml          # Package configuration
│   └── src/
│       ├── main.rs         # Application entry point
│       ├── config.rs       # Configuration management
│       ├── logging.rs      # Logging initialization
│       ├── scanner.rs      # Core scanner orchestration
│       └── plugins/        # Plugin implementations
│           ├── mod.rs      # Plugin trait and system
│           ├── ping.rs     # Ping scan plugin
│           ├── tcp_connect.rs  # TCP Connect scan
│           ├── tcp_syn.rs      # TCP SYN scan
│           └── udp.rs          # UDP scan plugin
```

## Technology Stack

### Core Dependencies

| Crate | Version | Purpose |
| --- | --- | --- |
| `tokio` | 1.42 | Async runtime for concurrent operations |
| `config` | 0.14 | Configuration management from files and environment |
| `serde` | 1.0 | Serialization/deserialization framework |
| `serde_json` | 1.0 | JSON support for configuration and output |
| `log` | 0.4 | Logging facade |
| `fern` | 0.7 | Structured logging implementation |
| `chrono` | 0.4 | Date and time handling for logs |
| `async-trait` | 0.1 | Async trait support for plugin system |

### Rust Edition

The project uses **Rust 2024 edition** with strict safety requirements:

- `unsafe_code = "forbid"` - No unsafe code allowed
- Dual-licensed under MIT OR Apache-2.0

## Implementation Phases

### Phase 1: Core Infrastructure ✅ (Complete)

**Objectives:**

- [x] Set up Rust 2024 workspace
- [x] Create main executable package structure
- [x] Configure dependencies
- [x] Implement basic module structure
- [x] Set up rustfmt configuration
- [x] Add Rust-specific .gitignore entries

**Deliverables:**

- Working Cargo workspace that compiles without errors
- Module structure with proper separation of concerns
- Rustfmt configuration following project standards

### Phase 2: Configuration System ✅ (Complete)

**Objectives:**

- [x] Implement configuration loading from files
- [x] Support environment variable overrides
- [x] Add command-line argument parsing with `clap`
- [x] Create example configuration files
- [x] Document all configuration options

**Implementation Details:**

1. **Configuration Sources (Priority Order):**
	- Command-line arguments (highest priority)
	- Environment variables (prefix: `HUGINN_`)
	- Configuration file (`config.json` or specified path)
	- Default values (lowest priority)

2. **Required Configuration Fields:**
	- `targets`: List of hosts/IPs to scan
	- `scan_types`: List of enabled scan types
	- `port`: Optional server port for daemon mode
	- `output_format`: JSON, text, or structured output
	- `log_level`: Debug, info, warn, error

3. **Files to Create:**
	- `config.example.json` - Example configuration
	- `README.md` section on configuration

### Phase 3: Logging System ✅ (Complete)

**Objectives:**

- [x] Set up structured logging with fern
- [x] Configure log levels and output format
- [ ] Add file logging support
- [ ] Implement log rotation

**Implementation Details:**

1. **Log Levels:**
	- ERROR: Critical failures
	- WARN: Non-critical issues
	- INFO: Progress and results
	- DEBUG: Detailed execution flow

2. **Log Format:**
	```text
	[YYYY-MM-DD HH:MM:SS LEVEL module] message
	```

3. **Future Enhancements:**
	- Optional JSON structured logging
	- Separate log files per scan
	- Log rotation based on size/date

### Phase 4: Scanner Core 🔄 (Partial)

**Objectives:**

- [x] Implement Scanner structure
- [x] Create plugin registration system
- [ ] Add result aggregation
- [ ] Implement concurrent scanning with tokio
- [ ] Add progress reporting
- [ ] Implement error handling and recovery

**Implementation Details:**

1. **Scanner Architecture:**
	```rust
	Scanner::new(config)
		.register_plugin(PingScanPlugin)
		.register_plugin(TcpConnectScanPlugin)
		.run().await
	```

2. **Concurrency Strategy:**
	- Use tokio tasks for parallel target scanning
	- Implement rate limiting to prevent network flooding
	- Add configurable timeout per scan type
	- Handle partial failures gracefully

3. **Result Collection:**
	- Aggregate results from all plugins
	- Provide unified result format
	- Support multiple output formats (JSON, text, CSV)

### Phase 5: Plugin System ✅ (Structure Complete)

**Objectives:**

- [x] Define Plugin trait with async support
- [x] Implement basic plugin structure
- [ ] Add plugin discovery mechanism
- [ ] Support dynamic plugin loading (future)

**Plugin Trait Design:**

```rust
#[async_trait]
pub trait Plugin: Send + Sync {
	fn name(&self) -> String;
	fn scan_type(&self) -> String;
	async fn scan(&self, target: &str) -> Result<Vec<ScanResult>, Box<dyn Error>>;
}
```

**ScanResult Structure:**

```rust
pub struct ScanResult {
	pub target: String,
	pub scan_type: String,
	pub status: String,
	pub details: Option<String>,
}
```

### Phase 6: Implement Scan Plugins 🔄 (Stubs Complete)

#### 6.1 Ping Scan ⏳

**Purpose:** Check host availability using ICMP echo requests

**Implementation Steps:**

1. Add `surge-ping` or equivalent crate for ICMP
2. Implement async ICMP echo request
3. Handle timeout and retry logic
4. Parse ICMP responses
5. Return host status (up/down) with latency

**Notes:**

- May require elevated privileges (CAP_NET_RAW on Linux)
- Consider fallback to TCP ping if ICMP unavailable

#### 6.2 TCP Connect Scan ⏳

**Purpose:** Full TCP connection to determine port status

**Implementation Steps:**

1. Use tokio's `TcpStream::connect()`
2. Implement port range scanning
3. Configure connection timeout
4. Categorize results: open, closed, filtered
5. Add service banner grabbing (optional)

**Configuration:**

- Port range: single port, list, or range (e.g., "80,443,8000-9000")
- Timeout per connection: configurable (default 5s)

#### 6.3 TCP SYN Scan ⏳

**Purpose:** Stealth scan using SYN packets

**Implementation Steps:**

1. Add raw socket support with `socket2` or `pnet`
2. Craft TCP SYN packets
3. Listen for SYN-ACK responses
4. Send RST to avoid full connection
5. Categorize port states

**Notes:**

- Requires root/admin privileges
- More complex packet handling
- Implement proper error handling for permission issues

#### 6.4 UDP Scan ⏳

**Purpose:** Scan UDP ports

**Implementation Steps:**

1. Use tokio's `UdpSocket`
2. Send UDP packets to target ports
3. Wait for ICMP port unreachable responses
4. Handle timeout for open|filtered ports
5. Implement protocol-specific probes

**Challenges:**

- UDP is stateless, harder to determine state
- Requires ICMP error message interpretation
- Slower due to timeout requirements

#### 6.5 Additional Plugins (Future)

**Version Detection:**

- Banner grabbing
- Service fingerprinting
- Protocol detection

**OS Detection:**

- TCP/IP stack fingerprinting
- TTL analysis
- Window size patterns

**Service Detection:**

- HTTP/HTTPS probing
- SSH version detection
- Database service identification

### Phase 7: Command-Line Interface ⏳

**Objectives:**

- [ ] Implement CLI argument parsing with `clap`
- [ ] Add subcommands for different modes
- [ ] Create help text and examples
- [ ] Support both standalone and daemon modes

**Proposed CLI:**

```bash
# Basic scan
huginn scan --target 192.168.1.1 --scan-type ping,tcp_connect

# Multiple targets from file
huginn scan --target-file targets.txt --all-scans

# Daemon mode
huginn daemon --config daemon-config.json

# Version and help
huginn --version
huginn --help
```

**CLI Structure:**

```text
huginn [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -v, --verbose         Enable verbose output
    -c, --config <FILE>   Configuration file path
    --log-level <LEVEL>   Set log level

SUBCOMMANDS:
    scan      Perform scanning operations
    daemon    Run as a service
    version   Show version information
```

### Phase 8: Testing Infrastructure 🔄

**Current Status:**

- [x] Basic unit tests for config module
- [x] Basic unit tests for scanner module
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Security testing

**Testing Strategy:**

1. **Unit Tests:**
	- Test each module in isolation
	- Mock external dependencies
	- Test error conditions
	- Target 80%+ code coverage

2. **Integration Tests:**
	- End-to-end scanning scenarios
	- Configuration loading from various sources
	- Plugin interaction with scanner core
	- Output format validation

3. **Test Infrastructure:**
	```text
	tests/
	├── integration/
	│   ├── scan_tests.rs
	│   ├── config_tests.rs
	│   └── plugin_tests.rs
	├── fixtures/
	│   ├── test-config.json
	│   └── test-targets.txt
	└── mocks/
	    └── network_mock.rs
	```

4. **Performance Tests:**
	- Benchmark scan speeds
	- Memory usage profiling
	- Concurrent scan limits
	- Network resource utilization

### Phase 9: Documentation 📝

**Required Documentation:**

1. **User Documentation:**
	- [ ] Installation guide
	- [ ] Configuration reference
	- [ ] Usage examples
	- [ ] Troubleshooting guide
	- [ ] FAQ

2. **Developer Documentation:**
	- [ ] Architecture overview
	- [ ] Plugin development guide
	- [ ] API documentation (rustdoc)
	- [ ] Contribution guidelines

3. **API Documentation:**
	- Generate with `cargo doc`
	- Ensure all public APIs are documented
	- Include examples in doc comments
	- Link between related items

4. **README Updates:**
	- Add build instructions
	- Add testing instructions
	- Update installation section
	- Add usage examples

### Phase 10: Daemon Mode (Future) 🔮

**Objectives:**

- [ ] Implement continuous scanning mode
- [ ] Add scheduling system
- [ ] Create REST API for control
- [ ] Implement result storage
- [ ] Add alerting system

**Architecture:**

```text
Daemon Components:
- HTTP API (using axum or actix-web)
- Scheduler (using tokio-cron-scheduler)
- Result storage (SQLite or PostgreSQL)
- Alert system (webhook, email)
```

### Phase 11: Security Hardening 🔒

**Security Checklist:**

- [ ] Input validation for all user inputs
- [ ] Rate limiting to prevent abuse
- [ ] Proper error handling without information leakage
- [ ] Audit dependencies with `cargo audit`
- [ ] OWASP ASVS compliance review
- [ ] Fuzzing tests with `cargo-fuzz`
- [ ] Security advisory process

**Privilege Management:**

- Document required privileges for each scan type
- Implement privilege dropping after initialization
- Provide warnings for operations requiring elevation
- Support capability-based permissions on Linux

## Development Workflow

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Formatting and Linting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run clippy linter
cargo clippy -- -D warnings

# Fix clippy suggestions
cargo clippy --fix
```

### Security Auditing

```bash
# Install cargo-audit
cargo install cargo-audit

# Run security audit
cargo audit

# Check for outdated dependencies
cargo outdated
```

## Code Standards

### Rust-Specific Standards

1. **Safety:**
	- No unsafe code (enforced by `#[forbid(unsafe_code)]`)
	- Handle all errors explicitly
	- Use Result types, avoid unwrap() in production code
	- Prefer ? operator for error propagation

2. **Async Patterns:**
	- Use tokio for async runtime
	- Prefer async/await over manual futures
	- Use tokio channels for message passing
	- Implement proper cancellation handling

3. **Error Handling:**
	- Use `thiserror` for custom error types
	- Provide context with error messages
	- Log errors before returning them
	- Don't expose internal implementation details in errors

4. **Documentation:**
	- Document all public APIs with rustdoc
	- Include usage examples in doc comments
	- Document panics, errors, and safety requirements
	- Keep README in sync with code

5. **Testing:**
	- Write unit tests for all business logic
	- Use integration tests for end-to-end scenarios
	- Test error cases and edge conditions
	- Use `#[should_panic]` for panic tests

### Code Organization

1. **Module Structure:**
	- One module per file
	- Group related functionality
	- Use `mod.rs` for module organization
	- Keep files under 500 lines when possible

2. **Naming Conventions:**
	- Types: `PascalCase`
	- Functions/variables: `snake_case`
	- Constants: `SCREAMING_SNAKE_CASE`
	- Lifetimes: short lowercase ('a, 'b)

3. **Imports:**
	- Group: std, external crates, internal modules
	- Use `use` statements, not fully qualified paths
	- Keep imports sorted and organized

## Performance Considerations

### Optimization Strategy

1. **Profiling:**
	- Use `cargo flamegraph` for CPU profiling
	- Use `cargo bench` for benchmarking
	- Monitor memory usage with `valgrind` or `heaptrack`

2. **Async Performance:**
	- Balance parallelism vs resource usage
	- Use bounded channels to prevent memory growth
	- Implement timeout and cancellation
	- Consider using `rayon` for CPU-bound work

3. **Network Efficiency:**
	- Connection pooling for reusable connections
	- Batch operations when possible
	- Implement exponential backoff for retries
	- Respect rate limits

4. **Memory Management:**
	- Use `Arc` for shared immutable data
	- Prefer stack allocation over heap when possible
	- Use `Vec::with_capacity()` for known sizes
	- Profile for memory leaks in long-running mode

## Deployment Considerations

### Platform Support

| Platform | Status | Notes |
| --- | --- | --- |
| Linux | ✅ Primary | Full support, all features |
| WSL | 🟡 Limited | Works but with some limitations |
| macOS | ⏳ Testing | Intel and ARM support planned |
| BSD | ⏳ Testing | Community testing needed |
| Windows Native | ⏳ Future | Limited support planned |

### System Requirements

**Minimum:**

- Rust 1.89+ (2024 edition support)
- Linux kernel 3.10+ or equivalent
- 256 MB RAM
- 50 MB disk space

**Recommended:**

- Rust 1.89+
- Modern Linux distribution
- 1 GB RAM for large-scale scans
- 100 MB disk space

### Packaging

**Distribution Methods:**

1. **Source:**
	- Clone repository and build with cargo
	- Requires Rust toolchain

2. **Binary Releases:**
	- Pre-built binaries for common platforms
	- Static linking when possible
	- Distributed via GitHub Releases

3. **Package Managers (Future):**
	- Debian/Ubuntu: `.deb` package
	- Red Hat/Fedora: `.rpm` package
	- Arch Linux: AUR package
	- Homebrew: macOS formula
	- Cargo: `cargo install huginn`

## Future Enhancements

### Short-term (v0.2.0)

- [ ] Complete CLI implementation
- [ ] Implement all basic scan types
- [ ] Add comprehensive error handling
- [ ] Create example configurations
- [ ] Write user documentation

### Medium-term (v0.3.0)

- [ ] Daemon mode with REST API
- [ ] Result persistence and history
- [ ] Scheduling system
- [ ] Web dashboard
- [ ] Export to multiple formats

### Long-term (v1.0.0+)

- [ ] Plugin marketplace
- [ ] Distributed scanning
- [ ] Machine learning threat detection
- [ ] Integration with SIEM systems
- [ ] Compliance reporting

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines on:

- Code submission process
- Testing requirements
- Documentation standards
- Review process

## License

Dual-licensed under MIT OR Apache-2.0. See [LICENSE.md](../../LICENSE.md) for details.

## Notes for Implementation Agents

### Getting Started

1. **Environment Setup:**
	```bash
	# Install Rust
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

	# Clone repository
	git clone https://github.com/JEleniel/huginn.git
	cd huginn

	# Build project
	cargo build

	# Run tests
	cargo test
	```

2. **Development Cycle:**
	- Read existing code and understand structure
	- Make small, focused changes
	- Write tests for new functionality
	- Run `cargo test` and `cargo clippy`
	- Update documentation
	- Submit PR with clear description

3. **Common Tasks:**
	- Adding a new plugin: Copy existing plugin structure, implement trait
	- Adding configuration: Update Config struct and load() function
	- Adding CLI argument: Update clap definitions in main.rs
	- Adding test: Create test module at bottom of file

### Important Files

- `Cargo.toml` (workspace): Workspace configuration and shared dependencies
- `huginn/Cargo.toml`: Binary package configuration
- `huginn/src/main.rs`: Application entry point
- `huginn/src/plugins/mod.rs`: Plugin trait definition
- `rustfmt.toml`: Code formatting rules

### Testing Strategy

- Unit tests: Test individual functions in isolation
- Integration tests: Test complete workflows
- Always test error conditions
- Use fixtures for test data
- Mock external dependencies when needed

### Debugging Tips

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run specific test with output
cargo test test_name -- --nocapture

# Check for common issues
cargo clippy

# Format code
cargo fmt
```

### Common Pitfalls

1. **Async Functions:**
	- Don't forget `#[async_trait]` on trait implementations
	- Use `.await` on async functions
	- Handle cancellation properly

2. **Error Handling:**
	- Always propagate errors with `?`
	- Don't use `unwrap()` in production code
	- Provide helpful error messages

3. **Ownership:**
	- Clone sparingly, prefer borrowing
	- Use `Arc` for shared ownership
	- Understand the borrow checker

4. **Testing:**
	- Test both success and error paths
	- Don't make tests depend on external state
	- Use `#[tokio::test]` for async tests

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Conclusion

This implementation plan provides a roadmap for developing Huginn into a robust, secure, and efficient cyber threat scanning toolkit. The modular architecture allows for incremental development while maintaining code quality and security standards.

Each phase builds upon previous work, with clear objectives and deliverables. The plugin system enables easy extension with new scanning techniques while maintaining a clean separation of concerns.

Follow the development workflow and code standards to ensure consistency and maintainability throughout the project lifecycle.
