//! main.rs
//!

use instllr_tst::echo_cmd;
fn main() {
    println!("Hello, world! :)");
    let out = echo_cmd("Hello world -- from Rust via terminal Command!!! :P :P")
        .expect("Failed to execute command");
    println!("Output: {}", String::from_utf8_lossy(&out.stdout));
}
