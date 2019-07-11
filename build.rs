extern crate cc;
extern crate rustc_version;
use rustc_version::{version, version_meta, Channel, Version};

fn main() { cc::Build::new().file("src/cpuid.c").compile("libcpuid.a"); }
