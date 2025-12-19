#!/bin/bash
# Crabby Package Manager Installer
# Works on macOS and Linux

set -e

echo "🦀 Installing Crabby Package Manager..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "📥 Please install Rust from: https://rustup.rs/"
    echo ""
    echo "Run this command:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"
echo ""

# Build Crabby
echo "🔨 Building Crabby..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Determine install location
INSTALL_DIR="$HOME/.crabby/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "📦 Installing to $INSTALL_DIR..."
cp target/release/crabby "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/crabby"

echo "✅ Crabby installed!"
echo ""

# Check if already in PATH
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo "✅ $INSTALL_DIR is already in PATH"
else
    echo "⚠️  Add Crabby to your PATH:"
    echo ""
    
    # Detect shell
    if [ -n "$BASH_VERSION" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
    
    echo "Add this line to $SHELL_RC:"
    echo "export PATH=\"\$HOME/.crabby/bin:\$PATH\""
    echo ""
    echo "Then run: source $SHELL_RC"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "📝 Key Features:"
echo "  ✅ Standalone - No Node.js required!"
echo "  ✅ Fast TypeScript execution with tsx"
echo "  ✅ Full npm ecosystem support"
echo "  ✅ Global package support (install -g)"
echo "  ✅ Security auditing (audit)"
echo ""
echo "🚀 Get started:"
echo "  crabby init              # Initialize a new project"
echo "  crabby add react         # Add a package"
echo "  crabby exec tsc --init   # Run binaries (or use 'x')"
echo "  crabby install -g tool   # Install global CLI tools"
echo "  crabby audit             # Check vulnerabilities"
echo "  crabby run app.ts        # Run TypeScript files"
echo ""
echo "📚 Learn more: https://github.com/AqwozTheDeveloper/crabby"
