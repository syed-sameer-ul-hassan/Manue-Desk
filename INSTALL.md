# Installation Guide

This guide provides step-by-step instructions for building, installing, and running **Manue Desk** on Linux.

---

## System Requirements

- **Operating System**: Linux (Fedora, Ubuntu, Debian, Arch Linux, Manjaro, openSUSE, etc.)
- **Architecture**: x86_64 or ARM64
- **Dependencies**: Rust 1.70+ and Cargo

---

## 1. Prerequisites Installation

### Fedora / RHEL
```bash
sudo dnf install gcc gcc-c++ make pkg-config fontconfig-devel
```

### Ubuntu / Debian
```bash
sudo apt update
sudo apt install build-essential pkg-config libfontconfig1-dev
```

### Arch Linux / Manjaro
```bash
sudo pacman -S base-devel fontconfig
```

---

## 2. Install Rust Toolchain

If Rust is not installed on your system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompt, then update your current shell session:

```bash
source "$HOME/.cargo/env"
```

Verify your installation:

```bash
cargo --version
rustc --version
```

---

## 3. Build & Install Manue Desk

Clone the repository and build the binary:

```bash
git clone https://github.com/syed-sameer-ul-hassan/ADD-TO-MANUE.git
cd ADD-TO-MANUE

# Build release binary
cargo build --release
```

### System-wide or User Installation

To install the compiled binary into your local executable path (`~/.local/bin`):

```bash
mkdir -p ~/.local/bin
cp target/release/manue-desk ~/.local/bin/
```

Ensure `~/.local/bin` is in your system `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## 4. Building .deb & .rpm Packages

```bash
# Install cargo packaging tools
cargo install cargo-deb cargo-generate-rpm

# Generate Debian/Ubuntu (.deb) package
cargo deb

# Generate Fedora/RHEL (.rpm) package
cargo generate-rpm
```

- **.deb output location**: `target/debian/manue-desk_0.1.0_amd64.deb`
- **.rpm output location**: `target/generate-rpm/manue-desk-0.1.0-1.x86_64.rpm`

---

## 5. Launching the App

Run `manue-desk` directly from your terminal:

```bash
manue-desk
```
