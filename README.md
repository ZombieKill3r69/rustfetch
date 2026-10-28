# Rustfetch

## Overview

Rustfetch is a rust based neofetch-like program used to display system information at a glance from the terminal.

## Features

- **Minimal Output** – Shows only essential system info without clutter
- **Blazing Fast** – Significantly faster than Bash-based alternatives like Neofetch
- **Lightweight** – Minimal dependencies, written in Rust for performance
- **Linux Native** – Made for Linux

## Getting Started

### Prerequisites

- Git
- Cargo
- An internet connection (installation only)
- A Linux system

### Installation

#### Automatic
1. **Copy into terminal**
    ```bash
    curl -sSL https://raw.githubusercontent.com/ZombieKill3r69/rustfetch/refs/heads/master/install.sh | bash
    ```
2. **Run command in terminal**
    ```bash
    rustfetch
    ```

#### Manual

1. **Clone the repo**
    ```bash
    $ git clone https://github.com/ZombieKill3r69/rustfetch.git
    $ cd rustfetch
    ```
2. **Compile**
    ```bash
    $ cargo build --release
    ```
3. **Transport the file**
    ```bash
    $ sudo cp target/release/rustfetch /usr/local/bin/
    ```
4. **Run rustfetch**
    ```bash
    $ rustfetch
    ```

## Usage

Simply run:
```bash
rustfetch
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Note:** This is a personal project. While contributions are not actively sought, you're welcome to fork and modify it for your own use.