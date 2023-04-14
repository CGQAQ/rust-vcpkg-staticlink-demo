#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::ffi::CStr;

fn main() {
    let version = unsafe {
        CStr::from_ptr(get_openssl_version())
    };

    println!("OpenSSL version: {}", version.to_str().unwrap());
}
