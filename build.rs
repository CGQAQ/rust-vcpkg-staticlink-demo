fn main() {
    #[cfg(windows)]
    println!("cargo:rustc-env=RUSTFLAGS=-Ctarget-feature=+crt-static");
    // println!("cargo:env=OPENSSL_STATIC");
    let lib_openssl = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("openssl")
        .unwrap();

    println!("cargo:rerun-if-changed=src/bindings.c");
    cc::Build::new()
        .file("src/bindings.c")
        .include(lib_openssl.include_paths[0].to_str().unwrap())
        .compile("bindings");
}