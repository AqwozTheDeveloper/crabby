#!/bin/bash
# Crabby Package Manager Uninstaller
# Works on macOS and Linux

echo "Uninstalling Crabby Package Manager..."
echo ""

INSTALL_DIR="$HOME/.crabby"
BIN_DIR="$INSTALL_DIR/bin"

# Ask for confirmation
echo "This will remove:"
echo "  - Crabby binary from $BIN_DIR"
echo "  - Global cache from $INSTALL_DIR/cache"
echo "  - Runtime from $INSTALL_DIR/runtime"
echo ""
read -p "Continue? (y/n): " confirmation

if [ "$confirmation" != "y" ] && [ "$confirmation" != "Y" ]; then
    echo "[CANCELLED] Uninstall cancelled"
    exit 0
fi

echo ""

# Remove Crabby directory
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing $INSTALL_DIR..."
    rm -rf "$INSTALL_DIR"
    echo "[OK] Removed Crabby files"
else
    echo "[INFO] Crabby directory not found"
fi

# Instructions for removing from PATH
echo ""
echo "To remove Crabby from your PATH:"
echo ""

# Detect shell
if [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
else
    SHELL_RC="$HOME/.profile"
fi

echo "Edit $SHELL_RC and remove this line:"
echo "export PATH=\"\$HOME/.crabby/bin:\$PATH\""
echo ""
echo "Then run: source $SHELL_RC"
echo ""

echo "Uninstall complete!"
echo ""
echo "Thank you for using Crabby!"
