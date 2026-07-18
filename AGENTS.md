# Instructions for AI Coding Agents (`AGENTS.md`)

Welcome, AI agent! You are assisting in the development of **RayCrate**, a modern, robust, high-performance cross-platform proxy client written in Rust, featuring Xray-core integration, advanced TUN mode, system proxy management, subscription handling, and modular protocol support (including OpenVPN and OpenConnect).

---

## 1. Project Overview & Tech Stack
- **Language**: Rust (edition 2021+) for core engine, backend, routing, and TUN management.
- **GUI Framework**: Tauri v2 (HTML/TS/Tailwind/Lucide frontend + Rust backend) or native Rust (e.g., Iced/Egui) for ultra-fast, modern, beautiful user interfaces.
- **Proxy Core**: Xray-core (managed as a sidecar binary with dynamic JSON configuration generation), with modular wrappers for OpenVPN, OpenConnect (`ocserv`), and Shadowsocks.
- **TUN Mode**: `tun` / `tun2` crates for cross-platform network interface creation (`utun` on macOS/iOS, `tunX` on Linux, `Wintun` on Windows) combined with IP packet routing and smoltcp/direct packet forwarding.

---

## 2. Core Architecture Rules for AI Agents
1. **Modularity & Separation of Concerns**:
   - `raycrate-core`: Rust library containing profile parsing, subscription management, Xray process manager, TUN mode controller, routing rules, and speed/latency testers.
   - `raycrate-gui` (or Tauri backend): Exposes safe async commands to the frontend, manages application state via `tokio::sync::Mutex` or `RwLock`.
2. **Error Handling**:
   - Never use `.unwrap()` or `.expect()` in production library code (`raycrate-core`). Use custom `thiserror` enums and return `Result<T, RayCrateError>`.
   - Log errors gracefully using `tracing` crate (`tracing::info!`, `tracing::error!`).
3. **Safety & Cross-Platform Compatibility**:
   - TUN mode requires administrator/root privileges. Always check permissions gracefully and prompt user or handle elevation cleanly.
   - Ensure path handling uses `std::path::PathBuf` across Windows, macOS, and Linux.
4. **Testing**:
   - Write unit tests for profile parsers (v2ray links, base64 subscriptions, Clash YAML config parsing).
   - Mock core processes when testing state management.

---

## 3. Directory Structure
```
RayCrate/
├── Cargo.toml                  # Workspace manifest
├── core/                       # RayCrate Core Rust Library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Library entry point
│       ├── config.rs           # Profile & subscription data models
│       ├── xray.rs             # Xray-core sidecar manager & config generator
│       ├── tun.rs              # Cross-platform TUN device & packet router
│       ├── router.rs           # Routing rules engine (direct, proxy, block, geosite)
│       └── protocols/          # OpenVPN, OpenConnect, Shadowsocks integration wrappers
├── gui/                        # Tauri v2 Frontend / App
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/                    # Frontend (TS/Vue or React/Tailwind) & Rust backend
├── docs/                       # Professional Documentation
│   ├── ARCHITECTURE.md         # Detailed system design
│   ├── TUN_MODE.md             # TUN mode implementation details
│   └── DEVELOPMENT.md          # Setup and contribution guide
├── AGENTS.md                   # This file
├── LICENSE                     # MIT License
└── CONTRIBUTORS.md             # Contributors list
```

---

## 4. Coding Standards & Guidelines
- **Idiomatic Rust**: Use async/await (`tokio`), clean trait definitions, pattern matching, and zero-copy parsing where applicable.
- **Comments & Docstrings**: Public items must have `///` doc comments explaining parameters and errors.
- **Commit Messages**: Use Conventional Commits (`feat(core): add VLESS subscription parser`, `fix(tun): handle windows wintun driver loading`).

---

## 5. Typical Agent Tasks
When asked to implement a feature or fix a bug:
1. Check existing modules in `core/src/`.
2. Implement robust logic with tests in `core/src/tests.rs` or module test blocks.
3. Update relevant documentation in `docs/` if architecture or APIs change.
4. Verify cross-platform compilation validity.
