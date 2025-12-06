# 🦀 Crabby BETA-1.0.0 Release Notes

**Release Date:** December 6, 2025  
**Type:** Beta Release

---

## 🎉 Overview

We're excited to announce the first beta release of **Crabby** - a blazingly fast Node.js package manager written in Rust! This release brings a feature-complete package manager that rivals npm and yarn in functionality while offering superior performance.

## ✨ Key Features

### 📦 Package Management
- **Install packages** from the npm registry with full dependency resolution
- **Remove packages** cleanly with `crabby remove` or `crabby rm`
- **Recursive dependency installation** with intelligent conflict resolution
- **Semantic versioning** support for version constraints
- **Reproducible builds** using `crabby.lock` lockfile

### 🚀 Performance & Caching
- **Global cache system** - Downloaded packages are cached locally for instant reinstalls
  - Windows: `%LOCALAPPDATA%\crabby\cache`
  - Unix/Linux/Mac: `~/.cache/crabby`
- **Blazingly fast** installation powered by Rust's performance
- **Offline capability** for previously cached packages

### 🏃 Universal Script Runner
- **Auto-detection** - Run `.js` and `.ts` files without flags
  ```bash
  crabby run app.ts    # Automatically uses ts-node
  crabby run server.js # Automatically uses node
  ```
- **Package scripts** - Compatible with npm scripts in `package.json`
- **GUI framework support** - Handles Electron, Tauri, and other GUI applications
- **Real-time output streaming** - See your script output as it happens

### ⚙️ Advanced Features
- **Lifecycle scripts** - Automatic execution of `postinstall` scripts
- **Binary linking** - CLI tools automatically available in `node_modules/.bin`
- **Scoped packages** - Full support for `@scope/package` notation
- **Custom registries** - Configure alternative npm registries via `crabby.config.json`

## 📋 Available Commands

```bash
crabby init              # Initialize a new project
crabby install <pkg>     # Install a package (alias: i)
crabby remove <pkg>      # Remove a package (alias: rm)
crabby run <script|file> # Run a script or file (alias: cook)
crabby start             # Run the start script
crabby test              # Run the test script
```

## 🎯 What's Included

### Core Functionality
- ✅ Project initialization
- ✅ Package installation with dependency resolution
- ✅ Package removal
- ✅ Script execution
- ✅ TypeScript and JavaScript file execution
- ✅ Global package cache
- ✅ Lockfile management

### Developer Experience
- ✅ Beautiful CLI output with emojis and colors
- ✅ Progress indicators for long operations
- ✅ Helpful error messages
- ✅ Performance metrics (install duration)

## 🔧 Installation

### From Source
```bash
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
cargo build --release
```

The binary will be available at `./target/release/crabby`

## 📝 Example Usage

```bash
# Initialize a new project
crabby init

# Install packages
crabby install lodash
crabby install electron

# Run TypeScript files directly
crabby run src/app.ts

# Run package scripts
crabby run build
crabby start

# Remove packages
crabby remove lodash
```

## 🐛 Known Limitations

- **Workspaces** - Monorepo/workspace support is not yet implemented
- **Dev Dependencies** - All dependencies are treated as production dependencies
- **Peer Dependencies** - Not yet handled automatically

## 🔮 Coming Soon

- [ ] Workspace/monorepo support
- [ ] Dev dependencies distinction
- [ ] Peer dependency resolution
- [ ] Package publishing
- [ ] Update command

## 🙏 Acknowledgments

Built with 🦀 and ❤️ by **aqwozthedeveloper**

## 📄 License

MIT License

---

**Try it out and let us know what you think!** 🚀

Report issues at: https://github.com/AqwozTheDeveloper/crabby/issues
