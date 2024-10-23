# compression_rs

A simple Rust command-line tool to compress and decompress files using Gzip.

## Features
- **Compression**: Compress any file using Gzip.
- **Decompression**: Decompress Gzip files.
- **Performance Timing**: Displays the time taken for each operation.

## Usage

### Build the project

First, clone the repository and build the project using `cargo`:

```bash
git clone https://github.com/your-username/rs_compression.git
cd rs_compression
cargo build --release
```

### Run the tool

#### Compress a file:
```bash
cargo run --release -- -c <source_file> <output_file>
```

#### Decompress a file:
```bash
cargo run --release -- -d <source_file> <output_file>
```

### Example:
To compress a file:
```bash
cargo run --release -- -c example.txt example.txt.gz
```

To decompress a file:
```bash
cargo run --release -- -d example.txt.gz decompressed.txt
```

## Requirements

- Rust (latest stable version)
