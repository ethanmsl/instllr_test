//! main.rs
//!

// use instllr_tst::*;
// use std::process::Command;
use cmd_lib::run_cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msg = "I love rust";
    run_cmd!(echo $msg)?;
    run_cmd!(echo "This is the message: $msg")?;
    run_cmd!(brew install procs)?;
    Ok(())

    // println!("-------------");
    // println!("Hello, world! :)");
    //
    // println!("-------------");
    // let out = echo_cmd("Hello world -⦳ℯ- from Rust via terminal Command!!! :P :P")
    //     .expect("Failed to execute command");
    // println!("Output: {}", String::from_utf8_lossy(&out.stdout));
    // println!("Output: {:?}", &out.stdout);
    //
    // println!("-------------");
    // let out = pwd_cmd().expect("Failed to execute command");
    // println!("Output: {}", String::from_utf8_lossy(&out.stdout));
    //
    // // println!("-------------");
    // // let out = up_cmd().expect("Failed to execute command");
    // // println!("Output: {}", String::from_utf8_lossy(&out.stdout));
    //
    // println!("-------------");
    // let out = Command::new("cd");
    // out.arg("src")
    //     .arg("ls")
    //     .output()
    //     .expect("Failed to execute command");
    // println!("Output: {}", String::from_utf8_lossy(&out.stdout));
    //
    // println!("-------------");
    // let out = pwd_cmd().expect("Failed to execute command");
    // println!("Output: {}", String::from_utf8_lossy(&out.stdout));

    // println!("-------------");
    // let out = Command::new("brew").arg("uninstall").arg("procs")
    //     .output()
    //     .expect("Failed to execute command");
    // println!("Output: {}", String::from_utf8_lossy(&out.stdout));
}
