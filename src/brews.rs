//! files inovled in using `brew` commands
//! (where `brew` is the unofficial package manager for macOS))

use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output};

/// lists of arguments to particular brew subcommands
///
/// - `Installs` : `brew install`
/// - `Taps`     : `brew tap`
/// - `Casks`    : `brew install --cask`
pub enum BrewList<'a> {
    Installs(&'a [&'a str]),
    Taps(&'a [&'a str]),
    Casks(&'a [&'a str]),
}
impl<'a> BrewList<'a> {
    /// get the `brew` subcommand to use
    pub fn get_base(&self) -> BrewBase {
        match self {
            BrewList::Installs(_) => BrewBase::Install,
            BrewList::Taps(_) => BrewBase::Tap,
            BrewList::Casks(_) => BrewBase::Cask,
        }
    }
    /// get the arguments to the `brew` subcommand
    pub fn get_args(&self) -> &'a [&'a str] {
        match self {
            BrewList::Installs(args) => args,
            BrewList::Taps(args) => args,
            BrewList::Casks(args) => args,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
/// base commands to use with `brew`
pub enum BrewBase {
    Install,
    Cask,
    Tap,
    Info,
}
/// create `brew <subcommand>` command
pub fn make_brew(base: BrewBase) -> Command {
    let mut b = Command::new("brew");
    match base {
        BrewBase::Install => b.arg("install"),
        BrewBase::Cask => b.args(["install", "--cask"]),
        BrewBase::Tap => b.arg("tap"),
        BrewBase::Info => b.arg("info"),
    };
    b
}
/// takes a brew subcommand enum and string input
pub fn brew_action<S: AsRef<OsStr>>(base: BrewBase, arg: S) -> io::Result<Output> {
    make_brew(base).arg(arg).output()
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
