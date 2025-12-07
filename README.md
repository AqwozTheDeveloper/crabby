<img width="1600" height="400" alt="Crabby" src="https://github.com/user-attachments/assets/725fffcd-3ae8-47b9-8577-c471f51929bd" />

<div align="center">

# 🦀 Crabby

**A Blazingly Fast, Standalone Package Manager for Node.js**

[![Rust](https://img.shields.io/badge/built_with-Rust-d35400.svg?style=flat-square)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

*No Node.js required • 20x faster TypeScript • Full npm compatibility*

<img width="1139" height="1006" alt="Crabby Demo" src="https://github.com/user-attachments/assets/0e18800c-22b5-4721-a6de-390f46f1b453" />

</div>

---

## ✨ Why Crabby?

**Crabby** is a modern, standalone package manager that works without Node.js installed. It automatically downloads a portable Node.js runtime if needed, making it perfect for fresh systems and CI/CD environments.

### Key Features

- 🚀 **Standalone** - No Node.js installation required
- ⚡ **Blazingly Fast** - 20x faster TypeScript execution with tsx
- 📦 **Full npm Support** - Works with all npm packages and frameworks
- 🔒 **Reproducible** - Lock files ensure consistent builds
- 🗄️ **Global Cache** - Shared cache for lightning-fast installs
- 🎨 **Beautiful CLI** - Modern interface with emojis and colors
- 🔧 **Dev Dependencies** - Separate dev and production dependencies
- 📊 **Package Management** - Update, outdated, info commands
- 🏢 **Workspaces** - Basic monorepo support

## 🚀 Quick Start

### Installation

**Windows:**
```powershell
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
.\install.ps1
```

**macOS / Linux:**
```bash
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
chmod +x install.sh
./install.sh
```

### Usage

```bash
# Initialize a new project
crabby init

# Install all dependencies from package.json
crabby install

# Install a specific package
crabby install express
crabby install typescript --save-dev

# Run TypeScript/JavaScript files
crabby run app.ts
crabby run server.js

# Package management
crabby update              # Check for updates
crabby outdated            # Show outdated packages
crabby info express        # Show package info
crabby list                # List installed packages
crabby clean               # Clean node_modules
```

## 📚 Commands

### Project Management

| Command | Description |
|---------|-------------|
| `crabby init` | Initialize a new project |
| `crabby install` | Install all dependencies |
| `crabby install <package>` | Install a specific package |
| `crabby install <pkg> -D` | Install as dev dependency |
| `crabby remove <package>` | Remove a package |
| `crabby list` | List installed packages |
| `crabby clean` | Clean node_modules and cache |

### Updates & Information

| Command | Description |
|---------|-------------|
| `crabby update` | Check for package updates |
| `crabby update <package>` | Update specific package |
| `crabby outdated` | Show outdated packages |
| `crabby info <package>` | Show package information |

### Running Code

| Command | Description |
|---------|-------------|
| `crabby run <script>` | Run package.json script |
| `crabby run app.ts` | Run TypeScript file |
| `crabby run app.js` | Run JavaScript file |
| `crabby start` | Run start script |
| `crabby test` | Run test script |

### Utilities

| Command | Description |
|---------|-------------|
| `crabby --version` | Show version |
| `crabby --help` | Show help |

## 🎯 Features in Detail

### Standalone Runtime

Crabby automatically detects if Node.js is installed. If not, it downloads a portable version (~50MB) to `~/.crabby/runtime/`. This happens once and is reused forever.

```bash
# First run without Node.js
crabby run app.ts
# 📥 Downloading Node.js runtime (one-time setup)...
# ✅ Node.js runtime installed!
# 🍳 Cooking: npx -y tsx app.ts
```

### Dev Dependencies

Separate your development and production dependencies:

```bash
crabby install typescript -D
crabby install jest --save-dev
```

Your `package.json`:
```json
{
  "dependencies": {
    "express": "^5.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.0.0"
  }
}
```

### Package Updates

Stay up to date with the latest versions:

```bash
# Check what's outdated
crabby outdated
# 📊 Outdated packages:
#   express 4.18.0 → 5.2.1
#   typescript 4.9.0 → 5.3.3

# Update specific package
crabby update express

# Check all for updates
crabby update
```

### Install All Dependencies

Just like npm, you can install all dependencies at once:

```bash
crabby install
# 📦 Installing 15 packages...
#   ⬇️  Installing express... ✅ 5.2.1
#   ⬇️  Installing typescript... ✅ 5.3.3
#   ...
# 🎉 Installed 15 packages in 12s
```

## 🔧 Configuration

Create `crabby.config.json` in your project root:

```json
{
  "registry": "https://registry.npmjs.org",
  "log_level": "info"
}
```

## 🏗️ Advanced Features

### Recursive Dependencies

Crabby automatically resolves and installs all nested dependencies, just like npm.

### Lifecycle Scripts

Automatically runs `postinstall` scripts, essential for packages like Electron.

### Binary Linking

Creates executables in `node_modules/.bin` for CLI tools.

### GUI Framework Support

Works seamlessly with Electron, Tauri, and other GUI frameworks.

### Semantic Versioning

Intelligently handles version ranges (`^1.2.0`, `~2.0.0`) to prevent conflicts.

### Global Cache

Packages are cached globally at `~/.crabby/cache/` for faster subsequent installs.

## 📊 Performance

- **TypeScript Execution**: 20x faster than ts-node using tsx
- **Install Speed**: Comparable to npm with global caching
- **Startup Time**: Instant with no runtime overhead
- **Memory Usage**: Efficient Rust implementation

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [tsx](https://github.com/esbuild-kit/tsx) for fast TypeScript execution
- Inspired by npm, yarn, and pnpm

---

<div align="center">

**Built with 🦀 and ❤️ by [AqwozTheDeveloper](https://github.com/AqwozTheDeveloper)**

[Report Bug](https://github.com/AqwozTheDeveloper/crabby/issues) • [Request Feature](https://github.com/AqwozTheDeveloper/crabby/issues)

</div>
