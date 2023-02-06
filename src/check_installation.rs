//! Inspect installation status of items

use crate::brews;
use crate::brews::BrewBase;
use serde_json;
use std::ffi::OsStr;
use std::io;
use std::process::Command;
use which::which;

/// Check if a command is is found in system $PATH
///
/// # Warning:
/// While this is a practical stand-in for (usefully) installed, there may be scenarios,
/// e.g. due to installer misconfig or PATH manipulation, where a command is installed,
/// but not on Path.
pub fn is_in_path<S: AsRef<OsStr>>(arg: S) -> Result<bool, io::Error> {
    match which(arg) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Calls list of all brew taps on machine then checks given argument against that list
pub fn is_brew_installed(base: BrewBase, arg_to_match: &str) -> Result<bool, io::Error> {
    let out = match base {
        BrewBase::Install | BrewBase::Cask => Command::new("brew")
            .arg("list")
            .output()
            .expect("failed to execute `brew list` capture"),
        BrewBase::Tap => Command::new("brew")
            .arg("tap")
            .output()
            .expect("failed to execute `brew tap` capture"),
        BrewBase::Info => panic!("BrewBase::Info is not a valid input for this function"),
    };

    let has_arg = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .any(|s| s == arg_to_match);

    Ok(has_arg)
}

// //////////////////// info_json //////////////////// //

#[allow(dead_code)]
/// requests json data about a command
/// (useful to feed into a parser/deserializer like `serde_json`)
fn info_json<S: AsRef<OsStr>>(arg: S) -> Result<Vec<u8>, io::Error> {
    let json = brews::make_brew(BrewBase::Info)
        .arg(arg)
        .arg("--json=v2")
        .output()?
        .stdout;
    Ok(json)
}

#[allow(dead_code)]
/// Inspects whether brew has a record of having installed a package
/// -- shell -- not desgined for use --
/// -- keeping here as a sucessfuly example of using serde_json --
/// -- and back-up should being on PATH be insufficient --
fn is_brew_installed_serde<S: AsRef<OsStr>>(arg: S) -> Result<bool, io::Error> {
    let stdo_json = info_json(arg)?;
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
