fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:target_feature=+crt-static");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rerun-if-changed=build.rs");
}
