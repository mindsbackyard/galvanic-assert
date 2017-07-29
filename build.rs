extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta().unwrap().channel {
        Channel::Beta => {
            println!("cargo:rustc-cfg=structural_matchers");
        },
        Channel::Nightly => {
            println!("cargo:rustc-cfg=structural_matchers");
        },
        _ => {}
    }
}
