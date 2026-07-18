# RayCrate Architecture

RayCrate is engineered with a modular, decoupled architecture designed for high performance, reliability, security, and extensibility across Windows, macOS, and Linux.

---

## 1. High-Level Architecture

```
+-------------------------------------------------------------------+
|                         RayCrate GUI (Tauri v2)                   |
|           (HTML/TS + Tailwind CSS + Lucide Icons + State)         |
+-------------------------------------------------------------------+
                                  │ IPC / Tauri Commands
                                  ▼
+-------------------------------------------------------------------+
|                       RayCrate Core Engine                        |
|  +-------------------+  +------------------+  +----------------+  |
|  | Profile & Sub     |  | Xray Process     |  | Routing Engine |  |
|  | Manager           |  | & Config Gen     |  | & GeoIP/Geosite|  |
|  +-------------------+  +------------------+  +----------------+  |
|  +-------------------+  +------------------+                    |  |
|  | TUN Mode Manager  |  | Protocol Plugins |                    |  |
|  | (tun2 / Wintun)   |  | (OpenVPN/Connect)|                    |  |
|  +-------------------+  +------------------+                    |  |
+-------------------------------------------------------------------+
                                  │ Manages Sub-processes / Drivers
                                  ▼
+-------------------------------------------------------------------+
|                    System Binaries & OS Drivers                   |
|  [xray-core binary]   [openvpn binary]   [openconnect binary]     |
+-------------------------------------------------------------------+
```

---

## 2. Core Modules (`raycrate-core`)

### 2.1 Profile & Subscription Manager (`config.rs`)
- Parses standard proxy links (`vless://`, `vmess://`, `trojan://`, `ss://`, `hysteria2://`, `tuic://`).
- Fetches and parses Base64-encoded subscription lists and Clash YAML configurations.
- Normalizes configurations into a unified internal Rust `ProxyProfile` struct.

### 2.2 Xray-Core Manager (`xray.rs`)
- Manages the lifecycle of the bundled `xray` binary (spawn, monitor, health-check, terminate).
- Dynamically generates JSON configuration files based on the active profile, local SOCKS5/HTTP inbound ports, and routing rules.
- Captures standard output/error for real-time connection logging and speed/latency metrics.

### 2.3 Routing Engine (`router.rs`)
- Handles traffic routing policies:
  - **Bypass LAN & China/Local**: Direct connection for local subnets.
  - **Global Proxy**: Route all traffic through the proxy.
  - **Direct/Block**: Custom rules using GeoIP and Geosite databases.

### 2.4 TUN Mode & Packet Routing (`tun.rs`)
- Creates a virtual network adapter (`tun0` on Linux/macOS, `Wintun` adapter on Windows) using the `tun2` or `tun` Rust crate.
- Intercepts layer 3 IP packets and routes them through the local SOCKS5/HTTP proxy inbound exposed by Xray-core.
- Configures OS routing tables and DNS servers automatically (using `iptables`/`nftables` on Linux, `pf` on macOS, and Windows routing APIs).

### 2.5 Protocol Plugins (`protocols/`)
- Extensible plugin architecture supporting non-Xray protocols:
  - **OpenVPN**: Manages `openvpn` CLI wrapper with `.ovpn` configuration parsing.
  - **OpenConnect / AnyConnect**: Manages `openconnect` for corporate VPN endpoints.

---

## 3. GUI Layer (`raycrate-gui`)
- Built with **Tauri v2**, combining a Rust backend with a modern web frontend (TypeScript, Tailwind CSS, Lucide icons).
- Provides a clean, responsive, dark-mode-first interface inspired by Raycast and modern developer tools.
- Real-time statistics dashboard (upload/download speed, active connections, latency graphs).
