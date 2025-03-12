#!/usr/bin/env bash
# Exit on error
set -e

# Install OpenSSL dependencies
apt-get update
apt-get install -y pkg-config libssl-dev

# Build the project
cargo build --release