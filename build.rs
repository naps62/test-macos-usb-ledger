// **************************************************************************
// Copyright (c) 2015 Roland Ruckerbauer All Rights Reserved.
//
// This file is part of hidapi_rust.
//
// hidapi_rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// hidapi_rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with hidapi_rust.  If not, see <http://www.gnu.org/licenses/>.
// *************************************************************************

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

    // First check the features enabled for the crate.
    // Only one linux backend should be enabled at a time.

    let avail_backends: [(&'static str, Box<dyn Fn()>); 1] = [(
        "LINUX_STATIC_RUSB",
        Box::new(|| {
            let mut config = cc::Build::new();
            config
                .file("etc/hidapi/libusb/hid.c")
                .include("etc/")
                .include("etc/hidapi/hidapi");
            config.compile("libhidapi.a");
        }),
    )];

    let mut backends = avail_backends
        .iter()
        .filter(|f| env::var(format!("CARGO_FEATURE_{}", f.0)).is_ok());

    if backends.clone().count() != 1 {
        panic!("Exactly one linux hidapi backend must be selected.");
    }

    // Build it!
    (backends.next().unwrap().1)();
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
