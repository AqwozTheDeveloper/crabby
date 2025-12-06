#!/bin/sh
# Crabby Git Installer for Unix/Linux/macOS
set -e

REPO_URL="https://github.com/AqwozTheDeveloper/crabby.git"
INSTALL_DIR="$HOME/.crabby"

echo "🦀 Crabby Git Installer"
echo "======================="
echo ""

# Check if git is installed
if ! command -v git >/dev/null 2>&1; then
    echo "Error: 'git' is not installed. Please install git first."
    exit 1
fi

# Check if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: 'cargo' is not installed. Please install Rust from https://rustup.rs/ first."
    exit 1
fi

# Clone or update repository
if [ -d "$INSTALL_DIR" ]; then
    echo "📦 Updating existing Crabby installation..."
    cd "$INSTALL_DIR"
    git pull origin main
else
    echo "📦 Cloning Crabby repository..."
    git clone "$REPO_URL" "$INSTALL_DIR"
fi

# Change to the cloned directory
echo "📂 Entering directory: $INSTALL_DIR"
cd "$INSTALL_DIR"

echo ""
echo "🔨 Building and installing Crabby..."
echo ""

# Run the install script
chmod +x install.sh
./install.sh

echo ""
echo "✅ Crabby installation complete!"
echo ""
echo "Run 'crabby --help' to get started."
