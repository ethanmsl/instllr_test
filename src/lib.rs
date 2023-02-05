//! lib.rs

// disable clippy lint prevent a lint from running
#![allow(clippy::uninlined_format_args)]
pub mod brews;
pub mod install_loop;
pub mod saved_junk;
// use cmd_lib::*;
// use dirs::home_dir;
use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::io;
use which::which;
// use std::io;
// use std::{
//     path::PathBuf,
//     process::{Command, Output},
// };

// //////////////////// is_installed //////////////////// //

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

// //////////////////// RunnerInfo  //////////////////// //
#[allow(dead_code)]
#[derive(Debug)]
/// Some basic info about the host/runner:  
/// **OS** - Operating System (e.g. linux, windows, macos)  
/// **ARCH** - Architecture (e.g. x86_64, aarch64)  
/// **FAMILY** - Family (e.g. unix, windows)  
///
/// # Warning:
/// - `os` does not include version number
/// - `arch` will be `x86_64` on `aarch64` if run in compatibility/Rosetta mode
pub struct RunnerInfo {
    os: String,
    arch: String,
    family: String,
}

#[allow(clippy::new_without_default)]
impl RunnerInfo {
    pub fn new() -> RunnerInfo {
        RunnerInfo {
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
            family: env::consts::FAMILY.to_string(),
        }
    }
}

impl fmt::Display for RunnerInfo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "\nOS:     - {}\nArch:   - {}\nFamily: - {}",
            self.os, self.arch, self.family
        )
    }
}

// determine what the OS is

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
