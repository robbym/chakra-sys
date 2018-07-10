extern crate bindgen;
extern crate cmake;

use cmake::Config;

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    Command::new("python")
        .arg("./ChakraCore/bin/ch/jstoc.py")
        .arg("./ChakraCore/bin/ch/DbgController.js")
        .arg("controllerScript")
        .output()
        .unwrap();

    let path = Config::new("ChakraCore")
        .define("CMAKE_CXX_COMPILER", "/usr/bin/clang++")
        .define("CMAKE_C_COMPILER", "/usr/bin/clang")
        .define("ICU_SETTINGS_RESET", "1")
        // .define("STATIC_LIBRARY_SH", "1")
        .define("CC_USES_SYSTEM_ARCH_SH", "1")
        .define("CMAKE_BUILD_TYPE", "Debug")
        .define("INTL_ICU_SH", "1")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build_target("")
        .build();

    let mut header_path = path.clone();
    header_path.push("build/include/ChakraCore.h");

    let mut lib_path = path.clone();
    lib_path.push("build");
    
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    println!("cargo:rustc-link-search=native={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-flags=-l dylib=ChakraCore");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=icuuc");
    println!("cargo:rustc-flags=-l dylib=icutu");
    println!("cargo:rustc-flags=-l dylib=iculx");
    println!("cargo:rustc-flags=-l dylib=icui18n");
    println!("cargo:rustc-flags=-l dylib=icuio");
    println!("cargo:rustc-flags=-l dylib=icudata");
}
