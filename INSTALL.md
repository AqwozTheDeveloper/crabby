# Crabby Installation Guide

## Quick Install

### Windows
```powershell
# Run in PowerShell
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
.\install.ps1
```

### macOS / Linux
```bash
# Run in terminal
git clone https://github.com/AqwozTheDeveloper/crabby.git
cd crabby
chmod +x install.sh
./install.sh
```

## Requirements

- **Rust** (for building from source)
  - Windows: `winget install Rustlang.Rustup`
  - macOS/Linux: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- **Node.js** (optional - auto-downloaded if not present)
  - Crabby will automatically download Node.js if you don't have it installed!

## What Gets Installed

- `crabby` binary → `~/.crabby/bin/` (or `%USERPROFILE%\.crabby\bin\` on Windows)
- Added to your PATH automatically
- Node.js runtime (downloaded on first `crabby run` if needed) → `~/.crabby/runtime/`

## Standalone Features

✅ **No Node.js Required** - Crabby auto-downloads Node.js if needed  
✅ **Fast TypeScript** - Uses tsx for 20x faster execution  
✅ **Full npm Support** - Works with all npm packages  
✅ **Global Cache** - Faster installs with shared cache  

## Verify Installation

```bash
crabby --version
crabby --help
```

## Getting Started

```bash
# Initialize a new project
crabby init

# Install packages
crabby install express
crabby install typescript

# Run TypeScript/JavaScript
crabby run app.ts
crabby run server.js
```

## Uninstall

```bash
# Remove binary
rm -rf ~/.crabby

# Remove from PATH (edit your shell config)
# Remove the line: export PATH="$HOME/.crabby/bin:$PATH"
```

## Troubleshooting

### "command not found: crabby"
- Restart your terminal
- Or run: `source ~/.bashrc` (or `~/.zshrc`)

### Build fails
- Make sure Rust is up to date: `rustup update`
- Try: `cargo clean` then run install again

### Node.js download fails
- Check internet connection
- Crabby will retry on next run
- Or install Node.js manually from nodejs.org

## Uninstall

### Windows
```powershell
cd crabby
.\uninstall.ps1
```

### macOS / Linux
```bash
cd crabby
chmod +x uninstall.sh
./uninstall.sh
```

This will remove:
- Crabby binary from `~/.crabby/bin/`
- Global cache from `~/.crabby/cache/`
- Downloaded Node.js runtime from `~/.crabby/runtime/`
- PATH entry (Windows: automatic, Unix: manual instructions)

## Support

- 📚 Documentation: [GitHub Wiki](https://github.com/AqwozTheDeveloper/crabby/wiki)
- 🐛 Issues: [GitHub Issues](https://github.com/AqwozTheDeveloper/crabby/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/AqwozTheDeveloper/crabby/discussions)
