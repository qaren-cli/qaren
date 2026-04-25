# Installing Qaren

Qaren is distributed as a statically linked binary, ensuring a zero-dependency footprint on your host machine.

## Pre-compiled Binaries (Recommended)

Download the latest release for your platform and place it in your executable `$PATH`.

```bash
# Download the latest Linux AMD64 release
curl -LO https://github.com/alielesawy/qaren/releases/latest/download/qaren-linux-amd64.tar.gz

# Extract the archive
tar -xzf qaren-linux-amd64.tar.gz

# Move binary to local bin
sudo mv qaren /usr/local/bin/

# Verify installation
qaren --version
```

## Building from Source

For those who prefer to compile from source, you must have the Rust toolchain installed.

```bash
# Clone the repository
git clone https://github.com/alielesawy/qaren.git
cd qaren

# Build the release binary
cargo build --release

# Install to path
sudo cp target/release/qaren /usr/local/bin/
```

---
[Return to Index](README.md) | [Next: Qaren vs POSIX Diff](concepts/qaren-vs-posix-diff.md)
