# ipcalc - IP Address Calculator & Subnet Tools

> A powerful, feature-rich IP address calculator and subnet mask calculator written in Rust. Calculate subnets, CIDR notation, VLSM, DHCP scopes, and more for IPv4 and IPv6 networks.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.95+-orange.svg)](https://www.rust-lang.org)
[![Stars](https://img.shields.io/github/stars/cool0looc/ipcalc?style=social)](https://github.com/cool0looc/ipcalc)
[![Downloads](https://img.shields.io/github/downloads/cool0looc/ipcalc/total.svg)](https://github.com/cool0looc/ipcalc/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/cool0looc/ipcalc/release.yml)](https://github.com/cool0looc/ipcalc/actions)

<!-- TOC -->
- [ipcalc - IP Address Calculator & Subnet Tools](#ipcalc---ip-address-calculator--subnet-tools)
  - [✨ Features](#-features)
  - [🚀 Quick Start](#-quick-start)
  - [📦 Installation](#-installation)
  - [📖 What is ipcalc?](#-what-is-ipcalc)
  - [📚 Commands Reference](#-commands-reference)
    - [subnet - Subnet Calculator](#subnet---subnet-calculator)
    - [vlsm - VLSM Calculator](#vlsm---vlsm-calculator)
    - [divide - Divide Network](#divide---divide-network)
    - [summarize - CIDR Aggregation](#summarize---cidr-aggregation)
    - [expand - IP Range to CIDR](#expand---ip-range-to-cidr)
    - [private - Private IP Checker](#private---private-ip-checker)
    - [classify - IP Class](#classify---ip-class)
    - [convert - IP Format Converter](#convert---ip-format-converter)
    - [lookup - Network Lookup](#lookup---network-lookup)
    - [conflict - Conflict Detection](#conflict---conflict-detection)
    - [visualize - Network Tree View](#visualize---network-tree-view)
    - [plan - Subnet Planning](#plan---subnet-planning)
    - [dhcp - DHCP Planning](#dhcp---dhcp-planning)
  - [💡 Common Use Cases](#-common-use-cases)
    - [Network Administration](#network-administration)
    - [Subnet Planning](#subnet-planning)
    - [IP Range Conversion](#ip-range-conversion)
    - [Security & Compliance](#security--compliance)
    - [DHCP Planning](#dhcp-planning)
  - [📊 Output Formats](#-output-formats)
  - [🔧 Batch Processing](#-batch-processing)
  - [🏗️ Architecture](#️-architecture)
  - [🤝 Contributing](#-contributing)
  - [📄 License](#-license)
  - [❓ FAQ](#-faq)
  - [📞 Support](#-support)

<!-- /TOC -->

## ✨ Features

- **IPv4 & IPv6 Support** - Full support for both IP protocol versions
- **15+ Commands** - Comprehensive IP address calculation toolkit
- **Subnet Calculator** - Calculate network address, broadcast, subnet mask, wildcard mask, usable hosts
- **VLSM Calculator** - Variable Length Subnet Masking for efficient IP allocation
- **CIDR Calculator** - CIDR notation calculations and aggregation
- **DHCP Planning** - DHCP scope planning with reservations and exclusions
- **Multiple Output Formats** - Human-readable, JSON, YAML, CSV output
- **Batch Processing** - Process multiple IP addresses or networks at once
- **Network Visualization** - ASCII tree visualization of network hierarchy
- **Conflict Detection** - Detect IP address overlaps between networks

## 🚀 Quick Start

```bash
# Calculate subnet information (most common use)
ipcalc subnet 192.168.1.0/24

# Check if IP is private or public
ipcalc private 10.0.0.1

# Divide network into subnets
ipcalc divide 192.168.1.0/24 4

# VLSM allocation for multiple VLANs
ipcalc vlsm 192.168.1.0/24 100,50,25,10

# Aggregate multiple CIDR blocks
ipcalc summarize 192.168.1.0/24 192.168.2.0/24

# Convert IP range to CIDR
ipcalc expand 192.168.1.1-192.168.1.254
```

## 📦 Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/cool0looc/ipcalc.git
cd ipcalc

# Build release version
cargo build --release

# Install globally
cargo install --path .
```

### Pre-built Binaries

Download from the [Releases](https://github.com/cool0looc/ipcalc/releases) page:

| Platform | Download |
|----------|----------|
| Linux (x86_64) | ipcalc-linux-x86_64 |
| macOS (Apple Silicon) | ipcalc-macos-aarch64 |
| macOS (Intel) | ipcalc-macos-x86_64 |
| Windows | ipcalc-windows.exe |

## 📖 What is ipcalc?

**ipcalc** is a free, open-source IP address calculator and subnet calculator written in Rust. It provides network engineers, system administrators, and IT professionals with a comprehensive toolkit for:

- **Subnetting**: Calculate network boundaries, broadcast addresses, and usable host ranges
- **VLSM (Variable Length Subnet Masking)**: Efficiently allocate IP addresses based on department/VLAN requirements
- **CIDR (Classless Inter-Domain Routing)**: Work with CIDR notation and aggregate CIDR blocks
- **DHCP Planning**: Plan DHCP scopes with appropriate exclusion ranges and reservations
- **IP Validation**: Validate IP addresses and CIDR notation

### Why ipcalc?

| Feature | ipcalc | Online Calculators | Other CLI Tools |
|---------|--------|-------------------|-----------------|
| No internet required | ✅ | ❌ | ✅ |
| Scriptable/API | ✅ | ❌ | Limited |
| VLSM support | ✅ | Limited | Limited |
| IPv6 support | ✅ | Variable | Limited |
| Output formats | JSON/YAML/CSV | HTML only | Text only |
| Cross-platform | ✅ | Browser only | Variable |
| Open source | ✅ | ❌ | Variable |

## 📚 Commands Reference

### subnet - Subnet Calculator

Calculate detailed subnet information including network address, broadcast, netmask, wildcard mask, first/last usable IP, and total/usable host count.

```bash
# Basic subnet calculation
ipcalc subnet 192.168.1.0/24

# Calculate multiple subnets at once
ipcalc subnet 192.168.1.0/24 10.0.0.0/8 172.16.0.0/16

# Show all subnet details
ipcalc subnet --show-all 192.168.1.0/24
```

**Example Output:**
```
Network:      192.168.1.0/24
Netmask:      255.255.255.0
Wildcard:     0.0.0.255
Network:      192.168.1.0
Broadcast:    192.168.1.255
First IP:     192.168.1.1
Last IP:      192.168.1.254
Total IPs:    256
Usable Hosts: 254
```

**Options:**
- `--show-all, -s` - Show all subnet details

---

### vlsm - VLSM Calculator

Variable Length Subnet Masking (VLSM) calculator. Allocates subnets based on host requirements for efficient IP address utilization.

```bash
# VLSM calculation for network design
ipcalc vlsm 192.168.1.0/24 100,50,25,10

# Large network VLSM planning
ipcalc vlsm 10.0.0.0/8 10000,5000,2000,1000,500,100
```

**Example Output:**
```
Network:       172.16.0.0/12
Available:     1,048,574 hosts

Allocations (sorted by size):
  #1:  172.16.0.0/19   [8,190 hosts]  for VLAN_100
  #2:  172.16.32.0/20  [4,094 hosts]  for VLAN_200
  #3:  172.16.48.0/21  [2,046 hosts]  for VLAN_300
  #4:  172.16.56.0/22  [1,022 hosts]  for VLAN_400
  #5:  172.16.60.0/23  [510 hosts]     for VLAN_500
  #6:  172.16.62.0/24  [254 hosts]     for VLAN_600

Remaining:     34,302 hosts (unallocated)
```

**Options:**
- `REQUIREMENTS...` - Host requirements (comma-separated or multiple arguments)

---

### divide - Divide Network

Divide a network into equal-sized subnets.

```bash
# Divide into 4 equal subnets
ipcalc divide 192.168.1.0/24 4

# Divide into 8 subnets
ipcalc divide 10.0.0.0/8 256

# Custom subdivision
ipcalc divide 172.16.0.0/16 16
```

**Example Output:**
```
Parent Network:  192.168.1.0/24
Subnets:        4
Prefix:         /26 (255.255.255.192)

Subnets:
  #1:  192.168.1.0/26    (0-63)    [64 IPs, 62 usable]
  #2:  192.168.1.64/26   (64-127)  [64 IPs, 62 usable]
  #3:  192.168.1.128/26  (128-191) [64 IPs, 62 usable]
  #4:  192.168.1.192/26  (192-255) [64 IPs, 62 usable]
```

---

### summarize - CIDR Aggregation

Aggregate multiple CIDR blocks into supernets (CIDR consolidation).

```bash
# Aggregate CIDR blocks
ipcalc summarize 192.168.1.0/24 192.168.2.0/24

# Multiple network aggregation
ipcalc summarize 10.0.0.0/24 10.0.1.0/24 10.0.2.0/24 10.0.3.0/24
```

**Example Output:**
```
Original Networks:
  192.168.1.0/24
  192.168.2.0/24

Aggregated Result:
  192.168.1.0/23
```

---

### expand - IP Range to CIDR

Convert IP ranges to CIDR notation using RFC 4632 optimal aggregation.

```bash
# Convert IP range to CIDR blocks
ipcalc expand 192.168.1.1-192.168.1.254

# Full network range
ipcalc expand 192.168.1.0-192.168.1.255

# Range across subnet boundaries
ipcalc expand 192.168.1.128-192.168.2.127
```

**Options:**
- `--min-prefix, -n` - Minimum prefix length (default: 8)
- `--max-prefix, -m` - Maximum prefix length (default: 32)

---

### private - Private IP Checker

Check if an IP address is private or public according to RFC standards.

```bash
ipcalc private 10.0.0.1      # Private (RFC1918)
ipcalc private 172.16.0.1   # Private (RFC1918)
ipcalc private 192.168.1.1  # Private (RFC1918)
ipcalc private 8.8.8.8      # Public IP
ipcalc private 127.0.0.1    # Loopback
```

**Output Example:**
```
Address:     10.0.0.1
Type:        Private IP address
Reference:   RFC1918
Status:      Private
```

**Special Address Types Detected:**
- **Private (RFC1918)**: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
- **Loopback (RFC1122)**: 127.0.0.0/8
- **Link-Local (RFC3927)**: 169.254.0.0/16
- **Multicast (RFC5771)**: 224.0.0.0/4
- **CGNAT (RFC6598)**: 100.64.0.0/10

---

### classify - IP Class

Classify IP addresses into traditional A/B/C/D/E classes.

```bash
ipcalc classify 10.0.0.1
ipcalc classify 172.16.0.1
ipcalc classify 192.168.1.1
ipcalc classify 224.0.0.1  # Multicast
```

**IP Classes:**
| Class | Range | Default Mask | CIDR |
|-------|-------|--------------|------|
| Class A | 0.0.0.0 - 127.255.255.255 | 255.0.0.0 | /8 |
| Class B | 128.0.0.0 - 191.255.255.255 | 255.255.0.0 | /16 |
| Class C | 192.0.0.0 - 223.255.255.255 | 255.255.255.0 | /24 |
| Class D | 224.0.0.0 - 239.255.255.255 | N/A | Multicast |
| Class E | 240.0.0.0 - 255.255.255.255 | N/A | Reserved |

---

### convert - IP Format Converter

Convert IP addresses between different formats.

```bash
# Convert to integer
ipcalc convert 192.168.1.1

# Convert to binary
ipcalc convert 192.168.1.1 --to-format binary

# Convert to hexadecimal
ipcalc convert 192.168.1.1 --to-format hex

# Convert integer to dotted notation
ipcalc convert 3232235777 --to-format dotted
```

**Supported Formats:**
- `dotted` - Dotted decimal (192.168.1.1)
- `integer` - Integer (3232235777)
- `binary` - Binary (11000000.10101000.00000001.00000001)
- `hex` - Hexadecimal (0xC0A80101)

---

### lookup - Network Lookup

Lookup information about well-known network ranges and RFC designations.

```bash
# List all available networks
ipcalc lookup --list

# Lookup specific RFC networks
ipcalc lookup RFC1918   # Private addresses
ipcalc lookup LOOPBACK  # 127.0.0.0/8
ipcalc lookup MULTICAST # 224.0.0.0/4
ipcalc lookup CGNAT     # 100.64.0.0/10
```

**Available Networks:**
- RFC1918 - Private Addresses (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
- LOOPBACK - Loopback (127.0.0.0/8)
- LINKLOCAL - Link-Local (169.254.0.0/16)
- MULTICAST - Multicast (224.0.0.0/4)
- TEST-NET-1/2/3 - Documentation ranges
- CGNAT - Carrier-Grade NAT (100.64.0.0/10)
- BENCHMARK - Benchmark Testing (198.18.0.0/15)

---

### conflict - Conflict Detection

Detect IP address conflicts (overlaps) between multiple networks.

```bash
# Check for network conflicts
ipcalc conflict 192.168.1.0/24 192.168.1.128/25

# Verify no overlaps in network design
ipcalc conflict 10.0.0.0/8 172.16.0.0/12 192.168.0.0/16
```

**Output (No Conflicts):**
```
✓ No conflicts detected!
```

**Output (Conflicts Found):**
```
✗ Found 1 conflict!

Conflict #1: A contains B
Networks: 192.168.1.0/24, 192.168.1.128/25
Overlap: 192.168.1.128 - 192.168.1.255
```

---

### visualize - Network Tree View

Visualize network hierarchy as ASCII tree diagram.

```bash
# Basic network visualization
ipcalc visualize 192.168.1.0/24

# Visualize with depth limit
ipcalc visualize 10.0.0.0/8 --depth 2

# Larger network tree
ipcalc visualize 172.16.0.0/12 --depth 3
```

**Example Output:**
```
Network: 192.168.1.0/24
│
├── Network: 192.168.1.0/25 (128 hosts)
│   ├── Network: 192.168.1.0/26 (64 hosts)
│   │   ├── Network: 192.168.1.0/27 (32 hosts)
│   │   └── Network: 192.168.1.32/27 (32 hosts)
│   └── Network: 192.168.1.64/26 (64 hosts)
└── Network: 192.168.1.128/25 (128 hosts)
    └── Network: 192.168.1.128/26 (64 hosts)
```

---

### plan - Subnet Planning

Intelligent subnet planning with recommendations and feasibility ratings.

```bash
# Basic network planning
ipcalc plan 192.168.1.0/24

# Plan with specific requirements
ipcalc plan 192.168.1.0/24 100 50 25

# Large network subnet design
ipcalc plan 10.0.0.0/8 10000 5000 2000
```

**Example Output:**
```
Network: 192.168.1.0/24
Available: 254 hosts

Subnetting Suggestions:

1. Division: 4 subnets of 62 hosts each
   Networks: 192.168.1.0/26, 192.168.1.64/26, 192.168.1.128/26, 192.168.1.192/26
   Feasibility: ✓ Recommended
   
2. Division: 2 subnets of 126 hosts each
   Networks: 192.168.1.0/25, 192.168.1.128/25
   Feasibility: ✓ Recommended
```

---

### dhcp - DHCP Planning

DHCP scope planning with static reservations and exclusion ranges.

```bash
# Basic DHCP scope planning
ipcalc dhcp 192.168.1.0/24

# Plan with reservations
ipcalc dhcp 192.168.1.0/24 --reservations 10

# Plan with exclusions
ipcalc dhcp 192.168.1.0/24 --exclusions 5

# Complete DHCP planning
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5
```

**Example Output:**
```
DHCP Scope Planning for 192.168.1.0/24
============================================================

DHCP Scope:
  Network:    192.168.1.0/24
  Subnet:     255.255.255.0
  Range:      192.168.1.1 - 192.168.1.254
  Total IPs:  254

------------------------------------------------------------
Recommended Static Reservations:
  1: 192.168.1.1 (Gateway/Router)
  2: 192.168.1.2 (Primary DNS)
  3: 192.168.1.3 (Secondary DNS)

------------------------------------------------------------
Recommended Exclusions:
  1. 192.168.1.1 - 192.168.1.10 [Infrastructure]
  2. 192.168.1.11 - 192.168.1.30 [Static assignment]
  3. 192.168.1.200 - 192.168.1.220 [Reserved]

------------------------------------------------------------
DHCP Pool Summary:
  Total IPs in subnet: 254
  Static reservations:  3
  Excluded addresses:   40
  Available for DHCP:   211
```

---

### validate - IP Validator

Validate IP addresses or CIDR notation.

```bash
# Validate single IP
ipcalc validate 192.168.1.1

# Validate CIDR notation
ipcalc validate 192.168.1.0/24

# Validate multiple addresses
ipcalc validate 10.0.0.0/8 172.16.0.1 192.168.1.0/16

# Strict mode - fails on any invalid input
ipcalc validate --strict 192.168.1.0/24 invalid_ip
```

---

### range - IP Range Calculator

Show the IP range (start and end addresses) for CIDR notation.

```bash
ipcalc range 192.168.1.0/24
ipcalc range 10.0.0.0/8
```

**Example Output:**
```
CIDR:         192.168.1.0/24
Start:        192.168.1.0 (Network)
First Host:   192.168.1.1
Last Host:    192.168.1.254
End:          192.168.1.255 (Broadcast)
```

---

## 💡 Common Use Cases

### Network Administration

```bash
# Validate IP configuration
ipcalc validate 192.168.1.100/26

# Calculate subnet details
ipcalc subnet 192.168.1.0/26

# Check available hosts
ipcalc subnet 10.0.0.0/8 --show-all
```

### Subnet Planning

```bash
# Plan for new department (need 100 hosts)
ipcalc plan 192.168.0.0/16 100

# Divide network into subnets
ipcalc divide 192.168.1.0/24 8

# VLSM allocation for enterprise network
ipcalc vlsm 10.0.0.0/8 10000,5000,2000,1000,500
```

### IP Range Conversion

```bash
# Convert IP range to CIDR blocks
ipcalc expand 192.168.1.1-192.168.1.254

# Convert range across subnet boundaries
ipcalc expand 192.168.0.0-192.168.3.255

# Aggregate CIDR for route summarization
ipcalc summarize 192.168.0.0/24 192.168.1.0/24 192.168.2.0/24 192.168.3.0/24
```

### Security & Compliance

```bash
# Check for private vs public IP
ipcalc private 10.0.0.1 172.16.0.1 192.168.1.1 8.8.8.8

# Lookup special network ranges
ipcalc lookup RFC1918

# Detect IP conflicts
ipcalc conflict 192.168.1.0/24 192.168.1.128/25
```

### DHCP Planning

```bash
# Plan DHCP scope for office network
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5

# Plan for large enterprise network
ipcalc dhcp 10.0.0.0/8 --reservations 50 --exclusions 20
```

---

## 📊 Output Formats

All commands support multiple output formats: human-readable (default), JSON, YAML, and CSV.

```bash
# Human-readable output (default)
ipcalc subnet 192.168.1.0/24

# JSON output for scripting
ipcalc --format json subnet 192.168.1.0/24

# YAML output
ipcalc --format yaml subnet 192.168.1.0/24

# CSV output for data processing
ipcalc --format csv subnet 192.168.1.0/24
```

**Note:** The format option must come before the subcommand.

---

## 🔧 Batch Processing

Process multiple IP addresses or networks from a file.

```bash
# Create input file with IP addresses/CIDRs
cat > ips.txt << EOF
192.168.1.0/24
10.0.0.0/8
172.16.0.0/12
EOF

# Process from file
ipcalc subnet --file ips.txt

# Batch processing with JSON output
ipcalc --format json subnet --file ips.txt
```

---

## 🏗️ Architecture

```
ipcalc/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports
│   ├── cli/              # CLI commands (15+ subcommands)
│   │   ├── validate.rs   # IP/CIDR validation
│   │   ├── subnet.rs     # Subnet calculation
│   │   ├── range.rs      # IP range calculation
│   │   ├── expand.rs     # IP range to CIDR
│   │   ├── summarize.rs  # CIDR aggregation
│   │   ├── divide.rs     # Network division
│   │   ├── vlsm.rs       # VLSM calculator
│   │   ├── classify.rs   # IP classification
│   │   ├── private.rs    # Private IP detection
│   │   ├── convert.rs    # Format conversion
│   │   ├── lookup.rs     # Network lookup
│   │   ├── visualize.rs  # Network visualization
│   │   ├── conflict.rs   # Conflict detection
│   │   ├── plan.rs       # Subnet planning
│   │   └── dhcp.rs       # DHCP planning
│   ├── core/             # Core IP calculation logic
│   │   ├── ipv4/         # IPv4 implementation
│   │   ├── ipv6/         # IPv6 implementation
│   │   └── cidr/         # CIDR operations
│   ├── formats/          # Output formatters (JSON/YAML/CSV)
│   └── utils/            # Utilities
├── tests/                # Integration tests
├── Cargo.toml
└── README.md
```

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## ❓ FAQ

**Q: What is ipcalc used for?**

A: ipcalc is an IP address calculator and subnet mask calculator used for network planning, subnetting, and IP address management. It helps network administrators calculate subnets, plan VLSM allocations, and manage IP address spaces.

**Q: How do I calculate a subnet?**

A: Use the `subnet` command:
```bash
ipcalc subnet 192.168.1.0/24
```

**Q: What is the difference between ipcalc and an online IP calculator?**

A: ipcalc is a command-line tool that works offline, supports scripting, multiple output formats (JSON/YAML/CSV), and is designed for automation and integration into network management workflows.

**Q: Does ipcalc support IPv6?**

A: Yes, ipcalc supports both IPv4 and IPv6 address calculations.

**Q: What is VLSM?**

A: VLSM (Variable Length Subnet Masking) allows different subnets within the same network to have different subnet masks, enabling more efficient IP address allocation. Use `ipcalc vlsm` to calculate VLSM subnets.

**Q: How do I convert an IP range to CIDR?**

A: Use the `expand` command:
```bash
ipcalc expand 192.168.1.1-192.168.1.254
```

**Q: Can I use ipcalc for DHCP planning?**

A: Yes, the `dhcp` command helps plan DHCP scopes with appropriate exclusion ranges and static reservations:
```bash
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5
```

**Q: Is ipcalc free to use?**

A: Yes, ipcalc is open-source software released under the MIT License.

---

## 📞 Support

If you encounter any issues or have questions:

- Open an issue on [GitHub](https://github.com/cool0looc/ipcalc/issues)
- Check the [Wiki](https://github.com/cool0looc/ipcalc/wiki) for additional documentation

---

**Made with ❤️ and [Rust](https://www.rust-lang.org)** | **Star ⭐ if you find it useful!**
