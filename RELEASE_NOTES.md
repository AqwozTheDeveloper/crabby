# Crabby Release Notes

## Version 0.1.0 - Initial Release

### 🎉 Major Features

#### Standalone Runtime
- **No Node.js Required**: Crabby works out of the box without Node.js installed
- **Auto-Download**: Automatically downloads portable Node.js (~50MB) if not found
- **One-Time Setup**: Downloaded runtime is cached at `~/.crabby/runtime/` and reused
- **Cross-Platform**: Works on Windows, macOS, and Linux

#### Package Management
- **Install Packages**: `crabby install <package>` - Install from npm registry
- **Install All**: `crabby install` - Install all dependencies from package.json
- **Remove Packages**: `crabby remove <package>` - Clean removal with lock file updates
- **List Packages**: `crabby list` - View all installed packages
- **Clean Command**: `crabby clean` - Remove node_modules with confirmation prompt

#### Dev Dependencies
- **Separate Dependencies**: Full support for `devDependencies`
- **Install Flag**: `crabby install <pkg> --save-dev` or `-D`
- **Proper Tracking**: Maintains separate dependency lists in package.json

#### Package Updates & Information
- **Check Updates**: `crabby update` - See all available updates
- **Update Packages**: `crabby update <package>` - Update specific package
- **Show Outdated**: `crabby outdated` - List packages with newer versions
- **Package Info**: `crabby info <package>` - View package details from registry

#### Script Running
- **TypeScript Support**: 20x faster execution with tsx
- **JavaScript Support**: Direct Node.js execution
- **Package Scripts**: Run scripts from package.json
- **Quick Aliases**: `crabby start`, `crabby test`

#### Advanced Features
- **Recursive Dependencies**: Automatic deep dependency resolution
- **Semantic Versioning**: Smart version range handling (`^`, `~`)
- **Lifecycle Scripts**: Automatic `postinstall` execution
- **Binary Linking**: CLI tools in `node_modules/.bin`
- **Lock Files**: `crabby.lock` for reproducible builds
- **Global Cache**: Shared package cache at `~/.crabby/cache/`
- **GUI Framework Support**: Works with Electron, Tauri, etc.

#### Workspace Support (Basic)
- **Workspace Discovery**: Parses workspace patterns from package.json
- **Package Linking**: Symlinks local workspace packages
- **Monorepo Ready**: Basic monorepo support

### 🎨 User Experience

#### Beautiful CLI
- **Emojis**: Visual feedback with emojis (📦, ✅, ❌, etc.)
- **Colors**: Syntax highlighting with colored output
- **Progress Indicators**: Real-time installation progress
- **Clear Messages**: Helpful error messages and suggestions

#### Smart Defaults
- **Auto-Detection**: Automatically detects .ts vs .js files
- **Confirmation Prompts**: Safety checks for destructive operations
- **Error Handling**: Graceful failure with detailed error messages
- **Retry Logic**: Continues installing even if one package fails

### 📊 Performance

- **Fast TypeScript**: 20x faster than ts-node using tsx
- **Efficient Caching**: Global cache reduces download times
- **Rust Performance**: Native speed with low memory usage
- **Parallel Operations**: Async operations with Tokio

### 🔧 Technical Details

#### Dependencies
- Built with Rust 2021 edition
- Uses Tokio for async operations
- Clap for CLI parsing
- Serde for JSON handling
- Reqwest for HTTP requests

#### Platform Support
- **Windows**: Full support with PowerShell installer
- **macOS**: Full support with bash installer
- **Linux**: Full support with bash installer

#### File Structure
```
~/.crabby/
├── bin/           # Crabby executable
├── cache/         # Global package cache
└── runtime/       # Portable Node.js (if downloaded)
```

### 📝 Commands Reference

**16+ Commands Available:**
- `init`, `install`, `remove`, `list`, `clean`
- `update`, `outdated`, `info`
- `run`, `start`, `test`
- `--version`, `--help`

### 🐛 Known Limitations

- **Workspaces**: Basic support only (no shared dependencies yet)
- **Audit**: No security audit feature (planned for future)
- **Publishing**: No `publish` command (use npm for now)
- **Parallel Downloads**: Sequential installation (parallel planned)

### 🚀 Future Plans

- Full workspace support with shared dependencies
- Security audit integration
- Publishing to npm
- Parallel package downloads
- Watch mode for development
- Performance benchmarks

### 📚 Documentation

- **README.md**: Complete feature documentation
- **INSTALL.md**: Installation guide
- **Examples**: Sample projects in repository

### 🙏 Credits

- Built with Rust by AqwozTheDeveloper
- Inspired by npm, yarn, and pnpm
- Uses tsx for TypeScript execution
- Community feedback and contributions

---

## Installation

```bash
# Windows
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
.\install.ps1

# macOS/Linux
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
./install.sh
```

## Getting Started

```bash
crabby init
crabby install express
crabby run app.ts
```

---

**Thank you for using Crabby! 🦀**

Report issues: https://github.com/AqwozTheDeveloper/crabby/issues
