fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=TEST_FOO=1234567890");

    println!("cargo:rustc-cfg=feature=\"pass\"");
}