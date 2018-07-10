extern crate cmake;
extern crate bindgen;

use cmake::Config;

fn main() {
    Config::new("ChakraCore")
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
}