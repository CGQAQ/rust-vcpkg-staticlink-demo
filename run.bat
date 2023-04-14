@echo off

cargo install cargo-vcpkg
cargo vcpkg build
cargo r