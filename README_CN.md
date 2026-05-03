# ipcalc - IP地址计算器和子网工具

> 功能强大、特性丰富的 IP 地址计算器和子网掩码计算器，使用 Rust 编写。支持 IPv4 和 IPv6 网络的子网计算、CIDR 标记、VLSM、DHCP 作用域规划等功能。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Stars](https://img.shields.io/github/stars/cool0looc/ipcalc?style=social)](https://github.com/cool0looc/ipcalc)
[![Downloads](https://img.shields.io/github/downloads/cool0looc/ipcalc/total.svg)](https://github.com/cool0looc/ipcalc/releases)

<!-- 目录 -->
- [ipcalc - IP地址计算器和子网工具](#ipcalc---ip地址计算器和子网工具)
  - [✨ 功能特性](#-功能特性)
  - [🚀 快速开始](#-快速开始)
  - [📦 安装方式](#-安装方式)
  - [📖 什么是 ipcalc？](#-什么是-ipcalc)
  - [📚 命令参考](#-命令参考)
    - [subnet - 子网计算器](#subnet---子网计算器)
    - [vlsm - VLSM计算器](#vlsm---vlsm计算器)
    - [divide - 网络分割](#divide---网络分割)
    - [summarize - CIDR聚合](#summarize---cidr聚合)
    - [expand - IP范围转CIDR](#expand---ip范围转cidr)
    - [private - 私网IP检测](#private---私网ip检测)
    - [classify - IP分类](#classify---ip分类)
    - [convert - IP格式转换](#convert---ip格式转换)
    - [lookup - 网络查询](#lookup---网络查询)
    - [conflict - 冲突检测](#conflict---冲突检测)
    - [visualize - 网络可视化](#visualize---网络可视化)
    - [plan - 子网规划](#plan---子网规划)
    - [dhcp - DHCP规划](#dhcp---dhcp规划)
    - [validate - IP验证](#validate---ip验证)
    - [range - IP范围计算](#range---ip范围计算)
  - [💡 常见使用场景](#-常见使用场景)
    - [网络管理](#网络管理)
    - [子网规划](#子网规划)
    - [IP范围转换](#ip范围转换)
    - [安全与合规](#安全与合规)
    - [DHCP规划](#dhcp规划)
  - [📊 输出格式](#-输出格式)
  - [🔧 批量处理](#-批量处理)
  - [🏗️ 项目架构](#️-项目架构)
  - [🤝 贡献代码](#-贡献代码)
  - [📄 开源许可](#-开源许可)
  - [❓ 常见问题](#-常见问题)
  - [📞 技术支持](#-技术支持)

<!-- /目录 -->

## ✨ 功能特性

- **IPv4 & IPv6 双协议支持** - 完整支持 IPv4 和 IPv6 协议
- **15+ 命令工具** - 全面的 IP 地址计算工具包
- **子网计算器** - 计算网络地址、广播地址、子网掩码、通配符掩码、可用主机数
- **VLSM计算器** - 可变长子网掩码，实现高效的 IP 地址分配
- **CIDR计算器** - CIDR 标记计算和聚合功能
- **DHCP规划** - DHCP 作用域规划，支持预留地址和排除范围
- **多种输出格式** - 支持人类可读格式、JSON、YAML、CSV 输出
- **批量处理** - 一次处理多个 IP 地址或网络
- **网络可视化** - ASCII 树形图展示网络层级结构
- **冲突检测** - 检测网络之间的 IP 地址重叠

## 🚀 快速开始

```bash
# 计算子网信息（最常用）
ipcalc subnet 192.168.1.0/24

# 检测 IP 是否为私网地址
ipcalc private 10.0.0.1

# 将网络分割成子网
ipcalc divide 192.168.1.0/24 4

# VLSM 多VLAN地址分配
ipcalc vlsm 192.168.1.0/24 100,50,25,10

# 聚合多个 CIDR 网段
ipcalc summarize 192.168.1.0/24 192.168.2.0/24

# IP范围转换为CIDR
ipcalc expand 192.168.1.1-192.168.1.254
```

## 📦 安装方式

### 从源码安装（推荐）

```bash
# 克隆仓库
git clone https://github.com/cool0looc/ipcalc.git
cd ipcalc

# 构建 Release 版本
cargo build --release

# 全局安装
cargo install --path .
```

### 预编译二进制文件

从 [Releases](https://github.com/cool0looc/ipcalc/releases) 页面下载：

| 平台 | 下载文件 |
|------|----------|
| Linux (x86_64) | ipcalc-linux-x86_64 |
| macOS (Apple Silicon) | ipcalc-macos-aarch64 |
| macOS (Intel) | ipcalc-macos-x86_64 |
| Windows | ipcalc-windows.exe |

## 📖 什么是 ipcalc？

**ipcalc** 是一款免费、开源的 IP 地址计算器和子网掩码计算器，使用 Rust 语言编写。它为网络工程师、系统管理员和 IT 专业人员提供了一套全面的工具，用于：

- **子网划分**：计算网络边界、广播地址和可用主机范围
- **VLSM（可变长子网掩码）**：根据部门/VLAN 需求高效分配 IP 地址
- **CIDR（无类别域间路由）**：处理 CIDR 标记和聚合 CIDR 网段
- **DHCP规划**：规划 DHCP 作用域，设置合适的排除范围和预留地址
- **IP验证**：验证 IP 地址和 CIDR 标记的有效性

### 为什么选择 ipcalc？

| 特性 | ipcalc | 在线计算器 | 其他命令行工具 |
|------|--------|-----------|---------------|
| 无需网络连接 | ✅ | ❌ | ✅ |
| 可脚本化/API | ✅ | ❌ | 有限支持 |
| VLSM 支持 | ✅ | 有限支持 | 有限支持 |
| IPv6 支持 | ✅ | 支持情况不一 | 有限支持 |
| 输出格式 | JSON/YAML/CSV | 仅 HTML | 仅文本 |
| 跨平台 | ✅ | 仅浏览器 | 支持情况不一 |
| 开源 | ✅ | ❌ | 支持情况不一 |

## 📚 命令参考

### subnet - 子网计算器

计算详细的子网信息，包括网络地址、广播地址、子网掩码、通配符掩码、第一个/最后一个可用 IP、总可用主机数。

```bash
# 基础子网计算
ipcalc subnet 192.168.1.0/24

# 一次计算多个子网
ipcalc subnet 192.168.1.0/24 10.0.0.0/8 172.16.0.0/16

# 显示所有子网详情
ipcalc subnet --show-all 192.168.1.0/24
```

**输出示例：**
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

**选项：**
- `--show-all, -s` - 显示所有子网详情

---

### vlsm - VLSM计算器

可变长子网掩码（VLSM）计算器。根据主机需求分配子网，实现 IP 地址的高效利用。

```bash
# 网络设计的 VLSM 计算
ipcalc vlsm 192.168.1.0/24 100,50,25,10

# 大型网络 VLSM 规划
ipcalc vlsm 10.0.0.0/8 10000,5000,2000,1000,500,100
```

**输出示例：**
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

**选项：**
- `REQUIREMENTS...` - 主机需求（逗号分隔或多个参数）

---

### divide - 网络分割

将一个网络分割成等大小的子网。

```bash
# 分割成 4 个等分子网
ipcalc divide 192.168.1.0/24 4

# 分割成 8 个子网
ipcalc divide 10.0.0.0/8 256

# 自定义分割数量
ipcalc divide 172.16.0.0/16 16
```

**输出示例：**
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

### summarize - CIDR聚合

将多个 CIDR 网段聚合为超网（CIDR 合并）。

```bash
# 聚合 CIDR 网段
ipcalc summarize 192.168.1.0/24 192.168.2.0/24

# 多个网络聚合
ipcalc summarize 10.0.0.0/24 10.0.1.0/24 10.0.2.0/24 10.0.3.0/24
```

**输出示例：**
```
Original Networks:
  192.168.1.0/24
  192.168.2.0/24

Aggregated Result:
  192.168.1.0/23
```

---

### expand - IP范围转CIDR

使用 RFC 4632 最优聚合算法将 IP 范围转换为 CIDR 标记。

```bash
# IP范围转换为CIDR块
ipcalc expand 192.168.1.1-192.168.1.254

# 完整网络范围
ipcalc expand 192.168.1.0-192.168.1.255

# 跨子网边界的范围
ipcalc expand 192.168.1.128-192.168.2.127
```

**选项：**
- `--min-prefix, -n` - 最小前缀长度（默认：8）
- `--max-prefix, -m` - 最大前缀长度（默认：32）

---

### private - 私网IP检测

根据 RFC 标准检测 IP 地址是私网还是公网。

```bash
ipcalc private 10.0.0.1      # 私网地址 (RFC1918)
ipcalc private 172.16.0.1   # 私网地址 (RFC1918)
ipcalc private 192.168.1.1  # 私网地址 (RFC1918)
ipcalc private 8.8.8.8      # 公网IP
ipcalc private 127.0.0.1    # 环回地址
```

**输出示例：**
```
Address:     10.0.0.1
Type:        Private IP address
Reference:   RFC1918
Status:      Private
```

**特殊地址类型：**
- **私网地址 (RFC1918)**：10.0.0.0/8、172.16.0.0/12、192.168.0.0/16
- **环回地址 (RFC1122)**：127.0.0.0/8
- **链路本地 (RFC3927)**：169.254.0.0/16
- **组播地址 (RFC5771)**：224.0.0.0/4
- **CGNAT (RFC6598)**：100.64.0.0/10

---

### classify - IP分类

将 IP 地址分类为传统的 A/B/C/D/E 类。

```bash
ipcalc classify 10.0.0.1
ipcalc classify 172.16.0.1
ipcalc classify 192.168.1.1
ipcalc classify 224.0.0.1  # 组播地址
```

**IP 地址分类：**
| 类别 | 范围 | 默认掩码 | CIDR |
|------|------|----------|------|
| A类 | 0.0.0.0 - 127.255.255.255 | 255.0.0.0 | /8 |
| B类 | 128.0.0.0 - 191.255.255.255 | 255.255.0.0 | /16 |
| C类 | 192.0.0.0 - 223.255.255.255 | 255.255.255.0 | /24 |
| D类 | 224.0.0.0 - 239.255.255.255 | N/A | 组播 |
| E类 | 240.0.0.0 - 255.255.255.255 | N/A | 保留 |

---

### convert - IP格式转换

在不同格式之间转换 IP 地址。

```bash
# 转换为整数
ipcalc convert 192.168.1.1

# 转换为二进制
ipcalc convert 192.168.1.1 --to-format binary

# 转换为十六进制
ipcalc convert 192.168.1.1 --to-format hex

# 整数转换为点分十进制
ipcalc convert 3232235777 --to-format dotted
```

**支持的格式：**
- `dotted` - 点分十进制 (192.168.1.1)
- `integer` - 整数 (3232235777)
- `binary` - 二进制 (11000000.10101000.00000001.00000001)
- `hex` - 十六进制 (0xC0A80101)

---

### lookup - 网络查询

查询已知网络范围和 RFC 标准定义的信息。

```bash
# 列出所有可用网络
ipcalc lookup --list

# 查询特定 RFC 网络
ipcalc lookup RFC1918   # 私网地址
ipcalc lookup LOOPBACK  # 127.0.0.0/8
ipcalc lookup MULTICAST # 224.0.0.0/4
ipcalc lookup CGNAT     # 100.64.0.0/10
```

**可用网络查询：**
- RFC1918 - 私网地址 (10.0.0.0/8、172.16.0.0/12、192.168.0.0/16)
- LOOPBACK - 环回地址 (127.0.0.0/8)
- LINKLOCAL - 链路本地 (169.254.0.0/16)
- MULTICAST - 组播地址 (224.0.0.0/4)
- TEST-NET-1/2/3 - 文档地址范围
- CGNAT - 运营商级NAT (100.64.0.0/10)
- BENCHMARK - 基准测试 (198.18.0.0/15)

---

### conflict - 冲突检测

检测多个网络之间的 IP 地址冲突（重叠）。

```bash
# 检测网络冲突
ipcalc conflict 192.168.1.0/24 192.168.1.128/25

# 验证网络设计中无重叠
ipcalc conflict 10.0.0.0/8 172.16.0.0/12 192.168.0.0/16
```

**输出（无冲突）：**
```
✓ No conflicts detected!
```

**输出（发现冲突）：**
```
✗ Found 1 conflict!

Conflict #1: A contains B
Networks: 192.168.1.0/24, 192.168.1.128/25
Overlap: 192.168.1.128 - 192.168.1.255
```

---

### visualize - 网络可视化

以 ASCII 树形图展示网络层级结构。

```bash
# 基础网络可视化
ipcalc visualize 192.168.1.0/24

# 限制深度的可视化
ipcalc visualize 10.0.0.0/8 --depth 2

# 大型网络树状图
ipcalc visualize 172.16.0.0/12 --depth 3
```

**输出示例：**
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

### plan - 子网规划

智能子网规划，提供建议和可行性评级。

```bash
# 基础网络规划
ipcalc plan 192.168.1.0/24

# 带具体需求的规划
ipcalc plan 192.168.1.0/24 100 50 25

# 大型网络子网设计
ipcalc plan 10.0.0.0/8 10000 5000 2000
```

**输出示例：**
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

### dhcp - DHCP规划

DHCP 作用域规划，支持静态预留地址和排除范围。

```bash
# 基础 DHCP 作用域规划
ipcalc dhcp 192.168.1.0/24

# 带预留地址的规划
ipcalc dhcp 192.168.1.0/24 --reservations 10

# 带排除范围的规划
ipcalc dhcp 192.168.1.0/24 --exclusions 5

# 完整的 DHCP 规划
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5
```

**输出示例：**
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

### validate - IP验证

验证 IP 地址或 CIDR 标记的有效性。

```bash
# 验证单个 IP
ipcalc validate 192.168.1.1

# 验证 CIDR 标记
ipcalc validate 192.168.1.0/24

# 验证多个地址
ipcalc validate 10.0.0.0/8 172.16.0.1 192.168.1.0/16

# 严格模式 - 任何无效输入都会失败
ipcalc validate --strict 192.168.1.0/24 invalid_ip
```

---

### range - IP范围计算

显示 CIDR 标记的 IP 范围（起始和结束地址）。

```bash
ipcalc range 192.168.1.0/24
ipcalc range 10.0.0.0/8
```

**输出示例：**
```
CIDR:         192.168.1.0/24
Start:        192.168.1.0 (Network)
First Host:   192.168.1.1
Last Host:    192.168.1.254
End:          192.168.1.255 (Broadcast)
```

---

## 💡 常见使用场景

### 网络管理

```bash
# 验证 IP 配置
ipcalc validate 192.168.1.100/26

# 计算子网详情
ipcalc subnet 192.168.1.0/26

# 检查可用主机数
ipcalc subnet 10.0.0.0/8 --show-all
```

### 子网规划

```bash
# 新部门规划（需要100台主机）
ipcalc plan 192.168.0.0/16 100

# 将网络分割成子网
ipcalc divide 192.168.1.0/24 8

# 企业网络 VLSM 分配
ipcalc vlsm 10.0.0.0/8 10000,5000,2000,1000,500
```

### IP范围转换

```bash
# IP范围转换为CIDR块
ipcalc expand 192.168.1.1-192.168.1.254

# 跨子网边界的范围转换
ipcalc expand 192.168.0.0-192.168.3.255

# 聚合CIDR实现路由汇总
ipcalc summarize 192.168.0.0/24 192.168.1.0/24 192.168.2.0/24 192.168.3.0/24
```

### 安全与合规

```bash
# 检测私网与公网IP
ipcalc private 10.0.0.1 172.16.0.1 192.168.1.1 8.8.8.8

# 查询特殊网络范围
ipcalc lookup RFC1918

# 检测 IP 冲突
ipcalc conflict 192.168.1.0/24 192.168.1.128/25
```

### DHCP规划

```bash
# 办公室网络 DHCP 作用域规划
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5

# 大型企业网络规划
ipcalc dhcp 10.0.0.0/8 --reservations 50 --exclusions 20
```

---

## 📊 输出格式

所有命令都支持多种输出格式：人类可读格式（默认）、JSON、YAML 和 CSV。

```bash
# 人类可读输出（默认）
ipcalc subnet 192.168.1.0/24

# JSON 输出用于脚本处理
ipcalc --format json subnet 192.168.1.0/24

# YAML 输出
ipcalc --format yaml subnet 192.168.1.0/24

# CSV 输出用于数据处理
ipcalc --format csv subnet 192.168.1.0/24
```

**注意：** 格式选项必须放在子命令之前。

---

## 🔧 批量处理

从文件处理多个 IP 地址或网络。

```bash
# 创建包含 IP 地址/CIDR 的输入文件
cat > ips.txt << EOF
192.168.1.0/24
10.0.0.0/8
172.16.0.0/12
EOF

# 从文件处理
ipcalc subnet --file ips.txt

# 批量处理并输出 JSON
ipcalc --format json subnet --file ips.txt
```

---

## 🏗️ 项目架构

```
ipcalc/
├── src/
│   ├── main.rs           # CLI 入口点
│   ├── lib.rs            # 库导出
│   ├── cli/              # CLI 命令（15+个子命令）
│   │   ├── validate.rs   # IP/CIDR 验证
│   │   ├── subnet.rs     # 子网计算
│   │   ├── range.rs      # IP 范围计算
│   │   ├── expand.rs     # IP范围转CIDR
│   │   ├── summarize.rs  # CIDR 聚合
│   │   ├── divide.rs     # 网络分割
│   │   ├── vlsm.rs       # VLSM 计算器
│   │   ├── classify.rs   # IP 分类
│   │   ├── private.rs    # 私网IP检测
│   │   ├── convert.rs    # 格式转换
│   │   ├── lookup.rs     # 网络查询
│   │   ├── visualize.rs  # 网络可视化
│   │   ├── conflict.rs   # 冲突检测
│   │   ├── plan.rs       # 子网规划
│   │   └── dhcp.rs       # DHCP 规划
│   ├── core/             # 核心 IP 计算逻辑
│   │   ├── ipv4/         # IPv4 实现
│   │   ├── ipv6/         # IPv6 实现
│   │   └── cidr/         # CIDR 操作
│   ├── formats/          # 输出格式化（JSON/YAML/CSV）
│   └── utils/            # 工具函数
├── tests/                # 集成测试
├── Cargo.toml
└── README.md
```

---

## 🤝 贡献代码

欢迎贡献代码！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

---

## 📄 开源许可

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

## ❓ 常见问题

**问：ipcalc 有什么用？**

答：ipcalc 是一款 IP 地址计算器和子网掩码计算器，用于网络规划、子网划分和 IP 地址管理。它帮助网络管理员计算子网、规划 VLSM 分配和管理 IP 地址空间。

**问：如何计算子网？**

答：使用 `subnet` 命令：
```bash
ipcalc subnet 192.168.1.0/24
```

**问：ipcalc 和在线 IP 计算器有什么区别？**

答：ipcalc 是命令行工具，可以离线工作，支持脚本化，提供多种输出格式（JSON/YAML/CSV），专为自动化和集成到网络管理工作流而设计。

**问：ipcalc 支持 IPv6 吗？**

答：是的，ipcalc 支持 IPv4 和 IPv6 地址计算。

**问：什么是 VLSM？**

答：VLSM（可变长子网掩码）允许同一网络内的不同子网使用不同的子网掩码，从而实现更高效的 IP 地址分配。使用 `ipcalc vlsm` 计算 VLSM 子网。

**问：如何将 IP 范围转换为 CIDR？**

答：使用 `expand` 命令：
```bash
ipcalc expand 192.168.1.1-192.168.1.254
```

**问：ipcalc 可以用于 DHCP 规划吗？**

答：可以，`dhcp` 命令帮助规划 DHCP 作用域，设置适当的排除范围和静态预留地址：
```bash
ipcalc dhcp 192.168.1.0/24 --reservations 10 --exclusions 5
```

**问：ipcalc 是免费的吗？**

答：是的，ipcalc 是采用 MIT 许可证的开源软件。

---

## 📞 技术支持

如遇到问题或有疑问：

- 在 [GitHub](https://github.com/cool0looc/ipcalc/issues) 上提交 issue
- 查看 [Wiki](https://github.com/cool0looc/ipcalc/wiki) 获取更多文档

---

**用 ❤️ 和 [Rust](https://www.rust-lang.org) 打造** | **觉得有用就 Star ⭐一下吧！**
