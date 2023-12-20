extern crate cc;
extern crate bindgen;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use glob::glob;

fn main() {
    // Generate Rust Bindings for C Library
    let bindings = bindgen::Builder::default()
        .header("./extern/sofa.h") // Separate header than the source header
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to save bindings");

    // Compile C library
    let src_files = glob("./extern/src/*.c")
        .expect("Failed to read source files")
        .filter(|entry| {
            if let Ok(path) = entry {
                path.file_name() != Some(OsStr::new("t_sofa_c.c"))
            } else {
                true
            }
        });

    let mut cc_build = cc::Build::new();
    for entry in src_files {
        if let Ok(path) = entry {
            cc_build.file(path);
        }
    }

    cc_build.include("./extern/src")
        .compile("sofa");
}