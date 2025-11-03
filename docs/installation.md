# Installation Guide

This guide provides detailed instructions for installing the Aws provider for Hemmer.

---

## Prerequisites

- **Hemmer CLI** (recommended) or manual binary installation
- Supported platforms:
  - macOS (Intel x86_64 or Apple Silicon ARM64)
  - Linux (x86_64 or ARM64)
  - Windows (x86_64)

---

## Method 1: Using Hemmer CLI (Recommended)

The easiest way to install the provider is using the Hemmer CLI:

```bash
hemmer provider install aws
```

This command will:
1. Detect your platform automatically
2. Download the correct pre-built binary from the latest GitHub release
3. Verify the checksum
4. Install the provider to `~/.hemmer/providers/`

### Verify Installation

```bash
hemmer provider list
```

You should see `aws` in the list of installed providers.

---

## Method 2: Manual Installation

### Step 1: Download Binary

Download the appropriate binary for your platform from the [Releases](../../releases) page:

| Platform | Binary Name |
|----------|-------------|
| macOS (Intel) | `hemmer-provider-aws-darwin-amd64` |
| macOS (Apple Silicon) | `hemmer-provider-aws-darwin-arm64` |
| Linux (x86_64) | `hemmer-provider-aws-linux-amd64` |
| Linux (ARM64) | `hemmer-provider-aws-linux-arm64` |
| Windows (x86_64) | `hemmer-provider-aws-windows-amd64.exe` |

### Step 2: Verify Checksum (Recommended)

Download `checksums.txt` from the same release and verify:

**macOS/Linux:**
```bash
sha256sum -c checksums.txt
```

**Windows (PowerShell):**
```powershell
Get-FileHash -Algorithm SHA256 hemmer-provider-aws-windows-amd64.exe
# Compare with value in checksums.txt
```

### Step 3: Install Binary

Move the binary to your Hemmer providers directory:

**macOS/Linux:**
```bash
# Create providers directory if it doesn't exist
mkdir -p ~/.hemmer/providers

# Move and rename binary
mv hemmer-provider-aws-* ~/.hemmer/providers/hemmer-provider-aws

# Make executable
chmod +x ~/.hemmer/providers/hemmer-provider-aws
```

**Windows (PowerShell):**
```powershell
# Create providers directory if it doesn't exist
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.hemmer\providers

# Move binary
Move-Item hemmer-provider-aws-windows-amd64.exe $env:USERPROFILE\.hemmer\providers\hemmer-provider-aws.exe
```

### Step 4: Verify Installation

**macOS/Linux:**
```bash
~/.hemmer/providers/hemmer-provider-aws --version
```

**Windows (PowerShell):**
```powershell
& "$env:USERPROFILE\.hemmer\providers\hemmer-provider-aws.exe" --version
```

---

## Method 3: Build from Source

If you prefer to build the provider from source:

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Git

### Build Steps

```bash
# Clone the repository
git clone https://github.com/YOUR_ORG/hemmer-provider-aws.git
cd hemmer-provider-aws

# Build in release mode
cargo build --release

# The binary will be at:
# - macOS: target/release/libhemmer_aws_provider.dylib
# - Linux: target/release/libhemmer_aws_provider.so
# - Windows: target/release/hemmer_aws_provider.dll
```

### Install

Copy the built binary to your Hemmer providers directory:

**macOS:**
```bash
mkdir -p ~/.hemmer/providers
cp target/release/libhemmer_aws_provider.dylib \
   ~/.hemmer/providers/hemmer-provider-aws
chmod +x ~/.hemmer/providers/hemmer-provider-aws
```

**Linux:**
```bash
mkdir -p ~/.hemmer/providers
cp target/release/libhemmer_aws_provider.so \
   ~/.hemmer/providers/hemmer-provider-aws
chmod +x ~/.hemmer/providers/hemmer-provider-aws
```

**Windows:**
```powershell
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.hemmer\providers
Copy-Item target\release\hemmer_aws_provider.dll `
          $env:USERPROFILE\.hemmer\providers\hemmer-provider-aws.exe
```

---

## Troubleshooting

### Binary Not Found

If Hemmer can't find the provider, ensure:
1. The binary is in `~/.hemmer/providers/` (or `%USERPROFILE%\.hemmer\providers\` on Windows)
2. The binary is executable (`chmod +x` on macOS/Linux)
3. The binary name is correct: `hemmer-provider-aws`

### Permission Denied (macOS)

macOS may block the binary due to security settings:

```bash
xattr -d com.apple.quarantine ~/.hemmer/providers/hemmer-provider-aws
```

### Checksum Mismatch

If the checksum doesn't match:
1. Re-download the binary (it may have been corrupted)
2. Ensure you're using the correct checksum from `checksums.txt`
3. Report the issue on GitHub if the problem persists

---

## Upgrading

To upgrade to a newer version:

**Using Hemmer CLI:**
```bash
hemmer provider upgrade aws
```

**Manual:**
1. Download the new version
2. Replace the existing binary in `~/.hemmer/providers/`
3. Verify the installation

---

## Uninstalling

**Using Hemmer CLI:**
```bash
hemmer provider uninstall aws
```

**Manual:**
```bash
rm ~/.hemmer/providers/hemmer-provider-aws
```

---

## Next Steps

- 🚀 [Getting Started Guide](getting-started.md)
- 📚 [Service Documentation](services/)
- ⬅️ [Back to README](../README.md)
