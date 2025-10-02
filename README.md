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
# Install command here
```

## Configuration

The following configuration options are available:

### Environment Variables

| Variable     | Description                       | Default |
| ------------ | --------------------------------- | ------- |
| `API_KEY`    | Authentication key for API access | N/A     |
| `DEBUG_MODE` | Enable debug logging              | false   |
| `PORT`       | Server port number                | 3000    |

### Configuration File

The `config.json` file supports the following options:

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
