//! main.rs
//!

use instllr_tst::*;
use cmd_lib::{run_cmd, run_fun};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///////// cmd_lib //////////
    proto_install_script()?;

    ///////// cmd_lib //////////
    let msg = "I love rust";
    run_cmd!(echo $msg)?;
    run_cmd!(echo "This is the message: $msg")?;
    let res1 = run_cmd!(brew install procs);
    println!(
        "result of running 'brew install procs' using `run_cmd!`: {:?}",
        res1
    );

    let res2 = run_fun!(brew install procs);
    println!(
        "result of running 'brew install procs' using `run_fun!`: {:?}",
        res2
    );

    ///////// std::process //////////
    let output = Command::new("brew").arg("install").arg("procs").output()?;
    println!(
        "result of running 'brew install procs' using std::process: {:?}",
        String::from_utf8_lossy(&output.stdout)
    );

    Command::new("echo")
        .arg("hi y'all")
        .status()
        .unwrap();

    ///////// OK //////////
    Ok(())
}
