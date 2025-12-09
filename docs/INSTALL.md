# Installing the TDF CLI Tool

## Option 1: Use from Build Directory (Quick)

The binary is already built at:
```
./target/release/tdf
```

Run it directly:
```bash
./target/release/tdf info document.tdf
./target/release/tdf verify document.tdf
./target/release/tdf extract document.tdf -o data.json
```

## Option 2: Add to PATH (Recommended)

### Temporary (Current Session)
```bash
export PATH="$PATH:$(pwd)/target/release"
tdf info document.tdf
```

### Permanent (Add to ~/.bashrc or ~/.zshrc)
```bash
echo 'export PATH="$PATH:/home/n00b73/DevLab/TrustDoc/target/release"' >> ~/.bashrc
source ~/.bashrc
tdf info document.tdf
```

## Option 3: Install System-Wide

### Copy to /usr/local/bin
```bash
sudo cp target/release/tdf /usr/local/bin/
tdf info document.tdf
```

### Or to ~/.local/bin (User-only)
```bash
mkdir -p ~/.local/bin
cp target/release/tdf ~/.local/bin/
export PATH="$PATH:$HOME/.local/bin"  # Add to ~/.bashrc for permanent
tdf info document.tdf
```

## Option 4: Create Alias

Add to `~/.bashrc` or `~/.zshrc`:
```bash
alias tdf='/home/n00b73/DevLab/TrustDoc/target/release/tdf'
```

Then:
```bash
source ~/.bashrc
tdf info document.tdf
```

## Option 5: Build and Install with Cargo

```bash
cargo install --path tdf-cli
```

This installs to `~/.cargo/bin/tdf` (make sure `~/.cargo/bin` is in your PATH).

## Verify Installation

```bash
tdf --version
tdf --help
```

## Quick Test

```bash
# Using full path
./target/release/tdf info demo-invoice.tdf

# Or after adding to PATH
tdf info demo-invoice.tdf
```

