//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

// use cmd_lib::*;
// use instllr_tst::check_installation::is_in_path;
// use instllr_tst::install_loop::install_loop;
// use std::io;
// use std::io::Write;
// use std::process::{Command, Stdio};
use instllr_tst::RunnerInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_info = RunnerInfo::new();
    println!("host_info: {}", host_info);

    Ok(())
}
