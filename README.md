<img width="1000" height="500" alt="Untitled design (2)(2)" src="https://github.com/user-attachments/assets/f911c7f7-e2d1-474c-9b0b-f2dfb5d91261" />

<div align="center">
# 🦀 Crabby
**A Blazingly Fast Node.js Package Manager written in Rust.**

[![Rust](https://img.shields.io/badge/built_with-Rust-d35400.svg?style=flat-square)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

<img width="1139" height="1006" alt="image" src="https://github.com/user-attachments/assets/0e18800c-22b5-4721-a6de-390f46f1b453" />

</div>

---

**Crabby** is a modern, lightweight, and fast package manager for Node.js, built to demonstrate the power of Rust for developer tooling. It supports standard `package.json` workflows, recursive dependency resolution, and sophisticated lifecycle management.

## 🚀 Features

- **⚡ Blazingly Fast**: Powered by Rust's multi-threaded architecture.
- **📦 Recursive Installation**: Robustly resolves and downloads deep dependency trees.
- **🧠 Semantic Versioning**: Intelligently selects compatible versions (`^1.2.0`) to prevent conflicts.
- **⚙️ Lifecycle Scripts**: Automatically executes `postinstall` scripts (essential for packages like `electron`).
- **🔗 Binary Linking**: Creates executables in `node_modules/.bin` for CLI tools.
- **🏃 Universal Runner**: `crabby run <file>` supports:
    - **Auto-detection**: Smartly runs `.ts` (via `ts-node`) or `.js` (via `node`) files.
    - **GUI Support**: Handles process spawning for GUI frameworks like Electron and Tauri.
    - **Output Streaming**: Real-time output visibility for long-running processes.
- **🔒 Reproducible Builds**: Uses `crabby.lock` to lock versions and tarballs.
- **🎨 Configurable**: Custom registries via `crabby.config.json`.

## 📦 Installation

To build Crabby from source, ensure you have [Rust installed](https://rustup.rs/).

```bash
git clone https://github.com/yourusername/crabby.git
cd crabby
cargo build --release
```

The binary will be located at `./target/release/crabby`.

## 🛠️ Usage

### Initialize
Create a new `package.json` in the current directory:
```bash
crabby init
```

### Install
Install dependencies from the registry:
```bash
# Install a package (defaults to saving as dependency)
crabby install lodash

# Install a complex package with binaries and scripts
crabby install electron
```

### Run Scripts & Files
Execute scripts defined in your `package.json` OR run files directly:

```bash
# Run a specific script from package.json
crabby run test

# Run a TypeScript file (auto-uses ts-node)
crabby run src/app.ts

# Run a JavaScript file
crabby run server.js

# Shortcuts
crabby start  # alias for `run start`
crabby test   # alias for `run test`
```

### Configuration
Configure Crabby via `crabby.config.json` in your project root:
```json
{
  "registry": "https://registry.npmjs.org",
  "log_level": "info"
}
```

## 🗺️ Roadmap

- [x] MVP (Init, Install, Run)
- [x] Recursive Dependencies
- [x] Lifecycle Scripts
- [x] Semver Resolution
- [x] Binary Linking
- [x] TS/JS File Runner
- [x] Global Cache
- [x] `crabby remove`
- [ ] Workspaces support

---

<div align="center">
Built with 🦀 and ❤️ by aqwozthedeveloper.
</div>
