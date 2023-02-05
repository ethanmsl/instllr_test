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

    brews::make_brew(BrewBase::Install).arg("sk").status()?;

    match brews::install("sk")?.status.code() {
        Some(status) => println!("status: {:?}", status),
        None => println!("status: None"),
    }

    let x = which("sk")?;
    println!("x: {:?}", x);

    print!("------------------------------------");
    println!("json: {:?}", brews::is_installed("sk")?);

    // instllr_tst::saved_junk::old_main()?;
    Ok(())
}
