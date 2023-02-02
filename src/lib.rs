//! lib.rs

use cmd_lib::*;
use std::io;
use std::process::{Command, Output};

pub fn proto_install_script() -> Result<(), Box<dyn std::error::Error>> {
    // Note: `xcode-select --install` will **error** if already installed
    // NOTE: using {} instead of () with `run_cmd!` causes `rust_fmt` to keep formatting
    let output = run_cmd! {xcode-select --install};
    println!("result of running 'xcode-select --install': {:?}", output);

    let output = run_cmd! {
        brew tap wez/wezterm;
        brew install --cask wez/wezterm/wezterm;
        brew install starship zsh-autosuggestions zsh-syntax-highlighting bat choose-rust git-delta exa fd fzf navi ripgrep sd tealdeer viu zoxide eth-p/software/bat-extras-batman;
        brew tap homebrew/cask-fonts;
        brew install --cask font-anonymous-pro font-hack font-iosevka font-iosevka-slab font-major-mono-display font-syne-mono font-victor-mono;
    }?;
    println!("result of running `baby_install()`: {:?}", output);
    Ok(())
}

/// does not echo to terminal that cargo runs it from, but output is printable
/// after conversion from utf8
pub fn echo_cmd(to_say: &str) -> io::Result<Output> {
    let output = Command::new("echo").arg(to_say).output()?;

    // assert_eq!(to_say.as_bytes(), output.stdout.as_slice());
    Ok(output)
}

/// appears to succeed in getting current working directory
pub fn pwd_cmd() -> io::Result<Output> {
    let output = Command::new("pwd").output()?;
    Ok(output)
}

/// fails
pub fn up_cmd() -> io::Result<Output> {
    let output = Command::new("..").output()?;
    Ok(output)
}

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
