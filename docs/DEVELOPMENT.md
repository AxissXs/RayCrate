# RayCrate Development Guide

This guide explains how to set up your local development environment to build, test, and contribute to RayCrate.

---

## Prerequisites

1. **Rust Toolchain**: Install stable Rust (edition 2021+) via [rustup](https://rustup.rs/).
2. **Node.js & pnpm**: Required for the Tauri v2 frontend development (`node >= 18`, `pnpm`).
3. **Core Binaries**: Download or place `xray`, `openvpn`, and `openconnect` binaries in the `bin/` directory or ensure they are available in system PATH.

---

## Repository Setup

```bash
# Clone the repository
git clone https://github.com/AxissXs/RayCrate.git
cd RayCrate

# Build the Rust core
cargo build --workspace

# Run tests
cargo test --workspace
```

---

## Running in Development Mode

```bash
# Install frontend dependencies
cd gui
pnpm install

# Run Tauri development server (starts Rust backend + Vite frontend)
pnpm tauri dev
```

---

## Contributing Workflow

1. Create a feature branch from `arena/019f757f-raycrate` (or `main`).
2. Follow coding guidelines in `AGENTS.md` (no `.unwrap()` in library code, comprehensive error handling with `thiserror`).
3. Add unit tests for new features.
4. Open a Pull Request with a clear description of changes.
