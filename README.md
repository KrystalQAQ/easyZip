# easyZip

High-performance backup tool for frontend projects built with Rust.

## Features

- Fast ZIP compression with progress bar
- Stream-based compression (memory efficient)
- Automatically excludes node_modules, .git, dist, build, etc.
- Glob pattern support (*.log, *.tmp)
- Configuration file support (.easyzip.toml)
- Compression statistics
- Dry-run mode
- Preserves directory structure

## Installation

```bash
cargo build --release
```

## Usage

Basic backup:
```bash
easyzip /path/to/project
```

Specify output file:
```bash
easyzip /path/to/project -o backup.zip
```

Custom exclude patterns:
```bash
easyzip /path/to/project --exclude "*.log,temp,*.tmp"
```

Dry run (preview files):
```bash
easyzip /path/to/project --dry-run
```

## Configuration File

Create `.easyzip.toml` in your project root:

```toml
exclude_patterns = [
    "node_modules",
    "dist",
    "*.log",
    ".env"
]
```

## Default Excluded Patterns

- node_modules
- dist
- build
- .git
- .next
- .nuxt
- coverage
- .cache
- .DS_Store
- *.log
