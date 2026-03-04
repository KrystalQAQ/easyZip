# easyZip

High-performance backup tool for frontend projects built with Rust.

## Features

- Fast ZIP compression for code backups
- Automatically excludes node_modules, .git, dist, build, etc.
- Simple CLI interface
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
easyzip /path/to/project --exclude "*.log,temp"
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
