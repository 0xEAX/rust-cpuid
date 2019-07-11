extern crate cc;
fn main() { cc::Build::new().file("src/cpuid.c").compile("libcpuid.a"); }
