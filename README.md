# unrpyc-rs

A high-performance Ren'Py decompiler and RPA archive extractor written in Rust. This project aims to provide a fast, memory-safe alternative to the original Python-based `unrpyc`.

## Features

- **RPYC Decompiler**: Decompress and parse Ren'Py compiled script files (`.rpyc`).
- **RPA Extractor**: Extract files from Ren'Py Archive (`.rpa`) versions 2.0 and 3.0.
- **High Performance**: Leverages Rust's speed for batch processing large game projects.
- **FFI Support**: Can be compiled as a shared library (`.so`, `.dll`, `.dylib`) for use in C++, Python, or other languages.

## Installation

### From Releases
Download the pre-compiled binaries for your platform (Linux, Windows, macOS) from the [Releases](https://github.com/NST-Ghost/unrpyc-rs/releases) page.

### Building from Source
Ensure you have the [Rust toolchain](https://rustup.rs/) installed:

```bash
git clone https://github.com/NST-Ghost/unrpyc-rs.git
cd unrpyc-rs
cargo build --release
```
The binary will be located at `target/release/unrpyc_rs`.

## Usage (CLI)

### Decompile .rpyc
```bash
# Process a single file
unrpyc_rs path/to/script.rpyc

# Process all .rpyc files in a folder
unrpyc_rs path/to/game/game/

# Process all .rpyc files in a folder and its subfolders
unrpyc_rs path/to/game/game/ --recursive
```
*Note: Currently outputs the internal AST structure to the console. Full .rpy source generation is under development.*

### Extract .rpa Archive
```bash
unrpyc_rs extract path/to/archive.rpa --output ./extracted_files
```

### Dump Structure
To see the raw unpickled data structure:
```bash
unrpyc_rs --dump path/to/script.rpyc
```

## Integration

### As a Rust Library
Add this to your `Cargo.toml`:
```toml
[dependencies]
unrpyc_rs = { git = "hhttps://github.com/NST-Ghost//unrpyc-rs.git" }
```

### As a C-Compatible Library (FFI)
Build the project to generate shared/static libraries:
```bash
cargo build --release
```
Look for `libunrpyc_rs.so` (Linux), `unrpyc_rs.dll` (Windows), or `libunrpyc_rs.dylib` (macOS) in `target/release/`.

**Header Example (C++):**
```cpp
extern "C" {
    // Extract RPA archive
    int unrpyc_extract_rpa(const char* archive_path, const char* output_dir);
    
    // Decompile RPYC (Returns 0 on success)
    int unrpyc_decompile(const char* input_path, const char* output_path);
}
```

## License
MIT
