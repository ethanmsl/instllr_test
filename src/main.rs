//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

use instllr_tst::brews;
// use instllr_tst::brews::BrewBase;
use instllr_tst::RunnerInfo;
use serde::{Deserialize, Serialize};
use which::which;
// use std::process::{Command, Output};

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

    let sk = is_installed_witch("sk")?;
    let broot = is_installed_witch("broot")?;
    println!("sk: {}", sk);
    println!("broot: {}", broot);

    println!("\n----------------------\n");

    is_installed("sk")?;
    is_installed("broot")?;

    // instllr_tst::saved_junk::old_main()?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    data: String,
}

fn is_installed_witch<S: AsRef<OsStr>>(arg: S) -> Result<bool, io::Error> {
    match which(arg) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

use std::ffi::OsStr;
use std::io;

fn is_installed<S: AsRef<OsStr>>(arg: S) -> Result<bool, io::Error> {
    let stdo_json = brews::is_installed(arg)?;
    let prop_json: serde_json::Value = serde_json::from_slice(&stdo_json)?;

    match prop_json["formulae"][0]["installed"].as_array() {
        Some(arr) => match arr.is_empty() {
            true => {
                println!("not installed");
                Ok(false)
            }
            false => {
                println!("installed");
                Ok(true)
            }
        },
        None => {
            println!("Error");
            panic!("Error");
        }
    }
}
