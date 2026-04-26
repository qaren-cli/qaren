# Installing Qaren

Qaren is distributed as a statically linked binary, ensuring a zero-dependency footprint on your host machine.

## Automated Installation (Recommended)

The easiest way to install Qaren is via our automated installer scripts which detect your platform and architecture automatically.

### Linux & macOS
```bash
curl -sSfL https://qaren.me/install | sh
```

### Windows (PowerShell)
```powershell
irm https://qaren.me/install.ps1 | iex
```

### Homebrew
```bash
brew tap qaren-cli/qaren
brew install qaren
```

## Manual Installation

If you prefer to handle things manually, you can download the latest release for your platform from the [GitHub Releases](https://github.com/qaren-cli/qaren/releases) page.

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
