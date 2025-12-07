# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

biomers is a Rust wrapper for NIST's NBIS (NIST Biometric Image Software) via libbiomeval. It provides safe Rust bindings for WSQ (Wavelet Scalar Quantization) encoding/decoding - the FBI standard for fingerprint image compression.

## Build Commands

```bash
# Build the library (requires CMake, Clang/LLVM for bindgen)
cargo build

# Run the demo example
cargo run --example demo

# Run tests
cargo test
```

## Build Requirements

- **CMake**: Required to build the libbiomeval C library
- **Clang/LLVM**: Required by bindgen for generating FFI bindings

### System Packages

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
├── build.rs          # CMake build + bindgen FFI generation
├── wrapper.h         # C headers to expose via bindgen
├── libbiomeval/      # Git submodule: NIST libbiomeval (contains NBIS)
└── src/
    ├── lib.rs        # Public API re-exports
    ├── sys.rs        # Raw FFI bindings (auto-generated)
    ├── wsq.rs        # Safe WSQ encode/decode wrappers
    └── jpeg.rs       # Safe JPEGL encode wrapper
```

### Build Process

1. `build.rs` uses CMake to compile libbiomeval into a single `biomeval.lib` static library
2. On Windows with VCPKG_ROOT set, automatically configures CMAKE_TOOLCHAIN_FILE
3. bindgen generates Rust FFI bindings from `wrapper.h` into `$OUT_DIR/bindings.rs`
4. `sys.rs` includes the generated bindings (functions prefixed with `biomeval_nbis_`)

### Public API

- `wsq_encode(raw_data, width, height, bitrate)` - Encode grayscale pixels to WSQ format
- `wsq_decode(wsq_data)` - Decode WSQ to raw pixels, returns (pixels, width, height)

All functions handle C memory allocation/deallocation internally and return Rust `Vec<u8>`.

### Windows Runtime

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
