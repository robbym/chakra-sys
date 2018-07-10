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

    let mut path = Config::new("ChakraCore")
        .define("CMAKE_CXX_COMPILER", "/usr/bin/clang++")
        .define("CMAKE_C_COMPILER", "/usr/bin/clang")
        .define("ICU_SETTINGS_RESET", "1")
        .define("STATIC_LIBRARY_SH", "1")
        .define("CC_USES_SYSTEM_ARCH_SH", "1")
        .define("CMAKE_BUILD_TYPE", "Debug")
        .define("INTL_ICU_SH", "1")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build_target("")
        .build();

    path.push("build/include/ChakraCore.h");
    
    let bindings = bindgen::Builder::default()
        .header(path.to_str().unwrap())
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
