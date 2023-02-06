//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

// use cmd_lib::*;
use instllr_tst::{brews::BrewList, check_installation::is_in_path, install_loop, RunnerInfo};
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get Runner info
    // note: `arch` is not a reliable flag, due to compatibility mode
    let host_info = RunnerInfo::new();
    dbg!(host_info);

    // check `xcode-select`
    // prompt user to manually install if not found
    match is_in_path("xcode-select")? {
        true => println!("xcode-select is installed"),
        false => {
            println!("xcode-select is not installed");
            println!("please install xcode-select brew and then re-run this script");
            println!("To install xcode-select (apple dev tools) run:");
            println!("\n--->       xcode-select --install      <---\n");
            println!("To install brew (apple package manager) run:");
            println!("\n--->       /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"      <---\n");
            println!("(Due to required permissions we need you to install those manually!)");
            exit(1);
        }
    }
    // check `brew`
    // prompt user to manually install if not found
    match is_in_path("brew")? {
        true => println!("brew is installed"),
        false => {
            println!("brew is not installed");
            println!("please install brew and then re-run this script");
            println!("To install brew (apple package manager) run:");
            println!("\n--->       /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"      <---\n");
            println!("(Due to required permissions we need you to install that manually!)");
            exit(2);
        }
    }

    // enage brew install loop
    let to_install = [TAPS, CASKS, INSTALLS];
    to_install.iter().for_each(|list| {
        println!();
        install_loop::install(list);
    });

    // install_loop::install(vec!["".to_string()]);

    Ok(())
}

const TAPS: BrewList = BrewList::Taps(&["wez/wezterm", "homebrew/cask-fonts"]);
const CASKS: BrewList = BrewList::Casks(&[
    "wezterm", // "wez/wezterm/wezterm"
    // ^ both seem to work, shorter is what brew reports as installed
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
    "bat-extras-batman", // "eth-p/software/bat-extras-batman",
                         // ^ both seem to work, shorter is what brew reports as installed
]);
