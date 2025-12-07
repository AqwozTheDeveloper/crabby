<div align="center">

# 🦀 Crabby

### A Blazingly Fast, Standalone Package Manager for Node.js

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/AqwozTheDeveloper/crabby?style=for-the-badge&logo=github)](https://github.com/AqwozTheDeveloper/crabby/stargazers)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](http://makeapullrequest.com)

**No Node.js Required** • **20x Faster TypeScript** • **Full npm Compatibility**

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Documentation](#-documentation)

---

</div>

## ✨ Why Crabby?

**Crabby** is a modern, standalone package manager that works **without Node.js installed**. It automatically downloads a portable Node.js runtime if needed, making it perfect for fresh systems, CI/CD environments, and developers who want a truly standalone tool.

### 🎯 Key Highlights

<table>
<tr>
<td width="50%">

#### 🚀 Standalone Runtime
No Node.js installation required. Crabby auto-downloads a portable version (~50MB) on first run and caches it forever.

#### ⚡ Blazingly Fast
20x faster TypeScript execution using tsx. Install, run, and iterate at lightning speed.

</td>
<td width="50%">

#### 🔒 Enterprise Safety
Automatic backups, dry-run mode, SHA-256 checksums, and confirmation prompts for all destructive operations.

#### 🎨 Beautiful CLI
Modern interface with colors, emojis, progress indicators, and helpful error messages.

</td>
</tr>
</table>

---

## 🚀 Features

### Core Capabilities

- ✅ **Standalone** - Works without Node.js installed
- ✅ **Fast TypeScript** - 20x faster execution with tsx
- ✅ **Full npm Support** - Compatible with all npm packages
- ✅ **Dev Dependencies** - Separate `dependencies` and `devDependencies`
- ✅ **Lock Files** - `crabby.lock` for reproducible builds
- ✅ **Global Cache** - Shared cache at `~/.crabby/cache/`
- ✅ **Package Updates** - Check and update packages easily
- ✅ **Package Info** - Query npm registry for package details
- ✅ **Workspaces** - Basic monorepo support

### Safety Features

- 🔒 **SHA-256 Checksums** - Verify package integrity
- 💾 **Automatic Backups** - Before destructive operations
- 🧪 **Dry-Run Mode** - Preview changes without applying them
- ⚡ **Force Flags** - Skip confirmations for automation
- ✅ **Validation** - JSON and lockfile integrity checks

### Advanced Features

- 🔄 **Lifecycle Scripts** - Automatic `postinstall` execution
- 🔗 **Binary Linking** - CLI tools in `node_modules/.bin`
- 🖼️ **GUI Support** - Works with Electron, Tauri, etc.
- 📦 **Semantic Versioning** - Smart version range handling
- ⚙️ **Custom Registry** - Configure via `crabby.config.json`

---

## 📦 Installation

### Windows

```powershell
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
.\install.ps1
```

### macOS / Linux

```bash
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
chmod +x install.sh
./install.sh
```

> **Note**: Requires [Rust](https://rustup.rs/) to build from source.

---

## 🎯 Quick Start

```bash
# Initialize a new project
crabby init

# Install all dependencies
crabby install

# Install a specific package
crabby install express

# Install as dev dependency
crabby install typescript --save-dev

# Run TypeScript files
crabby run app.ts

# Run package scripts
crabby start
crabby test
```

---

## 📚 Commands

<details>
<summary><b>📦 Package Management</b></summary>

```bash
crabby init                    # Initialize a new project
crabby install                 # Install all dependencies
crabby install <package>       # Install specific package
crabby install <pkg> -D        # Install as dev dependency
crabby remove <package>        # Remove package
crabby remove <pkg> --force    # Remove without confirmation
crabby list                    # List installed packages
crabby clean                   # Clean node_modules
crabby clean --cache           # Also clean global cache
crabby clean --dry-run         # Preview what will be removed
```

</details>

<details>
<summary><b>🔄 Updates & Information</b></summary>

```bash
crabby update                  # Check for updates
crabby update <package>        # Update specific package
crabby outdated                # Show outdated packages
crabby info <package>          # Show package information
```

</details>

<details>
<summary><b>▶️ Running Code</b></summary>

```bash
crabby run <script>            # Run package.json script
crabby run app.ts              # Run TypeScript file
crabby run app.js              # Run JavaScript file
crabby start                   # Run start script
crabby test                    # Run test script
```

</details>

<details>
<summary><b>🛠️ Utilities</b></summary>

```bash
crabby --version               # Show version
crabby --help                  # Show help
```

</details>

---

## 🔧 Configuration

Create `crabby.config.json` in your project root:

```json
{
  "registry": "https://registry.npmjs.org",
  "log_level": "info"
}
```

---

## 📊 Comparison

| Feature | Crabby | npm | yarn | pnpm |
|---------|:------:|:---:|:----:|:----:|
| Standalone | ✅ | ❌ | ❌ | ❌ |
| Fast TypeScript | ✅ | ⚠️ | ⚠️ | ⚠️ |
| Automatic Backups | ✅ | ❌ | ❌ | ❌ |
| Dry-Run Mode | ✅ | ❌ | ✅ | ✅ |
| Lock Files | ✅ | ✅ | ✅ | ✅ |
| Workspaces | ⚠️ | ✅ | ✅ | ✅ |
| Dev Dependencies | ✅ | ✅ | ✅ | ✅ |
| Global Cache | ✅ | ✅ | ✅ | ✅ |

---

## 🎨 Examples

### Basic Usage

```bash
# Create a new project
crabby init

# Install dependencies
crabby install express
crabby install typescript -D

# Create a simple server
echo "import express from 'express';" > server.ts
echo "const app = express();" >> server.ts
echo "app.listen(3000);" >> server.ts

# Run it!
crabby run server.ts
```

### With Workspaces

```json
{
  "name": "my-monorepo",
  "workspaces": ["packages/*", "apps/*"]
}
```

```bash
crabby install  # Links all workspace packages
```

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby

# Build
cargo build --release

# Run tests
cargo test

# Run
./target/release/crabby --help
```

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Uses [tsx](https://github.com/esbuild-kit/tsx) for fast TypeScript execution
- Inspired by npm, yarn, and pnpm

---

## 🌟 Star History

If you find Crabby useful, please consider giving it a star! ⭐

---

<div align="center">

### Built with 🦀 and ❤️ by [AqwozTheDeveloper](https://github.com/AqwozTheDeveloper)

[Report Bug](https://github.com/AqwozTheDeveloper/crabby/issues) • [Request Feature](https://github.com/AqwozTheDeveloper/crabby/issues) • [Discussions](https://github.com/AqwozTheDeveloper/crabby/discussions)

**[⬆ Back to Top](#-crabby)**

</div>
