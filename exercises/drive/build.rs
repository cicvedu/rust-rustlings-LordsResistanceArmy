use std::env;
fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let timestamp_str = timestamp.to_string();
    env::set_var("TEST_FOO", timestamp_str);
    println!("cargo:rustc-env=TEST_FOO={}",&timestamp_str);
}