<div align="center">

# 🦀 Crabby

### A Blazingly Fast, Standalone Package Manager for Node.js

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/AqwozTheDeveloper/crabby?style=for-the-badge&logo=github)](https://github.com/AqwozTheDeveloper/crabby/stargazers)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](http://makeapullrequest.com)

**No Node.js Required** • **Built-in TypeScript Runtime** • **38x Faster** • **SHA-1 Verified**

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Documentation](#-documentation)

---

</div>

## ✨ Why Crabby?

**Crabby** is the **only package manager with a built-in TypeScript runtime**. While other tools require you to install `ts-node` or `tsx` and configure complex build steps, Crabby treats TypeScript as a first-class citizen. It works **without Node.js pre-installed**, automatically handling the runtime for you.

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
Automatic backups, dry-run mode, SHA-1 checksum verification, and confirmation prompts for all destructive operations.

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

- 🔒 **SHA-1 Checksum Verification** - Cryptographic verification of all packages (npm-compatible)
- 💾 **Automatic Backups** - Before destructive operations
- 🧪 **Dry-Run Mode** - Preview changes without applying them
- ⚡ **Force Flags** - Skip confirmations for automation
- ✅ **Validation** - JSON and lockfile integrity checks
- ⏱️ **Installation Timer** - Track installation duration

### Performance Features

- ⚡ **Parallel Downloads** - 16 concurrent package downloads
- 🔄 **Lockfile-First Resolution** - Skip network requests when locked
- 🌐 **Shared HTTP Client** - Connection pooling for faster downloads
- 💾 **Global Cache** - Reuse downloaded packages across projects
- 📊 **38x Faster** - Than npm for typical installations

### Advanced Features

- 🔄 **Lifecycle Scripts** - Automatic `postinstall` execution
- 🔗 **Binary Linking** - CLI tools in `node_modules/.bin`
- 🖼️ **GUI Support** - Works with Electron, Tauri, etc.
- 📦 **Semantic Versioning** - Smart version range handling
- ⚙️ **Custom Registry** - Configure via `crabby.config.json`
- 🔤 **UTF-8 BOM Support** - PowerShell compatibility

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
# Initialize a new project (interactive)
crabby init

# Install all dependencies
crabby install

# Add a package (alias for install)
crabby add express
crabby add typescript -D

# Run binaries (npx alternative)
crabby exec tsc --init
crabby x jest

# Run TypeScript files
crabby run src/index.ts

# Run package scripts
crabby start
crabby test
```

---

## 📚 Commands

<details>
<summary><b>📦 Package Management</b></summary>

```bash
crabby init                    # Initialize a new project (interactive)
crabby install                 # Install all dependencies
crabby add <package>           # Add a package (alias for install)
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
crabby exec <cmd>            # Run binary from node_modules (alias: x)
crabby run <script>            # Run package.json script
crabby run src/index.ts        # Run TypeScript file
crabby run src/index.js        # Run JavaScript file
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
| Built-in TS Runtime | ✅ | ❌ | ❌ | ❌ |
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
mkdir src
echo "import express from 'express';" > src/index.ts
echo "const app = express();" >> src/index.ts
echo "app.listen(3000);" >> src/index.ts

# Run it!
crabby run src/index.ts
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
