# biomers

Rust wrapper for NIST's [NBIS (Biometric Image Software)](https://www.nist.gov/services-resources/software/nist-biometric-image-software-nbis), providing safe bindings for biometric image format operations.

## Features

- **WSQ Encoding/Decoding** - Wavelet Scalar Quantization, the FBI standard for fingerprint image compression

## Requirements

### Build Dependencies

- **Rust** (edition 2024)
- **CMake** - Required to build the libbiomeval C library
- **Clang/LLVM** - Required by bindgen for generating FFI bindings

### System Packages

The following package manager commands install all packages needed to support all features of Biometric Evaluation Framework.

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

#### Notes

- **RHEL/CentOS 8**: Several packages are in the "PowerTools" repository, disabled by default. Enable with:
  ```bash
  sudo yum config-manager --set-enabled PowerTools
  ```
- **Windows (vcpkg)**: Provide CMake with the vcpkg toolchain path:
  ```
  cmake .. -DCMAKE_TOOLCHAIN_FILE=%VCPKG_ROOT%\scripts\buildsystems\vcpkg.cmake
  ```

#### Module Requirements

| Module | RHEL/CentOS | MacPorts/Homebrew | Ubuntu | vcpkg |
|:------:|:-----------:|:-----------------:|:------:|:-----:|
| OpenSSL (CORE) | `openssl-devel` | n/a (CommonCrypto) | `libssl-dev` | `openssl` |
| PCSC Lite (DEVICE) | `pcsc-lite-devel` | n/a (Command Line Tools) | `libpcsclite-dev` | - |
| OpenJPEG 2.x (IMAGE) | `openjpeg2-devel` | `openjpeg` | `libopenjp2-7-dev` | `openjpeg` |
| libjpeg (IMAGE) | `libjpeg-turbo-devel` | `jpeg` / `jpeg-turbo` | `libjpeg-dev` | `libjpeg-turbo` |
| libpng (IMAGE) | `libpng-devel` | `libpng` | `libpng-dev` | `libpng` |
| libtiff (IMAGE) | `libtiff-devel` | `tiff` / `libtiff` | `libtiff-dev` | `tiff` |
| Zlib (IMAGE/IO) | `zlib-devel` | `zlib` | `zlib1g-dev` | `zlib` |
| Open MPI (MPI*) | `openmpi-devel` | `openmpi` / `open-mpi` | `libopenmpi-dev` | `msmpi` |
| Berkeley DB (RECORDSTORE) | `libdb-cxx-devel` | `db62` / `berkeley-db` | `libdb++-dev` | `berkeleydb` |
| SQLite 3 (RECORDSTORE) | `sqlite-devel` | `sqlite3` / `sqlite` | `libsqlite3-dev` | `sqlite3` |
| hwloc (SYSTEM) | `hwloc-devel` | `hwloc` | `libhwloc-dev` | `hwloc` |
| ffmpeg (VIDEO) | Build from source | `ffmpeg` | `libavcodec-dev`, `libavformat-dev`, `libswscale-dev` | `ffmpeg` |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
biomers = { git = "https://github.com/your-username/biomers" }
```

## Usage

### WSQ Encoding

Encode raw grayscale image data to WSQ format:

```rust
use biomers::wsq_encode;

let width = 500;
let height = 500;
let raw_pixels: Vec<u8> = vec![128u8; (width * height) as usize];
let bitrate = 0.75; // Compression ratio

let wsq_data = wsq_encode(&raw_pixels, width, height, bitrate)?;
```

### WSQ Decoding

Decode WSQ data back to raw grayscale pixels:

```rust
use biomers::wsq_decode;

let (pixels, width, height) = wsq_decode(&wsq_data)?;
```

## Example

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 500;
    let height = 500;
    let raw_pixels = vec![128u8; (width * height) as usize];

    // Encode to WSQ
    let wsq_bytes = biomers::wsq_encode(&raw_pixels, width, height, 0.75)?;
    println!("Encoded {} bytes to WSQ", wsq_bytes.len());

    // Decode back
    let (decoded_pixels, w, h) = biomers::wsq_decode(&wsq_bytes)?;
    println!("Decoded {}x{} image", w, h);

    Ok(())
}
```

Run the included demo:

```bash
cargo run --example demo
```

## Important Considerations

### Memory Management

NBIS functions use `malloc` internally. This library handles memory safely by copying data into Rust `Vec<u8>` and immediately freeing the C-allocated memory.

### Concurrency

Most NBIS functions are thread-safe if they don't rely on global state. However, some older C libraries use global error buffers. Test thoroughly when using in multi-threaded contexts.

### JPEG 2000 (JP2)

NBIS does not include a JP2 encoder. If you need `.jp2` (ISO 15444) support, you must link libopenjpeg separately.

### Image Format

- All images must be **8-bit grayscale**
- WSQ encoding defaults to **500 PPI** (FBI standard for fingerprints)

### Windows Runtime

On Windows, you need vcpkg DLLs in your PATH or copy them to your executable directory:

```
copy %VCPKG_ROOT%\installed\x64-windows\bin\*.dll target\debug\examples\
copy "C:\Program Files\OpenSSL-Win64\bin\libcrypto-3-x64.dll" target\debug\examples\
```

## License

This wrapper is provided as-is. The underlying [libbiomeval](https://github.com/usnistgov/libbiomeval) and NBIS are in the public domain (see libbiomeval/LICENSE.md).
