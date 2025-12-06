#!/bin/sh
set -e

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: 'cargo' is not installed. Please install Rust and Cargo first."
    exit 1
fi

echo "Building Crabby from source..."
cargo build --release

INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    echo "Installing to ${INSTALL_DIR}."
    
    # Check if INSTALL_DIR is in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *) echo "Warning: ${INSTALL_DIR} is not in your PATH. You may need to add it." ;;
    esac
else
    echo "Installing to ${INSTALL_DIR}."
fi

BINARY_NAME="crabby"
SOURCE_BIN="target/release/${BINARY_NAME}"

if [ ! -f "$SOURCE_BIN" ]; then
    echo "Error: Build failed. ${SOURCE_BIN} not found."
    exit 1
fi

mv "$SOURCE_BIN" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo "Successfully installed ${BINARY_NAME}!"
