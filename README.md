# RayCrate 🚀

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021%252B-orange.svg)](https://www.rust-lang.org/)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app/)
[![Cross-Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-green.svg)]()

**RayCrate** is a modern, high-performance, cross-platform proxy client built in Rust. Inspired by tools like Nekoray, RayCrate offers a cleaner, faster, more robust, and exceptionally user-friendly experience with advanced capabilities including native TUN mode, Xray-core integration, subscription management, and multi-protocol support (including OpenVPN and OpenConnect).

---

## ✨ Key Features

- **⚡ Blazing Fast & Lightweight**: Built entirely in Rust and Tauri v2 for minimal memory footprint and maximum performance.
- **🛡️ Working TUN Mode**: Seamlessly proxy 100% of system traffic (TCP/UDP) across Linux, macOS (utun), and Windows (Wintun).
- **🌐 Advanced Core Integration**: Native management of **Xray-core** (VLESS, VMess, Trojan, Shadowsocks, Hysteria, TUIC) with dynamic JSON configuration generation.
- **🔌 Multi-Protocol Support**: Extensible architecture supporting OpenVPN and OpenConnect (`ocserv`) alongside modern proxy protocols.
- **📊 Real-time Analytics**: Live upload/download speeds, connection latency tests, and traffic usage graphs.
- **🤖 AI-Ready & Open Contributor Ecosystem**: Includes built-in `AGENTS.md` and structured guides for seamless collaboration with AI coding agents.

---

## 📂 Documentation

- [Architecture Overview](docs/ARCHITECTURE.md)
- [TUN Mode Deep Dive](docs/TUN_MODE.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [AI Agent Instructions](AGENTS.md)

---

## 🛠️ Tech Stack

- **Core Engine (`raycrate-core`)**: Rust, Tokio async runtime, `tun2` / `smoltcp` for packet routing, `reqwest` for subscriptions.
- **GUI Frontend (`raycrate-gui`)**: Tauri v2, TypeScript, Tailwind CSS, Lucide Icons.

---

## 📜 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for more information.
