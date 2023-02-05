//! files inovled in using `brew` commands
//! (where `brew` is the unofficial package manager for macOS))

use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output};

/// base commands to use with `brew`
pub enum BrewBase {
    Install,
    Tap,
    Info,
}
/// create `brew install` command
pub fn make_brew(base: BrewBase) -> Command {
    let mut b = Command::new("brew");
    match base {
        BrewBase::Install => b.arg("install"),
        BrewBase::Tap => b.arg("tap"),
        BrewBase::Info => b.arg("info"),
    };
    b
}

pub fn install<S: AsRef<OsStr>>(arg: S) -> io::Result<Output> {
    make_brew(BrewBase::Install).arg(arg).output()
}
pub fn tap<S: AsRef<OsStr>>(arg: S) -> io::Result<Output> {
    make_brew(BrewBase::Tap).arg(arg).output()
}

// //////////////////// info_json //////////////////// //

#[allow(dead_code)]
/// requests json data about a command
/// (useful to feed into a parser/deserializer like `serde_json`)
fn info_json<S: AsRef<OsStr>>(arg: S) -> Result<Vec<u8>, io::Error> {
    let json = make_brew(BrewBase::Info)
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
fn is_brew_installed<S: AsRef<OsStr>>(arg: S) -> Result<bool, io::Error> {
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
// tap wez/wezterm;
// install --cask wez/wezterm/wezterm;
// install starship zsh-autosuggestions zsh-syntax-highlighting bat choose-rust git-delta exa fd fzf navi ripgrep sd tealdeer viu zoxide eth-p/software/bat-extras-batman;
// tap homebrew/cask-fonts;
// install --cask font-anonymous-pro font-hack font-iosevka font-iosevka-slab font-major-mono-display font-syne-mono font-victor-mono;
