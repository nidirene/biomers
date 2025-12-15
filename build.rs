use std::env;
use std::path::PathBuf;

fn main() {
    let nist_src = PathBuf::from("libbiomeval");

    // 1. Build the C library using CMake
    // We target the root, which should build the NBIS static libs
    let mut cmake_config = cmake::Config::new(&nist_src);
    // Use ALL_BUILD for Visual Studio generator, "all" for Unix Makefiles
    #[cfg(windows)]
    cmake_config.build_target("ALL_BUILD");
    #[cfg(not(windows))]
    cmake_config.build_target("all");

    // On Windows, use vcpkg toolchain if VCPKG_ROOT is set
    if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
        let toolchain_file = PathBuf::from(&vcpkg_root)
            .join("scripts")
            .join("buildsystems")
            .join("vcpkg.cmake");
        if toolchain_file.exists() {
            cmake_config.define("CMAKE_TOOLCHAIN_FILE", &toolchain_file);
        }
    }

    // Disable optional features we don't need for NBIS
    cmake_config.define("WITH_HWLOC", "OFF");
    cmake_config.define("WITH_MPI", "OFF");
    cmake_config.define("WITH_FFMPEG", "OFF");
    cmake_config.define("WITH_PCSC", "OFF");

    // Use Release profile to avoid CRT debug library mismatch
    cmake_config.profile("Release");

    let dst = cmake_config.build();

    // Add library search paths - libbiomeval puts Debug/Release in subdirs on Windows
    println!("cargo:rustc-link-search=native={}/build/lib/Debug", dst.display());
    println!("cargo:rustc-link-search=native={}/build/lib/Release", dst.display());
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // libbiomeval aggregates NBIS into a single library
    println!("cargo:rustc-link-lib=static=biomeval");

    // Link vcpkg dependencies that libbiomeval requires
    if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
        let vcpkg_lib = PathBuf::from(&vcpkg_root)
            .join("installed")
            .join("x64-windows")
            .join("lib");
        if vcpkg_lib.exists() {
            println!("cargo:rustc-link-search=native={}", vcpkg_lib.display());
        }
    }

    // Link system libraries required by libbiomeval
    #[cfg(target_os = "macos")]
    {
        // macOS library names (via Homebrew or MacPorts)
        println!("cargo:rustc-link-lib=dylib=jpeg");
        println!("cargo:rustc-link-lib=dylib=openjp2");
        println!("cargo:rustc-link-lib=dylib=png");
        println!("cargo:rustc-link-lib=dylib=tiff");
        println!("cargo:rustc-link-lib=dylib=z");
        println!("cargo:rustc-link-lib=dylib=sqlite3");
        println!("cargo:rustc-link-lib=dylib=db_cxx");
        println!("cargo:rustc-link-lib=dylib=lzma"); // Required by tiff
        // macOS uses Security framework instead of OpenSSL
        println!("cargo:rustc-link-lib=framework=Security");
        // C++ standard library
        println!("cargo:rustc-link-lib=dylib=c++");
        // Add MacPorts library path
        println!("cargo:rustc-link-search=native=/opt/local/lib");
        println!("cargo:rustc-link-search=native=/opt/local/lib/db62");
    }

    #[cfg(target_os = "linux")]
    {
        // Linux library names
        println!("cargo:rustc-link-lib=dylib=jpeg");
        println!("cargo:rustc-link-lib=dylib=openjp2");
        println!("cargo:rustc-link-lib=dylib=png");
        println!("cargo:rustc-link-lib=dylib=tiff");
        println!("cargo:rustc-link-lib=dylib=z");
        println!("cargo:rustc-link-lib=dylib=sqlite3");
        println!("cargo:rustc-link-lib=dylib=db_cxx");
        println!("cargo:rustc-link-lib=dylib=lzma");
        println!("cargo:rustc-link-lib=dylib=crypto");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    #[cfg(windows)]
    {
        // Windows library names (vcpkg)
        println!("cargo:rustc-link-lib=dylib=jpeg");
        println!("cargo:rustc-link-lib=dylib=openjp2");
        println!("cargo:rustc-link-lib=dylib=libpng16");
        println!("cargo:rustc-link-lib=dylib=tiff");
        println!("cargo:rustc-link-lib=dylib=zlib");
        println!("cargo:rustc-link-lib=dylib=sqlite3");
        println!("cargo:rustc-link-lib=dylib=libdb48");
        println!("cargo:rustc-link-lib=dylib=lzma");
        // OpenSSL from system install
        println!("cargo:rustc-link-search=native=C:/Program Files/OpenSSL-Win64/lib/VC/x64/MD");
        println!("cargo:rustc-link-lib=dylib=libcrypto");
    }

    // 2. Generate Bindings
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut bindgen_builder = bindgen::Builder::default()
        .header("wrapper.h")
        // Add include paths for the C headers
        .clang_arg(format!("-I{}", nist_src.join("nbis/include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Add platform-specific include paths for dependencies like jpeglib.h
    #[cfg(target_os = "macos")]
    {
        // MacPorts include path
        bindgen_builder = bindgen_builder.clang_arg("-I/opt/local/include");
        // Homebrew include path (Intel and ARM)
        bindgen_builder = bindgen_builder.clang_arg("-I/usr/local/include");
        bindgen_builder = bindgen_builder.clang_arg("-I/opt/homebrew/include");
    }

    // Add vcpkg include path for jpeglib.h and other dependencies (Windows)
    if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
        let vcpkg_include = PathBuf::from(&vcpkg_root)
            .join("installed")
            .join("x64-windows")
            .join("include");
        if vcpkg_include.exists() {
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", vcpkg_include.display()));
        }
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}