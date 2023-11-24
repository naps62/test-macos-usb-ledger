extern crate cc;
extern crate pkg_config;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("cargo:warning=target={}", target);
    if target.contains("linux") {
        compile_linux();
    } else if target.contains("darwin") {
        compile_macos();
    } else {
        panic!("Unsupported target os for hidapi-rs");
    }
}

fn compile_linux() {
    println!("cargo:warning=compile-linux");

    cc::Build::new()
        .file("etc/hidapi/libusb/hid.c")
        .include("etc/")
        .include("etc/hidapi/hidapi")
        .compile("libhidapi.a");
}

fn compile_macos() {
    println!("cargo:warning=compile-macos");

    cc::Build::new()
        .file("etc/hidapi/mac/hid.c")
        .include("etc/hidapi/hidapi")
        .compile("libhidapi.a");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=AppKit")
}
