#!/bin/bash
set -e

echo "Downloading rustfetch..."
git clone https://github.com/ZombieKill3r69/rustfetch.git
cd rustfetch

echo "Comiling..."
cargo build --release

echo "Installing..."
cp target/release/rustfetch /usr/local/bin/

echo "Rustfetch has been installed successfully"
echo "Run "Rustfetch" to use it"
echo "Happy fetching!"