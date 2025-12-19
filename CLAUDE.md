# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

biomers is a Rust wrapper for NIST's NBIS (NIST Biometric Image Software) via libbiomeval. It provides safe Rust bindings for WSQ (Wavelet Scalar Quantization) encoding/decoding - the FBI standard for fingerprint image compression.

## Build Modes

The library supports two build modes via Cargo features:

### Minimal Build (Default) - WSQ Only
Zero external dependencies. Works on all platforms.

```bash
# Default minimal build (no external libraries needed)
cargo build

# Explicit minimal build
cargo build --features minimal
```

### Full Build - Complete libbiomeval
Requires CMake and many external libraries.

```bash
# Full build with all image format support
cargo build --features full --no-default-features
```

## Build Requirements

### Minimal Build (Default)
- **C Compiler**: GCC, Clang, or MSVC
- **Clang/LLVM**: Required by bindgen for generating FFI bindings

**No other external libraries required!**

### Full Build Requirements
- **CMake**: Required to build the full libbiomeval C library
- **Clang/LLVM**: Required by bindgen
- **System packages**: See "Full Build System Packages" section below

## Platform Dependencies Summary

| Platform | Minimal Build | Full Build |
|----------|--------------|------------|
| **Windows** | MSVC CRT only | OpenSSL, JPEG, PNG, TIFF, OpenJPEG, zlib, lzma, SQLite, BerkeleyDB (via vcpkg) |
| **Linux** | libc, libm (standard) | All above + libstdc++ |
| **macOS** | libc, libm (standard) | All above + Security framework |
| **iOS** | C++ runtime | N/A (use minimal) |
| **Android** | NDK runtime | N/A (use minimal) |

## Build Commands

```bash
# Build the library (minimal, no external deps)
cargo build

# Run the demo example
cargo run --example demo

# Run tests
cargo test

# Build for release
cargo build --release

# Cross-compile for iOS (minimal build)
cargo build --target aarch64-apple-ios

# Cross-compile for Android (minimal build)
cargo build --target aarch64-linux-android
```

## Full Build System Packages

Only needed if using `--features full`:

**Ubuntu:**
```bash
apt install libpcsclite-dev libssl-dev libopenjp2-7-dev libjpeg-dev libpng-dev libtiff-dev zlib1g-dev libopenmpi-dev libdb++-dev libsqlite3-dev libhwloc-dev libavcodec-dev libavformat-dev libswscale-dev
```

**macOS (Homebrew):**
```bash
brew install openjpeg jpeg-turbo libpng libtiff zlib open-mpi berkeley-db sqlite hwloc ffmpeg
```

**macOS (MacPorts):**
```bash
port install openjpeg jpeg libpng tiff zlib openmpi db62 sqlite3 hwloc ffmpeg
```

**RHEL/CentOS:**
```bash
dnf install openssl-devel pcsc-lite-devel openjpeg2-devel libjpeg-turbo-devel libpng-devel libtiff-devel zlib-devel openmpi-devel libdb-cxx-devel sqlite-devel
```

**Windows (vcpkg):**
```
vcpkg install openssl openjpeg libjpeg-turbo libpng tiff zlib msmpi berkeleydb sqlite3 hwloc ffmpeg
```

**Notes:**
- RHEL/CentOS 8: Enable PowerTools repo with `sudo yum config-manager --set-enabled PowerTools`
- Windows: Pass vcpkg toolchain to CMake: `cmake .. -DCMAKE_TOOLCHAIN_FILE=%VCPKG_ROOT%\scripts\buildsystems\vcpkg.cmake`

## Architecture

```
biomers/
├── build.rs          # Build script: minimal (cc) or full (CMake)
├── wrapper.h         # C headers to expose via bindgen
├── Cargo.toml        # Features: minimal (default), full
├── libbiomeval/      # Git submodule: NIST libbiomeval (contains NBIS)
└── src/
    ├── lib.rs        # Public API re-exports
    ├── sys.rs        # Raw FFI bindings (auto-generated)
    ├── wsq.rs        # Safe WSQ encode/decode wrappers
    └── jpeg.rs       # Safe JPEGL encode wrapper
```

### Build Process

**Minimal Build (default):**
1. `build.rs` compiles only WSQ/JPEGL C files directly using `cc` crate
2. Creates static library `nbis_wsq` with ~40 C source files (~18,000 lines)
3. bindgen generates Rust FFI bindings from `wrapper.h`
4. No external libraries linked

**Full Build (--features full):**
1. `build.rs` uses CMake to compile full libbiomeval
2. Links against external image libraries (JPEG, PNG, TIFF, etc.)
3. bindgen generates Rust FFI bindings from `wrapper.h`

### Public API

- `wsq_encode(raw_data, width, height, bitrate)` - Encode grayscale pixels to WSQ format
- `wsq_decode(wsq_data)` - Decode WSQ to raw pixels, returns (pixels, width, height)

All functions handle C memory allocation/deallocation internally and return Rust `Vec<u8>`.

### Windows Runtime (Full Build Only)

Copy vcpkg DLLs to executable directory:
```
copy %VCPKG_ROOT%\installed\x64-windows\bin\*.dll target\debug\
copy "C:\Program Files\OpenSSL-Win64\bin\libcrypto-3-x64.dll" target\debug\
```

## Important Considerations

### Memory Management
NBIS functions often use `malloc` internally and return a pointer. In Rust, we copy this data into a `Vec<u8>` and immediately call `libc::free` on the C pointer to avoid memory leaks.

### Concurrency
Most NBIS functions are thread-safe if they don't rely on global state. However, old C libraries sometimes use global error buffers. Test thoroughly if using in a multi-threaded web server.

### JPEG 2000 (JP2)
The NBIS folder structure (an2k, jpegb, jpegl, wsq) does not include a JP2 encoder. If you need `.jp2` (ISO 15444), you must link libopenjpeg separately and write a wrapper for it. NBIS an2k can wrap binary JP2 data but delegates actual compression to external libraries.

### Other Notes
- Input images are 8-bit grayscale only
- WSQ encoding uses 500 PPI (standard for biometrics)
- Minimal build includes JPEGL (lossless JPEG) in addition to WSQ
