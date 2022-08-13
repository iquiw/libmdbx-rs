#[cfg(not(windows))]
fn main() {}
#[cfg(windows)]
fn main() {
    println!("cargo:rustc-link-lib=dylib=ntdll");
}
