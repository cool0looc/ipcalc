# ipcalc

A powerful, feature-rich IP address calculator written in Rust for IPv4 and IPv6 networks.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)

## Features

- **IPv4 & IPv6 Support** - Full support for both IP protocol versions
- **15+ Commands** - Comprehensive toolkit for IP address calculations
- **Multiple Output Formats** - Human-readable, JSON, YAML, CSV
- **Batch Processing** - Process multiple IPs or networks at once
- **CIDR Aggregation** - Combine multiple CIDR blocks into supernets
- **VLSM Calculator** - Variable Length Subnet Masking calculations
- **DHCP Planning** - DHCP scope planning with reservations
- **Conflict Detection** - Detect IP address conflicts between networks
- **Subnet Planning** - Intelligent subnet planning recommendations
- **Network Visualization** - ASCII tree visualization of network hierarchy

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/cool0looc/ipcalc.git
cd ipcalc

# Build release version
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Binary Release

Download pre-built binaries from the [Releases](https://github.com/cool0looc/ipcalc/releases) page.

## Quick Start

```bash
# Calculate subnet information
ipcalc subnet 192.168.1.0/24

# Convert IP range to CIDR
ipcalc expand 192.168.1.1-192.168.1.254

# Check if IP is private
ipcalc private 10.0.0.1

# Divide network into subnets
ipcalc divide 192.168.1.0/24 4

# Aggregate multiple CIDRs
ipcalc summarize 192.168.1.0/24 192.168.2.0/24
```

## Commands Reference

### validate

Validate IP addresses or CIDR notation.

```bash
ipcalc validate 192.168.1.1
ipcalc validate 192.168.1.0/24
ipcalc validate 10.0.0.0/8 172.16.0.1 192.168.1.0/16

# Strict mode - fails on any invalid input
ipcalc validate --strict 192.168.1.0/24 invalid_ip
```

**Options:**
- `IP_OR_CIDR...` - IP addresses or CIDRs to validate

---

### subnet

Calculate detailed subnet information including network address, broadcast, netmask, wildcard mask, first/last usable IP, and total/usable host count.

```bash
# Basic usage
ipcalc subnet 192.168.1.0/24

# Multiple networks
ipcalc subnet 192.168.1.0/24 10.0.0.0/8 172.16.0.0/16

# Show all details
ipcalc subnet --show-all 192.168.1.0/24
```

**Output Example:**
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

### range

Show the IP range (start and end addresses) for CIDR notation.

```bash
ipcalc range 192.168.1.0/24
ipcalc range 10.0.0.0/8
ipcalc range 172.16.0.0/12 192.168.0.0/16
```

**Output Example:**
```
CIDR:         192.168.1.0/24
Start:        192.168.1.0 (Network)
First Host:   192.168.1.1
Last Host:    192.168.1.254
End:          192.168.1.255 (Broadcast)
```

---

### expand (IP Range to CIDR)

Convert IP ranges to CIDR notation. This finds the minimum CIDR that contains the entire range.

```bash
# Basic usage
ipcalc expand 192.168.1.1-192.168.1.254

# Full network
ipcalc expand 192.168.1.0-192.168.1.255

# Range across subnet boundaries
ipcalc expand 192.168.1.128-192.168.2.127

# Custom prefix limits
ipcalc expand 192.168.1.1-192.168.1.30 --min-prefix 28 --max-prefix 30
```

**Output Example:**
```
IP Range:     192.168.1.1-192.168.1.254
Total IPs:   254 (+2 reserved)

CIDR Blocks:
  192.168.1.0/24  (192.168.1.1 - 192.168.1.254)  [256 IPs, 254 usable]
```

**Options:**
- `--min-prefix, -n` - Minimum prefix length (default: 8)
- `--max-prefix, -m` - Maximum prefix length (default: 32)

**Algorithm:** Uses RFC 4632 optimal CIDR aggregation algorithm to find the smallest set of CIDR blocks that exactly cover the given range.

---

### summarize

Aggregate multiple CIDR blocks into supernets (CIDR consolidation).

```bash
# Basic usage
ipcalc summarize 192.168.1.0/24 192.168.2.0/24

# Multiple networks
ipcalc summarize 10.0.0.0/24 10.0.1.0/24 10.0.2.0/24 10.0.3.0/24

# Complex aggregation
ipcalc summarize 192.168.0.0/24 192.168.1.0/24 192.168.2.0/24 192.168.3.0/24
```

**Output Example:**
```
Original Networks:
  192.168.1.0/24
  192.168.2.0/24

Aggregated Result:
  192.168.1.0/23
```

---

### divide

Divide a network into equal-sized subnets.

```bash
# Divide into 4 subnets
ipcalc divide 192.168.1.0/24 4

# Divide into 8 subnets
ipcalc divide 10.0.0.0/8 256

# Custom number
ipcalc divide 172.16.0.0/16 16
```

**Output Example:**
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

**Options:**
- `CIDR` - Parent network in CIDR notation
- `NUM_SUBNETS` - Number of subnets to create

---

### vlsm

Variable Length Subnet Masking (VLSM) calculator. Allocates subnets based on host requirements.

```bash
# Basic usage with comma-separated values
ipcalc vlsm 192.168.1.0/24 100,50,25,10

# Multiple requirement arguments
ipcalc vlsm 10.0.0.0/8 1000 500 250 100 50

# Large network planning
ipcalc vlsm 172.16.0.0/12 10000,5000,2000,1000,500,100
```

**Output Example:**
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
- `REQUIREMENTS...` - Host requirements (can be comma-separated or multiple arguments)

---

### classify

Classify IP addresses into traditional A/B/C/D/E classes.

```bash
ipcalc classify 10.0.0.1
ipcalc classify 172.16.0.1 192.168.1.1 127.0.0.1
ipcalc classify 240.0.0.1 224.0.0.1
```

**Output Example:**
```
Address:     172.16.0.1
Class:       Class B
Default Mask: 255.255.0.0
CIDR:        /16
Range:       172.16.0.0 - 172.16.255.255
```

**IP Classes:**
- **Class A**: 0.0.0.0 - 127.255.255.255 (/8)
- **Class B**: 128.0.0.0 - 191.255.255.255 (/16)
- **Class C**: 192.0.0.0 - 223.255.255.255 (/24)
- **Class D**: 224.0.0.0 - 239.255.255.255 (Multicast)
- **Class E**: 240.0.0.0 - 255.255.255.255 (Reserved)

---

### private

Check if an IP address is private or public, and identify its type according to RFC standards.

```bash
ipcalc private 10.0.0.1
ipcalc private 172.16.0.1
ipcalc private 192.168.1.1
ipcalc private 8.8.8.8
ipcalc private 127.0.0.1
```

**Output Example:**
```
Address:     10.0.0.1
Type:        Private IP address
Reference:   RFC1918
Status:      Private
```

**Special Address Types:**
- **Private (RFC1918)**: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
- **Loopback (RFC1122)**: 127.0.0.0/8
- **Link-Local (RFC3927)**: 169.254.0.0/16
- **Multicast (RFC5771)**: 224.0.0.0/4
- **Documentation (RFC5737)**: 192.0.2.0/24, 198.51.100.0/24, 203.0.113.0/24
- **CGNAT (RFC6598)**: 100.64.0.0/10

---

### convert

Convert IP addresses between different formats.

```bash
# Convert to integer (default)
ipcalc convert 192.168.1.1

# Convert to binary
ipcalc convert 192.168.1.1 --to-format binary

# Convert to hexadecimal
ipcalc convert 192.168.1.1 --to-format hex

# Convert to dotted notation
ipcalc convert 3232235777 --to-format dotted

# Convert from binary
ipcalc convert 11000000.10101000.00000001.00000001 --to-format dotted
```

**Output Formats:**
- `dotted` - Dotted decimal notation (192.168.1.1)
- `integer` - Integer representation (3232235777)
- `binary` - Binary representation (11000000.10101000.00000001.00000001)
- `hex` - Hexadecimal representation (0xC0A80101)

---

### lookup

Lookup information about well-known network ranges.

```bash
# List all available networks
ipcalc lookup --list

# Lookup specific network
ipcalc lookup RFC1918
ipcalc lookup LOOPBACK
ipcalc lookup MULTICAST
ipcalc lookup DOCUMENTATION
```

**Available Networks:**
- **RFC1918** - Private Addresses (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
- **LOOPBACK** - Loopback (127.0.0.0/8)
- **LINKLOCAL** - Link-Local (169.254.0.0/16)
- **MULTICAST** - Multicast (224.0.0.0/4)
- **TEST-NET-1/2/3** - Documentation ranges
- **DOCUMENTATION** - All documentation ranges
- **CGNAT** - Carrier-Grade NAT (100.64.0.0/10)
- **6TO4** - 6to4 Relay (192.88.99.0/24)
- **BENCHMARK** - Benchmark Testing (198.18.0.0/15)

---

### visualize

Visualize network hierarchy as an ASCII tree diagram.

```bash
# Basic visualization
ipcalc visualize 192.168.1.0/24

# With depth limit
ipcalc visualize 10.0.0.0/8 --depth 2

# Larger network
ipcalc visualize 172.16.0.0/12 --depth 3
```

**Output Example:**
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

Statistics:
  Total Networks: 5
  Total IPs: 256
  Usable Hosts: 254
```

**Options:**
- `--depth, -d` - Maximum tree depth

---

### conflict

Detect IP address conflicts (overlaps) between multiple networks.

```bash
# Check for conflicts
ipcalc conflict 192.168.1.0/24 192.168.1.128/25

# Multiple networks
ipcalc conflict 10.0.0.0/8 172.16.0.0/12 192.168.0.0/16

# Complex scenario
ipcalc conflict 192.168.1.0/24 192.168.1.0/25 192.168.1.128/26
```

**Output Example (No Conflicts):**
```
Checking 3 networks for conflicts...

✓ No conflicts detected!

Networks:
  192.168.1.0/24  (192.168.1.0 - 192.168.1.255) [256 IPs]
  192.168.2.0/24  (192.168.2.0 - 192.168.2.255) [256 IPs]
  192.168.3.0/24  (192.168.3.0 - 192.168.3.255) [256 IPs]
```

**Output Example (Conflicts Found):**
```
Checking 2 networks for conflicts...

✗ Found 1 conflict!

Conflicts:

  Conflict #1: A contains B
  Networks: 192.168.1.0/24, 192.168.1.128/25
  Overlap: 192.168.1.128 - 192.168.1.255

All Networks:
  192.168.1.0/24  (192.168.1.0 - 192.168.1.255) [256 IPs]
  192.168.1.128/25  (192.168.1.128 - 192.168.1.255) [128 IPs]
```

---

### plan

Intelligent subnet planning with recommendations and feasibility ratings.

```bash
# Basic planning
ipcalc plan 192.168.1.0/24

# With expected host count
ipcalc plan 192.168.1.0/24 100 50 25

# Larger network
ipcalc plan 10.0.0.0/8 10000 5000 2000
```

**Output Example:**
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

3. Division: 8 subnets of 30 hosts each
   Networks: 192.168.1.0/27, 192.168.1.32/27, ...
   Feasibility: ⚠ Consider if smaller subnets needed
```

**Options:**
- `CIDR` - Network to plan (CIDR notation)
- `HOST_COUNT...` - Expected number of hosts per subnet

---

### dhcp

DHCP scope planning with static reservations and exclusion ranges.

```bash
# Basic DHCP planning
ipcalc dhcp 192.168.1.0/24

# With reservations
ipcalc dhcp 192.168.1.0/24 --reservations 10

# With exclusions
ipcalc dhcp 192.168.1.0/24 --exclusions 5

# Full planning
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5
```

**Output Example:**
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
(First 3 addresses reserved for infrastructure)

  1: 192.168.1.1 (Gateway/Router)
          MAC: 00:11:22:33:44:01
  2: 192.168.1.2 (Primary DNS)
          MAC: 00:11:22:33:44:02
  3: 192.168.1.3 (Secondary DNS)
          MAC: 00:11:22:33:44:03

------------------------------------------------------------
Recommended Exclusions:
(Addresses excluded from DHCP pool for static assignment)

  1. 192.168.1.1 - 192.168.1.10 [Infrastructure (gateway, DNS, servers)]
  2. 192.168.1.11 - 192.168.1.30 [Static workstations/printers]
  3. 192.168.1.200 - 192.168.1.220 [Reserved for future infrastructure]

------------------------------------------------------------
DHCP Pool Summary:
  Total IPs in subnet: 254
  Static reservations:  3
  Excluded addresses:   40
  Available for DHCP:   211

============================================================
Example isc-dhcp-server Configuration:
  subnet 192.168.1.0 netmask 255.255.255.0 {
    range 192.168.1.31 192.168.1.199;
    option routers 192.168.1.1;
    # Add exclusions and reservations above
  }
```

**Options:**
- `--reservations, -r` - Number of static reservations to plan
- `--exclusions, -e` - Number of exclusion ranges

---

## Output Formats

All commands support multiple output formats using the `--format` option.

```bash
# Human-readable (default)
ipcalc subnet 192.168.1.0/24

# JSON output
ipcalc subnet 192.168.1.0/24 --format json

# YAML output
ipcalc subnet 192.168.1.0/24 --format yaml

# CSV output
ipcalc subnet 192.168.1.0/24 --format csv
```

**Format Options:**
- `human` - Human-readable output (default)
- `json` - JSON formatted output
- `yaml` - YAML formatted output
- `csv` - CSV formatted output

---

## Batch Processing

Process multiple IPs or networks from a file.

```bash
# Create input file
cat > ips.txt << EOF
192.168.1.0/24
10.0.0.0/8
172.16.0.0/12
EOF

# Process from file
ipcalc subnet --file ips.txt

# With format
ipcalc subnet --file ips.txt --format json
```

---

## Common Use Cases

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

# Divide network
ipcalc divide 192.168.1.0/24 8

# VLSM allocation
ipcalc vlsm 10.0.0.0/8 10000,5000,2000,1000,500
```

### IP Range Conversion

```bash
# Convert IP range to CIDR
ipcalc expand 192.168.1.1-192.168.1.254

# Convert range across boundaries
ipcalc expand 192.168.0.0-192.168.3.255
```

### Security & Compliance

```bash
# Check private IPs
ipcalc private 10.0.0.1 172.16.0.1 192.168.1.1 8.8.8.8

# Lookup network types
ipcalc lookup RFC1918

# Detect conflicts
ipcalc conflict 192.168.1.0/24 192.168.1.128/25
```

### DHCP Planning

```bash
# Plan DHCP scope
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5

# Plan for large network
ipcalc dhcp 10.0.0.0/8 --reservations 50 --exclusions 20
```

---

## Global Options

- `--format, -f` - Output format (human, json, yaml, csv)
- `--strict` - Strict mode: fail on any invalid input
- `--file, -i` - Input file for batch processing
- `--verbose, -v` - Show verbose output
- `--help, -h` - Show help information
- `--version, -V` - Show version information

---

## Architecture

```
ipcalc/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports
│   ├── cli/              # CLI commands
│   │   ├── mod.rs
│   │   ├── validate.rs
│   │   ├── subnet.rs
│   │   ├── range.rs
│   │   ├── expand.rs
│   │   ├── summarize.rs
│   │   ├── divide.rs
│   │   ├── vlsm.rs
│   │   ├── classify.rs
│   │   ├── private.rs
│   │   ├── convert.rs
│   │   ├── lookup.rs
│   │   ├── visualize.rs
│   │   ├── conflict.rs
│   │   ├── plan.rs
│   │   └── dhcp.rs
│   ├── core/             # Core IP calculations
│   │   ├── mod.rs
│   │   ├── ipv4/         # IPv4 implementation
│   │   │   ├── address.rs
│   │   │   ├── network.rs
│   │   │   ├── classify.rs
│   │   │   └── private.rs
│   │   ├── ipv6/         # IPv6 implementation
│   │   │   ├── address.rs
│   │   │   └── network.rs
│   │   ├── cidr/         # CIDR operations
│   │   │   ├── aggregate.rs
│   │   │   └── collapse.rs
│   │   └── vlsm/         # VLSM calculations
│   ├── formats/          # Output formatters
│   │   ├── mod.rs
│   │   ├── json.rs
│   │   ├── yaml.rs
│   │   ├── csv.rs
│   │   └── human.rs
│   └── utils/            # Utility functions
│       ├── mod.rs
│       ├── validation.rs
│       ├── parsing.rs
│       └── constants.rs
├── tests/                # Integration tests
├── Cargo.toml
└── README.md
```

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Inspired by the classic `ipcalc` tool
- Built with Rust for performance and safety
- Thanks to all contributors

---

## Support

If you encounter any issues or have questions:

- Open an issue on [GitHub](https://github.com/cool0looc/ipcalc/issues)
- Check the [Wiki](https://github.com/cool0looc/ipcalc/wiki) for additional documentation

---

**Made with ❤️ and Rust**
