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
/// create `brew <subcommand>` command
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

// tap wez/wezterm;
// install --cask wez/wezterm/wezterm;
// install starship zsh-autosuggestions zsh-syntax-highlighting bat choose-rust git-delta exa fd fzf navi ripgrep sd tealdeer viu zoxide eth-p/software/bat-extras-batman;
// tap homebrew/cask-fonts;
// install --cask font-anonymous-pro font-hack font-iosevka font-iosevka-slab font-major-mono-display font-syne-mono font-victor-mono;
