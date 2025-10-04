# Application Name

![Static Badge](https://img.shields.io/badge/AAA-green?style=for-the-badge&label=WCAG%20Accessibility&labelColor=%23204080&link=https%3A%2F%2Fwww.w3.org%2FWAI%2Fstandards-guidelines%2Fwcag%2F)
![GitHub License](https://img.shields.io/github/license/JEleniel/huginn?style=for-the-badge&logo=MIT&labelColor=%23204080&link=LICENSE.md)
![Version](https://img.shields.io/github/v/release/JEleniel/template?style=for-the-badge)
![Issues](https://img.shields.io/github/issues/JEleniel/template?style=for-the-badge)
![Pull Requests](https://img.shields.io/github/issues-pr/JEleniel/template?style=for-the-badge)

⭐ Star us on GitHub — it motivates us a lot!

## Overview

The raven of Odin searches the world for knowledge and threats; Huginn is a cyber threat scanning toolkit. It can be run as a standalone tool or as a system service for continual scanning.

## Features

- 100% Rust with `unsafe="forbid"` linting
- Plugin architecture makes it easy to add new scanning techniques
- Many useful scans already included:

| Scan Type	| Description |
| --- | --- |
| TCP Connect Scan	| Establishes a full TCP connection to determine port status. |
| TCP SYN Scan	| A stealth scan that sends SYN packets to check if ports are open, closed, or filtered. |
| UDP Scan	| Scans for open UDP ports, which do not require a handshake. |
| Version Detection	| Identifies the version of services running on open ports.	|
| OS Detection	| Attempts to determine the operating system of the target device.|
| Server Detection | Scans to detect specific types of services, e.g. HTTP, on the target device. |
| Ping Scan	| Checks which hosts are up by sending ICMP echo requests. |

## Documentation

- [Full Documentation](docs/README.md)
- [API Reference](docs/api/README.md)
- [Designs](docs/design/README.md)

## Getting Started

Explore the [Getting Started](../../wiki/Getting-Started) guide.

### Installation

```bash
# Clone the repository
git clone https://github.com/JEleniel/huginn.git
cd huginn

# Build the project
cargo build --release

# Run Huginn
./target/release/huginn --help
```

## Configuration

Huginn supports multiple configuration methods with the following precedence (highest to lowest):
1. Command-line arguments
2. Environment variables (prefix: `HUGINN_`)
3. Configuration file (`config.json`)
4. Default values

### Command-Line Usage

```bash
# Basic scan
huginn scan --target 192.168.1.1

# Multiple targets with specific scan types
huginn scan --target 192.168.1.1 --target example.com --scan-type ping,tcp_connect

# Specify output format
huginn scan --target 192.168.1.1 --format json --output results.json

# Show version
huginn version

# Get help
huginn --help
huginn scan --help
```

### Output Formats

Huginn supports three output formats:

- **text** (default): Human-readable colored output with summary statistics
- **json**: Machine-readable JSON format
- **csv**: Spreadsheet-compatible CSV format

```bash
# Text output with colors (default)
huginn scan -t 192.168.1.1

# JSON output
huginn scan -t 192.168.1.1 -f json

# CSV output for spreadsheets
huginn scan -t 192.168.1.1 -f csv -o results.csv
```

### Progress Reporting

Huginn displays a progress bar for multi-target scans showing:
- Current operation (scan type and target)
- Elapsed time and estimated time to completion
- Progress percentage

Progress bars are automatically disabled in verbose mode (`-v`) or when output is redirected.

### Environment Variables

All configuration options can be set via environment variables with the `HUGINN_` prefix:

| Variable                | Description                                    | Default |
| ----------------------- | ---------------------------------------------- | ------- |
| `HUGINN_CONFIG`         | Path to configuration file                     | N/A     |
| `HUGINN_TARGETS`        | Comma-separated list of targets to scan        | N/A     |
| `HUGINN_SCAN_TYPES`     | Comma-separated list of scan types to perform  | `ping`  |
| `HUGINN_PORT`           | Server port for daemon mode                    | N/A     |
| `HUGINN_OUTPUT_FORMAT`  | Output format: text, json, or csv              | `text`  |
| `HUGINN_LOG_LEVEL`      | Log level: debug, info, warn, or error         | `info`  |

Example:

```bash
HUGINN_TARGETS="192.168.1.1,example.com" HUGINN_SCAN_TYPES="ping,tcp_connect" huginn scan --target localhost
```

### Configuration File

You can specify configuration in a JSON file. By default, Huginn looks for `config.json` in the current directory, or you can specify a custom path with `--config` or `HUGINN_CONFIG`.

**Example: config.example.json**

```json
{
	"targets": ["192.168.1.1", "example.com"],
	"scan_types": ["ping", "tcp_connect"],
	"output_format": "text",
	"log_level": "info"
}
```

**Example: config.full.json (all options)**

```json
{
	"targets": [
		"192.168.1.1",
		"192.168.1.0/24",
		"example.com"
	],
	"scan_types": ["ping", "tcp_connect", "tcp_syn", "udp"],
	"port": 8080,
	"output_format": "json",
	"log_level": "debug"
}
```

**Example: config.minimal.json**

```json
{
	"targets": ["localhost"]
}
```

### Configuration Options Reference

| Option           | Type     | Description                                              | Default  | Required |
| ---------------- | -------- | -------------------------------------------------------- | -------- | -------- |
| `targets`        | array    | List of targets (IPs, hostnames, CIDR ranges)            | `[]`     | Yes      |
| `scan_types`     | array    | Scan types: `ping`, `tcp_connect`, `tcp_syn`, `udp`      | `[ping]` | No       |
| `port`           | number   | Server port for daemon mode (future feature)             | N/A      | No       |
| `output_format`  | string   | Output format: `text`, `json`, or `csv`                  | `text`   | No       |
| `log_level`      | string   | Logging level: `debug`, `info`, `warn`, or `error`       | `info`   | No       |

### Scan Types

The following scan types are available:

| Scan Type       | Description                                            | Privileges Required |
| --------------- | ------------------------------------------------------ | ------------------- |
| `ping`          | ICMP echo request to check if host is alive            | CAP_NET_RAW or root |
| `tcp_connect`   | Full TCP connection to determine port status           | None                |
| `tcp_syn`       | Stealth SYN scan (half-open connection)                | CAP_NET_RAW or root |
| `udp`           | UDP port scanning                                      | None                |

**Note:** Some scan types require elevated privileges. Run with appropriate permissions or use scans that don't require special privileges.

## Support

[Getting Support](SUPPORT.md)

### Supported Platforms

| Platform | Version 1.0.0 | Notes               |
| -------- | :-----------: | ------------------- |
| Linux    |       ✔       | Full support        |
| Windows  |      ❌       | Limited WSL support |
| MacOS    |      ❌       | Intel/ARM support   |
| BSD      |      ❌       |  Testing needed      |

## Feedback and Contributions

Please be sure to read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

[Reporting Security Issues](SECURITY.md)

[Contributing to the Project](CONTRIBUTING.md)

## Building and Testing

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## License

This project is dual licensed under the MIT and Apache 2.0 licenses. See [LICENSE.md](LICENSE.md) file for details.

## Contributors

<a href="https://github.com/JEleniel/huginn/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=JEleniel/huginn" alt="contributor images" />
</a>

## Tooling

The tools we use include:

![Dependabot](https://img.shields.io/badge/dependabot-025E8C?style=for-the-badge&logo=dependabot&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Linux Mint](https://img.shields.io/badge/Linux%20Mint-87CF3E?style=for-the-badge&logo=Linux%20Mint&logoColor=white)
![VS Code](https://img.shields.io/badge/VS%20Code-0078d7.svg?style=for-the-badge&logo=visual-studio-code&logoColor=white)
![Git](https://img.shields.io/badge/git-%23F05033.svg?style=for-the-badge&logo=git&logoColor=white)

# Acknowledgements

Badges provided by [Shields.io](https://shields.io)
Contributer images thanks to [contrib.rocks](https://contrib.rocks).
