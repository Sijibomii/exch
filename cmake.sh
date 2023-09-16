#!/bin/bash

# Check if the user has sudo privileges
if [ "$EUID" -ne 0 ]; then
  echo "Please run this script with sudo or as the root user."
  exit 1
fi

# Define the CMake version and download URL
CMAKE_VERSION="3.27.5"
DOWNLOAD_URL="https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-linux-x86_64.tar.gz"

# Define the installation directory (change this if needed)
INSTALL_DIR="/usr/local"

# Create a temporary directory for downloading and extracting CMake
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR" || exit 1

# Download CMake tar.gz archive
echo "Downloading CMake $CMAKE_VERSION..."
wget "$DOWNLOAD_URL" || { echo "Failed to download CMake."; exit 1; }

# Extract the archive
tar -xzf "cmake-$CMAKE_VERSION-linux-x86_64.tar.gz"

# Copy CMake binaries to the installation directory
cp -r "cmake-$CMAKE_VERSION-linux-x86_64/." "$INSTALL_DIR/"

# Add the CMake binary directory to the system's PATH
echo "export PATH=$INSTALL_DIR/bin:\$PATH" >> /etc/profile

# Verify the installation
source /etc/profile
cmake --version

# Clean up
cd "$TMP_DIR"
rm -rf "$TMP_DIR"

echo "CMake $CMAKE_VERSION has been successfully installed to $INSTALL_DIR."