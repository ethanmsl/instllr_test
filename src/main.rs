//! main.rs
//!

use instllr_tst::*;
fn main() {
    println!("-------------");
    println!("Hello, world! :)");

    println!("-------------");
    let out = echo_cmd("Hello world -- from Rust via terminal Command!!! :P :P")
        .expect("Failed to execute command");
    println!("Output: {}", String::from_utf8_lossy(&out.stdout));

    println!("-------------");
    let out = pwd_cmd()
        .expect("Failed to execute command");
    println!("Output: {}", String::from_utf8_lossy(&out.stdout));
}
