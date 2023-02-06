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
    // get Runner info
    // note: `arch` is not a reliable flag, due to compatibility mode
    let host_info = RunnerInfo::new();
    dbg!(host_info);

    // check `xcode-select`
    // prompt user to manually install if not found
    match is_in_path("xcode-select")? {
        true => println!("xcode-select is installed"),
        false => println!("xcode-select is not installed"),
    }

    // check `brew`
    // prompt user to manually install if not found
    match is_in_path("brew")? {
        true => println!("brew is installed"),
        false => println!("brew is not installed"),
    }

    // enage brew install loop

    // install_loop::install(vec!["".to_string()]);

    Ok(())
}

const TAPS: BrewList = BrewList::Taps(&["wez/wezterm", "homebrew/cask-fonts"]);
const CASKS: BrewList = BrewList::Casks(&[
    "wes/wezterm/wezterm",
    "font-anonymous-pro",
    "font-hack",
    "font-iosevka",
    "font-iosevka-slab",
    "font-major-mono-display",
    "font-syne-mono",
    "font-victor-mono",
]);
const INSTALLS: BrewList = BrewList::Installs(&[
    "starship",
    "zsh-autosuggestions",
    "zsh-syntax-highlighting",
    "bat",
    "choose-rust",
    "git-delta",
    "exa",
    "fd",
    "fzf",
    "navi",
    "ripgrep",
    "sd",
    "tealdeer",
    "viu",
    "zoxide",
    "eth-p/software/bat-extras-batman",
]);
