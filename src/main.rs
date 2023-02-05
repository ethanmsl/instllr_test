//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

// use instllr_tst::brews::BrewBase;
use instllr_tst::{is_in_path, RunnerInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_info = RunnerInfo::new();
    println!("host_info: {}", host_info);

    // brews::make_brew(BrewBase::Install).arg("sk").status()?;
    //
    // match brews::install("sk")?.status.code() {
    //     Some(status) => println!("status: {:?}", status),
    //     None => println!("status: None"),
    // }

    println!("\n----------------------\n");

    let sk = is_in_path("sk")?;
    let broot = is_in_path("broot")?;
    println!("sk: {}", sk);
    println!("broot: {}", broot);

    println!("\n----------------------\n");

    // instllr_tst::saved_junk::old_main()?;
    Ok(())
}
