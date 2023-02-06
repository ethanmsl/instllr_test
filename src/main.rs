//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

// use cmd_lib::*;
use instllr_tst::{brews::BrewList, check_installation::is_in_path, install_loop, RunnerInfo};

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
