//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

use instllr_tst::brews;
use instllr_tst::brews::BrewBase;
use instllr_tst::RunnerInfo;
use which::which;
// use std::process::{Command, Output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_info = RunnerInfo::new();
    println!("host_info: {}", host_info);

    // let output: Output = Command::new("ls").arg("-l").output()?;
    // println!("output.STATUS: {:?}", output.status.code());
    // println!("output.STDOUT: {:?}", String::from_utf8_lossy(&output.stdout));
    // println!("output.STDERR: {:?}", String::from_utf8_lossy(&output.stderr));

    brews::make_brew(BrewBase::Install).arg("sk").status()?;
    if let Some(status) = brews::install("sqlite")?.status.code() {
        println!("status: {:?}", status);
    }

    let x = which("sk")?;
    println!("x: {:?}", x);
    // instllr_tst::saved_junk::old_main()?;
    Ok(())
}
