# Deployment Architecture

## Overview

This document describes the deployment architecture for Huginn, including supported platforms, deployment modes, system requirements, and operational considerations.

## Deployment Modes

Huginn supports multiple deployment modes to accommodate different use cases:

```mermaid
graph TB
    subgraph "Deployment Modes"
        CLI[Standalone CLI]
        Service[System Service]
        Container[Containerized]
        Library[Library Integration]
    end
    
    subgraph "Use Cases"
        AdHoc[Ad-hoc Scans]
        Scheduled[Scheduled Scans]
        Monitoring[Continuous Monitoring]
        CICD[CI/CD Integration]
    end
    
    CLI --> AdHoc
    Service --> Scheduled
    Service --> Monitoring
    Container --> CICD
    Container --> Monitoring
    Library --> CICD
```

### Standalone CLI Mode

The primary deployment mode for interactive use.

```mermaid
graph LR
    User[Security Admin] --> CLI[Huginn CLI]
    CLI --> Target1[Target Network 1]
    CLI --> Target2[Target Network 2]
    CLI --> Target3[Target Network 3]
    CLI --> Report[Scan Report]
```

**Characteristics:**
- Single execution per invocation
- Configuration from files or CLI arguments
- Results output to stdout or file
- No persistent state
- Suitable for ad-hoc scanning

**Typical Usage:**
```bash
# Quick scan
huginn scan 192.168.1.0/24 -t ping,tcp_connect

# Detailed scan with output
huginn scan -T targets.txt -t all -o results.json -f json
```

### System Service Mode (Future)

Background service for continuous operation.

```mermaid
graph TB
    subgraph "Service Components"
        API[HTTP API]
        Scheduler[Job Scheduler]
        Scanner[Scanner Engine]
        Storage[Result Storage]
    end
    
    subgraph "Clients"
        WebUI[Web Dashboard]
        APIClient[API Clients]
        CLI2[CLI Client]
    end
    
    WebUI --> API
    APIClient --> API
    CLI2 --> API
    
    API --> Scheduler
    Scheduler --> Scanner
    Scanner --> Storage
```

**Characteristics:**
- Long-running daemon process
- REST API for control
- Job scheduling support
- Result persistence
- Web dashboard for monitoring

**Service Management:**
```bash
# systemd service
sudo systemctl start huginn
sudo systemctl enable huginn
sudo systemctl status huginn

# Service API
curl http://localhost:8080/api/scan -d '{"target": "192.168.1.1"}'
```

### Containerized Deployment

Docker/Podman containers for isolated execution.

```mermaid
graph TB
    subgraph "Container Environment"
        Container[Huginn Container]
        Volume1[Config Volume]
        Volume2[Results Volume]
        Network[Network Access]
    end
    
    subgraph "Orchestration"
        Compose[Docker Compose]
        K8s[Kubernetes]
        Swarm[Docker Swarm]
    end
    
    Compose --> Container
    K8s --> Container
    Swarm --> Container
    
    Container --> Volume1
    Container --> Volume2
    Container --> Network
```

**Characteristics:**
- Isolated environment
- Reproducible builds
- Easy deployment
- Resource constraints
- Network mode configuration

**Example Dockerfile:**
```dockerfile
FROM rust:1.89-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libcap2-bin ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/huginn /usr/local/bin/
ENTRYPOINT ["huginn"]
```

**Container Usage:**
```bash
# Run with Docker
docker run -v $(pwd)/config:/config \
    -v $(pwd)/results:/results \
    --network=host \
    huginn:latest scan -T /config/targets.txt

# Docker Compose
docker-compose up -d
```

## Platform Support

### Linux (Primary Platform)

```mermaid
graph TB
    subgraph "Linux Distributions"
        Ubuntu[Ubuntu 20.04+]
        Debian[Debian 11+]
        RHEL[RHEL 8+]
        Fedora[Fedora 36+]
        Arch[Arch Linux]
    end
    
    subgraph "Capabilities"
        NetRaw[CAP_NET_RAW<br/>Raw sockets]
        NetAdmin[CAP_NET_ADMIN<br/>Network admin]
        Standard[Standard user<br/>TCP scans]
    end
    
    Ubuntu --> NetRaw
    Debian --> NetRaw
    RHEL --> NetRaw
    Fedora --> NetRaw
    Arch --> NetRaw
    
    Ubuntu --> Standard
    Debian --> Standard
    RHEL --> Standard
```

**Support Level:** ✅ Full Support

**Features:**
- All scan types supported
- Linux capabilities for privilege management
- Native binary packages
- Systemd service integration

**System Requirements:**
- Kernel: 3.10+ (recommended 5.0+)
- glibc: 2.28+ or musl
- Architecture: x86_64, ARM64

**Installation:**
```bash
# From package manager (future)
apt-get install huginn
dnf install huginn
pacman -S huginn

# From binary release
wget https://github.com/.../huginn-linux-x86_64.tar.gz
tar xzf huginn-linux-x86_64.tar.gz
sudo install -m 755 huginn /usr/local/bin/

# Set capabilities for privileged scans
sudo setcap cap_net_raw+ep /usr/local/bin/huginn
```

### Windows/WSL

```mermaid
graph TB
    subgraph "Windows Deployment"
        WSL2[WSL 2]
        Native[Native Windows]
    end
    
    subgraph "Capabilities"
        Limited[Limited Scans<br/>TCP Connect only]
        Full[Full Scans<br/>via WSL]
    end
    
    WSL2 --> Full
    Native --> Limited
```

**Support Level:** 🟡 Limited (WSL), ⏳ Future (Native)

**WSL Support:**
- Run Linux binary in WSL 2
- Most features available
- Some privilege limitations

**Native Windows:**
- TCP Connect scans work
- Raw socket support limited
- Future: Winsock raw socket support

**Installation (WSL):**
```bash
# In WSL 2
wsl --install
# Then follow Linux installation
```

### macOS

```mermaid
graph TB
    subgraph "macOS Support"
        Intel[Intel Macs]
        AppleSilicon[Apple Silicon M1/M2/M3]
    end
    
    subgraph "Capabilities"
        BPF[BPF Devices<br/>Raw packets]
        Standard[Standard Scans<br/>TCP/UDP]
    end
    
    Intel --> BPF
    AppleSilicon --> BPF
    Intel --> Standard
    AppleSilicon --> Standard
```

**Support Level:** ⏳ Testing

**Features:**
- Cross-compiled ARM64 and x86_64 binaries
- BPF device access for raw packets
- Universal binary support planned

**System Requirements:**
- macOS 11 Big Sur or later
- Apple Silicon (M1/M2/M3) or Intel

**Installation:**
```bash
# Homebrew (future)
brew install huginn

# From binary
curl -LO https://github.com/.../huginn-darwin-universal.tar.gz
tar xzf huginn-darwin-universal.tar.gz
sudo install -m 755 huginn /usr/local/bin/
```

### BSD Systems

**Support Level:** ⏳ Community Testing

**Distributions:**
- FreeBSD
- OpenBSD
- NetBSD

**Status:**
- Basic functionality should work
- Community testing needed
- Platform-specific adjustments may be required

## System Architecture

### Single-Host Deployment

```mermaid
graph TB
    subgraph "Host System"
        Huginn[Huginn Process]
        Config[Configuration Files]
        Results[Result Files]
        Logs[Log Files]
    end
    
    subgraph "System Resources"
        Network[Network Stack]
        FileSystem[File System]
        Memory[Memory]
        CPU[CPU]
    end
    
    Huginn --> Config
    Huginn --> Results
    Huginn --> Logs
    Huginn --> Network
    Huginn --> FileSystem
    Huginn --> Memory
    Huginn --> CPU
    
    Network --> Internet[Internet/Networks]
```

### Multi-Instance Deployment (Future)

```mermaid
graph TB
    subgraph "Coordinator"
        Master[Master Instance]
        Queue[Job Queue]
        Aggregator[Result Aggregator]
    end
    
    subgraph "Workers"
        Worker1[Worker 1]
        Worker2[Worker 2]
        Worker3[Worker 3]
        WorkerN[Worker N]
    end
    
    Master --> Queue
    Queue --> Worker1
    Queue --> Worker2
    Queue --> Worker3
    Queue --> WorkerN
    
    Worker1 --> Aggregator
    Worker2 --> Aggregator
    Worker3 --> Aggregator
    WorkerN --> Aggregator
    
    Aggregator --> Master
```

## Network Configuration

### Network Access Requirements

```mermaid
graph TB
    subgraph "Outbound Traffic"
        ICMP[ICMP<br/>Echo Requests]
        TCP[TCP<br/>Various Ports]
        UDP[UDP<br/>Various Ports]
        DNS[DNS<br/>Port 53]
    end
    
    subgraph "Firewall Rules"
        Allow[Allow Rules]
        RateLimit[Rate Limiting]
        Logging[Traffic Logging]
    end
    
    ICMP --> Allow
    TCP --> Allow
    UDP --> Allow
    DNS --> Allow
    
    Allow --> RateLimit
    RateLimit --> Logging
```

**Required Outbound Access:**
- ICMP: Echo requests (type 8) for ping scans
- TCP: Destination ports as configured
- UDP: Destination ports as configured
- DNS: Port 53 for name resolution

**Firewall Considerations:**
- Allow ICMP echo requests and replies
- Allow TCP connections to scan ports
- Allow UDP packets to scan ports
- Consider rate limiting for large scans

### Network Topologies

**Direct Scanning:**
```mermaid
graph LR
    Huginn[Huginn Scanner] --> Router[Network Router]
    Router --> Target1[Target 1]
    Router --> Target2[Target 2]
    Router --> Target3[Target 3]
```

**Scanning Through Jump Host:**
```mermaid
graph LR
    Admin[Admin Workstation] --> Jump[Jump Host]
    Jump --> Huginn[Huginn Scanner]
    Huginn --> DMZ[DMZ Network]
    Huginn --> Internal[Internal Network]
```

**Distributed Scanning (Future):**
```mermaid
graph TB
    Coordinator[Coordinator] --> Scanner1[Scanner 1<br/>Region A]
    Coordinator --> Scanner2[Scanner 2<br/>Region B]
    Coordinator --> Scanner3[Scanner 3<br/>Region C]
    
    Scanner1 --> Network1[Network Range 1]
    Scanner2 --> Network2[Network Range 2]
    Scanner3 --> Network3[Network Range 3]
```

## Resource Requirements

### Minimum Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| CPU | 1 core | 2+ cores |
| Memory | 256 MB | 1 GB |
| Disk Space | 50 MB | 100 MB |
| Network | 1 Mbps | 10+ Mbps |

### Scaling Guidelines

```mermaid
graph TB
    subgraph "Small Scale"
        S1[< 100 targets]
        S2[1 core, 256MB RAM]
        S3[Concurrency: 5]
    end
    
    subgraph "Medium Scale"
        M1[100-1000 targets]
        M2[2 cores, 1GB RAM]
        M3[Concurrency: 10-20]
    end
    
    subgraph "Large Scale"
        L1[1000+ targets]
        L2[4+ cores, 2GB+ RAM]
        L3[Concurrency: 50+]
    end
```

**Resource Calculation:**
- Memory: ~1 MB per concurrent scan + base overhead
- CPU: Minimal for I/O-bound operations
- Network: Depends on scan types and targets
- Disk: Results storage varies by target count

## Storage Architecture

### File System Layout

```
/usr/local/bin/
    huginn                   # Binary executable

/etc/huginn/
    config.json             # System-wide configuration
    targets.txt             # Default target lists

/var/lib/huginn/
    results/                # Scan results
    cache/                  # Cached data (DNS, etc.)

/var/log/huginn/
    huginn.log             # Application logs
    audit.log              # Audit trail

/home/user/.config/huginn/
    config.json            # User configuration
    credentials            # User credentials (mode 0600)
```

### Result Storage

```mermaid
graph TB
    subgraph "Storage Options"
        File[File System<br/>JSON/CSV/Text]
        DB[Database<br/>SQLite/PostgreSQL]
        Stream[Streaming<br/>Stdout/API]
    end
    
    subgraph "Result Format"
        JSON[JSON Format]
        CSV[CSV Format]
        Text[Text Format]
        Binary[Binary Format]
    end
    
    Scanner[Scanner] --> File
    Scanner --> DB
    Scanner --> Stream
    
    File --> JSON
    File --> CSV
    File --> Text
    DB --> Binary
```

## Security Architecture

### Privilege Management

```mermaid
graph TB
    subgraph "Execution Modes"
        User[User Mode<br/>No privileges]
        Capabilities[Capabilities<br/>CAP_NET_RAW]
        Root[Root Mode<br/>Full privileges]
    end
    
    subgraph "Scan Types"
        TCP[TCP Connect<br/>✓ User mode]
        UDP[UDP Scan<br/>✓ User mode]
        ICMP[ICMP Ping<br/>⚠ Capabilities]
        SYN[SYN Scan<br/>⚠ Capabilities]
    end
    
    User --> TCP
    User --> UDP
    Capabilities --> ICMP
    Capabilities --> SYN
    Root --> ICMP
    Root --> SYN
```

**Recommended Setup:**
```bash
# Grant capabilities (Linux)
sudo setcap cap_net_raw+ep /usr/local/bin/huginn

# Verify capabilities
getcap /usr/local/bin/huginn

# Create dedicated user (service mode)
sudo useradd -r -s /bin/false -d /var/lib/huginn huginn
```

### System Hardening

```mermaid
graph TB
    subgraph "Build Security"
        PIE[Position Independent<br/>Executable]
        Canary[Stack Canaries]
        RELRO[RELRO]
        NX[NX Bit]
    end
    
    subgraph "Runtime Security"
        ASLR[ASLR Enabled]
        Seccomp[Seccomp Filters]
        Namespace[Namespaces]
        Limits[Resource Limits]
    end
    
    subgraph "Deployment Security"
        MinPriv[Minimal Privileges]
        Isolation[Process Isolation]
        Audit[Audit Logging]
        Monitoring[Security Monitoring]
    end
```

## Service Management

### Systemd Service (Linux)

```ini
# /etc/systemd/system/huginn.service
[Unit]
Description=Huginn Cyber Threat Scanner
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=huginn
Group=huginn
WorkingDirectory=/var/lib/huginn
ExecStart=/usr/local/bin/huginn daemon --config /etc/huginn/config.json
Restart=on-failure
RestartSec=10

# Security hardening
PrivateTmp=yes
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/huginn /var/log/huginn

# Capabilities
AmbientCapabilities=CAP_NET_RAW
CapabilityBoundingSet=CAP_NET_RAW

[Install]
WantedBy=multi-user.target
```

**Service Management:**
```bash
# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable huginn
sudo systemctl start huginn

# Check status
sudo systemctl status huginn
sudo journalctl -u huginn -f

# Stop and disable
sudo systemctl stop huginn
sudo systemctl disable huginn
```

## Monitoring and Observability

### Metrics Collection

```mermaid
graph TB
    subgraph "Metrics"
        Scans[Scans Completed]
        Errors[Error Count]
        Duration[Scan Duration]
        Resources[Resource Usage]
    end
    
    subgraph "Collection"
        Logs[Log Files]
        API[Metrics API]
        Export[Prometheus Export]
    end
    
    subgraph "Visualization"
        Dashboard[Grafana Dashboard]
        Alerts[Alert Manager]
        Reports[Reports]
    end
    
    Scans --> Logs
    Errors --> Logs
    Duration --> Logs
    Resources --> Logs
    
    Logs --> API
    API --> Export
    Export --> Dashboard
    Dashboard --> Alerts
```

### Health Checks

```bash
# CLI health check
huginn --version

# Service health check (future)
curl http://localhost:8080/health

# Expected response
{
    "status": "healthy",
    "version": "0.1.0",
    "uptime_seconds": 3600
}
```

## Backup and Recovery

### Configuration Backup

```bash
# Backup configuration
tar czf huginn-config-$(date +%Y%m%d).tar.gz \
    /etc/huginn \
    /home/*/.config/huginn

# Restore configuration
tar xzf huginn-config-20240101.tar.gz -C /
```

### Result Archival

```bash
# Archive old results
find /var/lib/huginn/results -mtime +30 -exec gzip {} \;

# Move to archive
mv /var/lib/huginn/results/*.gz /var/lib/huginn/archive/
```

## Disaster Recovery

### Recovery Procedures

1. **Configuration Loss:**
   - Restore from backup
   - Regenerate from templates
   - Validate configuration

2. **Service Failure:**
   - Check service logs
   - Verify permissions and capabilities
   - Restart service
   - Escalate if persistent

3. **Data Corruption:**
   - Stop service
   - Restore from backup
   - Verify data integrity
   - Resume operations

## Migration Guide

### Upgrading Huginn

```bash
# Stop service
sudo systemctl stop huginn

# Backup current installation
sudo cp /usr/local/bin/huginn /usr/local/bin/huginn.backup

# Install new version
sudo install -m 755 huginn-new /usr/local/bin/huginn

# Update capabilities
sudo setcap cap_net_raw+ep /usr/local/bin/huginn

# Start service
sudo systemctl start huginn

# Verify
huginn --version
sudo systemctl status huginn
```

### Configuration Migration

```bash
# Check for config changes
huginn --check-config /etc/huginn/config.json

# Migrate if needed
huginn migrate-config /etc/huginn/config.json
```

## Performance Tuning

### System Tuning

```bash
# Increase file descriptor limits
ulimit -n 65536

# Kernel parameters for high-performance scanning
sudo sysctl -w net.ipv4.ip_local_port_range="1024 65535"
sudo sysctl -w net.core.somaxconn=65535
sudo sysctl -w net.ipv4.tcp_max_syn_backlog=65535
```

### Application Tuning

```json
{
    "concurrency": 50,
    "timeout": 10,
    "rate_limit": 1000,
    "buffer_size": 8192
}
```

## Summary

The Huginn deployment architecture provides:

- **Flexibility**: Multiple deployment modes for different use cases
- **Scalability**: From single host to distributed deployments
- **Security**: Comprehensive security hardening and privilege management
- **Reliability**: Service management and monitoring capabilities
- **Portability**: Support for multiple platforms and architectures

This architecture supports current needs while providing a path for future enhancements and scaling.
