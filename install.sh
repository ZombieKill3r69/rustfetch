#!/bin/bash
set -e

echo "Downloading rustfetch..."
git clone https://github.com/ZombieKill3r69/rustfetch.git
cd rustfetch

echo "Compiling..."
cargo build --release

echo "Installing..."
sudo cp target/release/rustfetch /usr/local/bin/

echo "Cleaning up"
rm -rf ../rustfetch

echo "Rustfetch has been installed successfully"
echo "Run 'rustfetch' to use it"
echo "Happy fetching!"
