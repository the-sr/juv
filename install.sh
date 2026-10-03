#!/usr/bin/env bash
# install.sh - Downloads and installs juv on the user's system.
# This script detects the operating system and architecture,
# downloads the correct binary, and sets it up.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# GitHub repository details
REPO_OWNER="the-sr"
REPO_NAME="juv"
VERSION="v0.1.1"

# Installation directory
INSTALL_DIR="${HOME}/.local/bin"

echo -e "${CYAN}Installing juv...${NC}"

# Detect operating system
OS="$(uname -s)"
case "${OS}" in
    Linux*)     OS="linux";;
    Darwin*)    OS="darwin";;
    *)          echo -e "${RED}Unsupported operating system: ${OS}${NC}"; exit 1;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64*)    ARCH="x86_64";;
    arm64*)     ARCH="aarch64";;
    aarch64*)   ARCH="aarch64";;
    *)          echo -e "${RED}Unsupported architecture: ${ARCH}${NC}"; exit 1;;
esac

# Build the target triple (must match the naming used in GitHub Actions)
case "${OS}-${ARCH}" in
    linux-x86_64)   TARGET="x86_64-unknown-linux-gnu";;
    linux-aarch64)  TARGET="aarch64-unknown-linux-gnu";;
    darwin-x86_64)  TARGET="x86_64-apple-darwin";;
    darwin-aarch64) TARGET="aarch64-apple-darwin";;
    *)              echo -e "${RED}Unsupported platform: ${OS}-${ARCH}${NC}"; exit 1;;
esac

# Build the download URL
DOWNLOAD_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${VERSION}/juv-${TARGET}.tar.gz"

echo -e "${CYAN}Downloading juv for ${TARGET}...${NC}"

# Create a temporary directory for the download
TEMP_DIR="$(mktemp -d)"
trap "rm -rf ${TEMP_DIR}" EXIT

# Download the binary
curl -LsSf "${DOWNLOAD_URL}" -o "${TEMP_DIR}/juv.tar.gz"

# Extract the archive
echo -e "${CYAN}Extracting...${NC}"
tar -xzf "${TEMP_DIR}/juv.tar.gz" -C "${TEMP_DIR}"

# Create the install directory if it does not exist
mkdir -p "${INSTALL_DIR}"

# Move the binary to the install directory
echo -e "${CYAN}Installing to ${INSTALL_DIR}/juv...${NC}"
mv "${TEMP_DIR}/juv" "${INSTALL_DIR}/juv"
chmod +x "${INSTALL_DIR}/juv"

# Add to PATH if needed
if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
    echo -e "${YELLOW}Adding ${INSTALL_DIR} to PATH...${NC}"
    echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "${HOME}/.bashrc"
    echo -e "${YELLOW}Please run: source ~/.bashrc${NC}"
fi

echo -e "${GREEN}juv installed successfully!${NC}"
echo -e "${CYAN}Run 'juv --help' to get started.${NC}"
