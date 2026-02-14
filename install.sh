#!/bin/sh
set -eu

REPO="mizunomi32/websearch-mcp"
BINARY="websearch-mcp"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

main() {
    check_dependencies
    detect_platform
    fetch_latest_version
    download_and_install
    echo "${BINARY} ${VERSION} has been installed to ${INSTALL_DIR}/${BINARY}"
}

check_dependencies() {
    for cmd in curl tar; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            echo "error: ${cmd} is required but not found" >&2
            exit 1
        fi
    done
}

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)  PLATFORM="linux" ;;
        Darwin) PLATFORM="darwin" ;;
        *)      echo "error: unsupported OS: ${OS}" >&2; exit 1 ;;
    esac

    case "$ARCH" in
        x86_64|amd64)  ARCH_LABEL="amd64" ;;
        aarch64|arm64) ARCH_LABEL="arm64" ;;
        *)             echo "error: unsupported architecture: ${ARCH}" >&2; exit 1 ;;
    esac

    ARTIFACT="${BINARY}-${PLATFORM}-${ARCH_LABEL}"
    echo "Detected platform: ${PLATFORM}/${ARCH_LABEL}"
}

fetch_latest_version() {
    VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d'"' -f4)"

    if [ -z "$VERSION" ]; then
        echo "error: failed to fetch latest version" >&2
        exit 1
    fi
    echo "Latest version: ${VERSION}"
}

download_and_install() {
    URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}.tar.gz"
    TMPDIR="$(mktemp -d)"
    trap 'rm -rf "$TMPDIR"' EXIT

    echo "Downloading ${URL}..."
    curl -fsSL "$URL" | tar xz -C "$TMPDIR"

    if [ -w "$INSTALL_DIR" ]; then
        mv "$TMPDIR/${BINARY}" "$INSTALL_DIR/${BINARY}"
    else
        echo "Installing to ${INSTALL_DIR} (requires sudo)..."
        sudo mv "$TMPDIR/${BINARY}" "$INSTALL_DIR/${BINARY}"
    fi

    chmod +x "$INSTALL_DIR/${BINARY}"
}

main
