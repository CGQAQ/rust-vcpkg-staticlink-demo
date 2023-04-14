use std::{path::PathBuf, env};

fn main() {
    #[cfg(windows)]
    println!("cargo:rustc-env=RUSTFLAGS=-Ctarget-feature=+crt-static");
    // println!("cargo:env=OPENSSL_STATIC");
    let lib_openssl = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("openssl")
        .unwrap();

    let bindings = bindgen::Builder::default()
        .header("src/bindings.h")
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)); // Tell cargo to invalidate the built crate whenever any of the included header files changed.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=src/bindings.c");
    cc::Build::new()
        .file("src/bindings.c")
        .include(lib_openssl.include_paths[0].to_str().unwrap())
        .compile("bindings");
}