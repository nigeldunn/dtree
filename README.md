# dtree - Directory Tree Viewer

A simple command-line tool written in Rust that displays directories in a tree format, similar to the Unix `tree` command.

## Features

- Display directories in a hierarchical tree structure
- Optional `-f` flag to include files in the tree view
- Clean, readable output with proper tree formatting
- Sorts directories first, then files alphabetically
- Supports custom directory paths

## Installation

1. Clone or download this project
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```
3. The executable will be available at `./target/release/dtree`

## Usage

### Basic usage (directories only)
```bash
./target/release/dtree
```

### Include files with the -f flag
```bash
./target/release/dtree -f
```

### Specify a custom directory
```bash
./target/release/dtree /path/to/directory
./target/release/dtree -f /path/to/directory
```

### Help
```bash
./target/release/dtree --help
```

## Examples

**Directories only:**
```
.
├── src
└── target
    └── release
        ├── .fingerprint
        ├── build
        ├── deps
        ├── examples
        └── incremental
```

**With files (-f flag):**
```
.
├── src
│   └── main.rs
├── target
│   └── release
│       ├── .fingerprint
│       ├── build
│       ├── deps
│       ├── examples
│       ├── incremental
│       ├── dtree
│       └── dtree.d
├── Cargo.lock
└── Cargo.toml
```

## Command Line Options

- `-f, --files`: Include files in the tree view (default: directories only)
- `-h, --help`: Print help information
- `[PATH]`: Directory to display (default: current directory)

## CI/CD

This project includes a comprehensive GitHub Actions workflow and automated dependency management:

### GitHub Actions Workflow
- **Tests**: Runs code formatting checks, linting with Clippy, and unit tests
- **Multi-platform builds**: Builds binaries for Linux (x86_64), Windows (x86_64), and macOS (x86_64 and ARM64)
- **Security audit**: Runs `cargo audit` to check for known security vulnerabilities
- **Code coverage**: Generates and uploads code coverage reports
- **Release automation**: Automatically creates release artifacts when a new release is published

### Dependabot Configuration
- **Automated dependency updates**: Dependabot automatically checks for updates to Rust crates and GitHub Actions
- **Weekly schedule**: Updates are checked every Monday at 9:00 AM
- **Grouped updates**: Minor and patch updates are grouped together to reduce PR noise
- **Auto-labeling**: PRs are automatically labeled with `dependencies`, `rust`, or `github-actions`
- **Security updates**: Critical security updates are handled with higher priority

### Build Status

The workflow runs on:
- Push to `main` or `master` branches
- Pull requests to `main` or `master` branches  
- When a new release is published

### Release Binaries

When you create a release on GitHub, the workflow will automatically build and attach the following binaries:
- `dtree-linux-amd64.tar.gz` - Linux x86_64
- `dtree-windows-amd64.exe.zip` - Windows x86_64
- `dtree-macos-amd64.tar.gz` - macOS x86_64 (Intel)
- `dtree-macos-arm64.tar.gz` - macOS ARM64 (Apple Silicon)

## Dependencies

- `clap`: Command-line argument parsing

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

