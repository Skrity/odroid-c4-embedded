use std::env::var as env;

fn main() {
    if env("TARGET").unwrap() == "aarch64-unknown-none" {
        println!("cargo:rerun-if-changed=./linker.ld");
        println!("cargo:rustc-link-arg=--script=./linker.ld");
        println!("cargo:rustc-link-arg=--oformat=binary");
    }
}
