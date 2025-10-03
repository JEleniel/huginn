# Plugin System Architecture

## Overview

The Huginn plugin system provides a flexible, type-safe mechanism for implementing different types of network security scans. This document details the architecture, interfaces, and lifecycle of the plugin system.

## Plugin System Design

### Core Concepts

```mermaid
graph TB
    subgraph "Plugin Trait"
        Trait[Plugin Trait<br/>- name<br/>- scan_type<br/>- scan]
    end
    
    subgraph "Built-in Plugins"
        Ping[Ping Scan]
        TCPConn[TCP Connect]
        TCPSYN[TCP SYN]
        UDP[UDP Scan]
        Service[Service Detection]
        Version[Version Detection]
        OS[OS Detection]
    end
    
    subgraph "Future Plugins"
        Custom1[Custom Plugin 1]
        Custom2[Custom Plugin 2]
        ThirdParty[Third-Party Plugins]
    end
    
    Trait -.implements.-> Ping
    Trait -.implements.-> TCPConn
    Trait -.implements.-> TCPSYN
    Trait -.implements.-> UDP
    Trait -.implements.-> Service
    Trait -.implements.-> Version
    Trait -.implements.-> OS
    
    Trait -.implements.-> Custom1
    Trait -.implements.-> Custom2
    Trait -.implements.-> ThirdParty
```

## Plugin Trait Interface

### Trait Definition

```mermaid
classDiagram
    class Plugin {
        <<trait>>
        +name() String
        +scan_type() String
        +description() String
        +required_privileges() Privileges
        +init(config: PluginConfig) Result
        +scan(target: &str) Future~Result~ScanResult~~
        +cleanup() Result
    }
    
    class PluginConfig {
        +timeout: Duration
        +retries: u32
        +custom: HashMap~String,Value~
    }
    
    class ScanResult {
        +target: String
        +scan_type: String
        +status: Status
        +details: HashMap~String,Value~
        +timestamp: DateTime
        +duration: Duration
        +error: Option~String~
    }
    
    class Privileges {
        <<enumeration>>
        None
        CapNetRaw
        CapNetAdmin
        Root
    }
    
    class Status {
        <<enumeration>>
        Success
        Failure
        Timeout
        Filtered
        Unreachable
    }
    
    Plugin --> PluginConfig
    Plugin --> ScanResult
    Plugin --> Privileges
    ScanResult --> Status
```

### Trait Methods

**Required Methods:**

- `name()`: Returns unique plugin identifier
- `scan_type()`: Returns scan type name for user selection
- `scan(target)`: Performs the actual scan operation (async)

**Optional Methods (with defaults):**

- `description()`: Returns human-readable description
- `required_privileges()`: Returns privilege requirements
- `init(config)`: Initialize plugin with configuration
- `cleanup()`: Clean up plugin resources

## Plugin Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Unregistered
    Unregistered --> Registered: register()
    Registered --> Initialized: init(config)
    Initialized --> Ready: validation success
    Initialized --> Error: validation failure
    Ready --> Executing: scan(target)
    Executing --> Ready: scan complete
    Executing --> Error: scan error
    Ready --> CleaningUp: cleanup()
    Error --> CleaningUp: cleanup()
    CleaningUp --> [*]
```

### Lifecycle Stages

1. **Unregistered**: Plugin struct created but not registered
2. **Registered**: Plugin registered with PluginManager
3. **Initialized**: Plugin initialized with configuration
4. **Ready**: Plugin ready to perform scans
5. **Executing**: Plugin actively scanning target
6. **CleaningUp**: Plugin releasing resources
7. **Error**: Plugin encountered error

## Plugin Registration

```mermaid
sequenceDiagram
    participant Main
    participant PluginMgr as Plugin Manager
    participant Plugin
    
    Main->>PluginMgr: new()
    Main->>Plugin: new()
    Main->>PluginMgr: register(plugin)
    PluginMgr->>Plugin: name()
    Plugin-->>PluginMgr: "ping_scan"
    PluginMgr->>PluginMgr: validate unique name
    PluginMgr->>PluginMgr: store plugin
    PluginMgr-->>Main: Ok()
    
    Main->>PluginMgr: init_all(config)
    PluginMgr->>Plugin: init(config)
    Plugin->>Plugin: validate config
    Plugin-->>PluginMgr: Ok()
    PluginMgr-->>Main: Ok()
```

## Plugin Execution

```mermaid
sequenceDiagram
    participant Scanner
    participant PluginMgr as Plugin Manager
    participant Plugin
    participant Network
    
    Scanner->>PluginMgr: execute_scans(target, types)
    
    loop For each scan type
        PluginMgr->>PluginMgr: get_plugin(type)
        PluginMgr->>Plugin: check privileges
        
        alt Has privileges
            PluginMgr->>Plugin: scan(target)
            Plugin->>Network: perform operations
            Network-->>Plugin: response
            Plugin->>Plugin: process results
            Plugin-->>PluginMgr: ScanResult
        else Missing privileges
            PluginMgr-->>Scanner: PrivilegeError
        end
    end
    
    PluginMgr-->>Scanner: Vec<ScanResult>
```

## Built-in Plugin Implementations

### Ping Scan Plugin

```mermaid
graph TB
    subgraph "Ping Scan Plugin"
        Init[Initialize ICMP Socket]
        Send[Send ICMP Echo Request]
        Wait[Wait for Reply]
        Process[Process Response]
        Result[Return Result]
    end
    
    Init --> Send
    Send --> Wait
    Wait -->|Reply Received| Process
    Wait -->|Timeout| Result
    Process --> Result
    
    Note1[Requires: CAP_NET_RAW<br/>or root on some systems]
```

**Features:**
- ICMP echo request/reply
- Configurable timeout
- Latency measurement
- Fallback to TCP ping if ICMP unavailable

### TCP Connect Scan Plugin

```mermaid
graph TB
    subgraph "TCP Connect Scan"
        Parse[Parse Port Range]
        Loop[For Each Port]
        Connect[Attempt Connection]
        Classify[Classify Result]
        Store[Store Port Status]
        Banner[Optional Banner Grab]
    end
    
    Parse --> Loop
    Loop --> Connect
    Connect -->|Success| Banner
    Connect -->|Refused| Classify
    Connect -->|Timeout| Classify
    Banner --> Classify
    Classify --> Store
    Store -->|More Ports| Loop
    Store -->|Done| Result[Return Results]
    
    Note1[No special privileges needed]
```

**Features:**
- Full TCP three-way handshake
- Port range scanning
- Service banner grabbing
- Connection timeout
- No special privileges required

### TCP SYN Scan Plugin

```mermaid
graph TB
    subgraph "TCP SYN Scan"
        InitRaw[Initialize Raw Socket]
        Loop[For Each Port]
        Craft[Craft SYN Packet]
        Send[Send SYN]
        Listen[Listen for Response]
        SendRST[Send RST]
        Classify[Classify Port State]
    end
    
    InitRaw --> Loop
    Loop --> Craft
    Craft --> Send
    Send --> Listen
    Listen -->|SYN-ACK| SendRST
    Listen -->|RST| Classify
    Listen -->|Timeout| Classify
    SendRST --> Classify
    Classify -->|More Ports| Loop
    Classify -->|Done| Result[Return Results]
    
    Note1[Requires: CAP_NET_RAW<br/>or root]
```

**Features:**
- Stealth scanning (no full connection)
- Raw socket operations
- Port state classification
- Requires elevated privileges

### UDP Scan Plugin

```mermaid
graph TB
    subgraph "UDP Scan"
        Loop[For Each Port]
        Send[Send UDP Packet]
        Listen[Listen for Response]
        CheckICMP[Check ICMP Errors]
        Classify[Classify State]
    end
    
    Loop --> Send
    Send --> Listen
    Listen -->|UDP Reply| Classify
    Listen -->|Timeout| CheckICMP
    CheckICMP -->|Port Unreachable| Classify
    CheckICMP -->|No Error| Classify
    Classify -->|More Ports| Loop
    Classify -->|Done| Result[Return Results]
    
    Note1[Open|Filtered distinction<br/>often difficult]
```

**Features:**
- UDP packet sending
- ICMP error detection
- Protocol-specific probes
- Timeout-based detection

## Plugin Configuration

### Configuration Structure

```mermaid
graph TB
    subgraph "Global Config"
        Timeout[Global Timeout]
        Concurrency[Concurrency Level]
        LogLevel[Log Level]
    end
    
    subgraph "Plugin-Specific Config"
        PingCfg[Ping Config<br/>- icmp_timeout<br/>- fallback_tcp]
        TCPCfg[TCP Config<br/>- port_range<br/>- banner_grab]
        SYNCfg[SYN Config<br/>- port_range<br/>- wait_time]
        UDPCfg[UDP Config<br/>- port_range<br/>- protocol_probes]
    end
    
    Timeout -.default.-> PingCfg
    Timeout -.default.-> TCPCfg
    Timeout -.default.-> SYNCfg
    Timeout -.default.-> UDPCfg
```

### Configuration Inheritance

- Global settings provide defaults
- Plugin-specific config overrides globals
- CLI arguments override all config
- Validation ensures compatibility

## Plugin Error Handling

```mermaid
graph TB
    subgraph "Error Types"
        NetErr[Network Error]
        ConfigErr[Configuration Error]
        PrivErr[Privilege Error]
        TimeoutErr[Timeout Error]
        SystemErr[System Error]
    end
    
    subgraph "Error Handling"
        Classify[Classify Error]
        Log[Log Error]
        Retry[Retry Logic]
        Report[Report to User]
    end
    
    NetErr --> Classify
    ConfigErr --> Classify
    PrivErr --> Classify
    TimeoutErr --> Classify
    SystemErr --> Classify
    
    Classify -->|Retryable| Retry
    Classify -->|Fatal| Log
    Retry -->|Success| Result[Return Result]
    Retry -->|Failed| Log
    Log --> Report
    Report --> Result
```

### Error Categories

**Retryable Errors:**
- Temporary network failures
- DNS resolution timeouts
- Connection resets

**Fatal Errors:**
- Missing privileges
- Invalid configuration
- System resource exhaustion
- Invalid target format

## Plugin Result Format

### ScanResult Structure

```rust
pub struct ScanResult {
    pub target: String,           // IP or hostname
    pub scan_type: String,         // Type of scan performed
    pub status: Status,            // Overall status
    pub details: HashMap<String, Value>, // Plugin-specific data
    pub timestamp: DateTime<Utc>,  // When scan started
    pub duration: Duration,        // How long scan took
    pub error: Option<String>,     // Error message if failed
}
```

### Details Field Examples

**Ping Scan:**
```json
{
    "alive": true,
    "latency_ms": 12.5,
    "method": "icmp"
}
```

**TCP Connect Scan:**
```json
{
    "open_ports": [22, 80, 443],
    "closed_ports": [25, 3306],
    "filtered_ports": [],
    "banners": {
        "80": "Apache/2.4.41"
    }
}
```

**Service Detection:**
```json
{
    "port": 80,
    "service": "http",
    "product": "Apache httpd",
    "version": "2.4.41",
    "os_cpe": "cpe:/o:linux:linux_kernel"
}
```

## Plugin Testing Framework

### Test Structure

```mermaid
graph TB
    subgraph "Test Types"
        Unit[Unit Tests<br/>- Mock network<br/>- Test logic]
        Integration[Integration Tests<br/>- Real network<br/>- Local targets]
        Performance[Performance Tests<br/>- Benchmark scans<br/>- Resource usage]
    end
    
    subgraph "Test Utilities"
        MockNet[Network Mocking]
        TestTargets[Test Target Generator]
        Assertions[Result Assertions]
    end
    
    Unit --> MockNet
    Unit --> Assertions
    Integration --> TestTargets
    Integration --> Assertions
    Performance --> TestTargets
```

### Mock Target System

```rust
// Example test setup
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ping_scan_success() {
        let plugin = PingScanPlugin::new();
        let result = plugin.scan("127.0.0.1").await.unwrap();
        
        assert_eq!(result.status, Status::Success);
        assert_eq!(result.scan_type, "ping");
        assert!(result.details.contains_key("alive"));
    }
}
```

## Plugin Development Guide

### Creating a New Plugin

```mermaid
graph TB
    Start[Start] --> Define[Define Plugin Struct]
    Define --> Implement[Implement Plugin Trait]
    Implement --> Config[Add Configuration]
    Config --> Logic[Implement Scan Logic]
    Logic --> Test[Write Tests]
    Test --> Doc[Write Documentation]
    Doc --> Register[Register Plugin]
    Register --> End[End]
```

### Plugin Checklist

- [ ] Define plugin struct with necessary fields
- [ ] Implement all required Plugin trait methods
- [ ] Handle errors gracefully
- [ ] Support async operations
- [ ] Add configuration validation
- [ ] Implement timeout handling
- [ ] Write unit tests
- [ ] Write integration tests
- [ ] Document configuration options
- [ ] Document required privileges
- [ ] Add usage examples
- [ ] Register with PluginManager

## Security Considerations

### Privilege Management

```mermaid
graph TB
    subgraph "Privilege Levels"
        None[None<br/>User-space only]
        CapNetRaw[CAP_NET_RAW<br/>Raw sockets]
        CapNetAdmin[CAP_NET_ADMIN<br/>Network admin]
        Root[Root<br/>Full access]
    end
    
    subgraph "Plugins"
        TCP[TCP Connect<br/>None]
        Ping[Ping Scan<br/>CAP_NET_RAW]
        SYN[SYN Scan<br/>CAP_NET_RAW]
        UDP[UDP Scan<br/>None]
    end
    
    None -.requires.-> TCP
    None -.requires.-> UDP
    CapNetRaw -.requires.-> Ping
    CapNetRaw -.requires.-> SYN
```

### Input Validation

All plugins must validate:
- Target format (IP, hostname, range)
- Port ranges (1-65535)
- Configuration values
- Network responses

### Resource Limits

Plugins must respect:
- Timeout configurations
- Memory limits
- File descriptor limits
- Network bandwidth limits

## Performance Optimization

### Concurrency

```mermaid
graph LR
    subgraph "Single Target"
        T1[Target] --> P1[Plugin 1]
        T1 --> P2[Plugin 2]
        T1 --> P3[Plugin 3]
    end
    
    subgraph "Multiple Targets"
        Scanner[Scanner] --> T1[Target 1]
        Scanner --> T2[Target 2]
        Scanner --> T3[Target 3]
    end
    
    Note1[Parallel plugin execution<br/>per target]
    Note2[Concurrent target scanning]
```

### Optimization Strategies

1. **Connection Pooling**: Reuse connections where possible
2. **Batching**: Group operations for efficiency
3. **Caching**: Cache DNS lookups and results
4. **Early Exit**: Stop on definitive results
5. **Resource Sharing**: Share resources between plugins

## Future Enhancements

### Planned Features

1. **Dynamic Plugin Loading**
   - Load plugins from shared libraries
   - Hot-reload capabilities
   - Plugin versioning

2. **Plugin Marketplace**
   - Plugin discovery service
   - Third-party plugin repository
   - Plugin ratings and reviews

3. **Advanced Plugin Types**
   - Credential testing plugins
   - Vulnerability scanning plugins
   - Compliance checking plugins

4. **Plugin Composition**
   - Chain multiple plugins
   - Conditional execution
   - Plugin dependencies

## Summary

The Huginn plugin system provides:

- **Type Safety**: Rust trait system ensures correctness
- **Flexibility**: Easy to add new scan types
- **Performance**: Async I/O for high throughput
- **Security**: Privilege management and input validation
- **Testability**: Comprehensive testing framework
- **Documentation**: Clear interfaces and examples

This architecture enables both built-in and custom plugins while maintaining safety, performance, and maintainability.
