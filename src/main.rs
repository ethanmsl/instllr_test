//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

// use instllr_tst::brews::BrewBase;
use cmd_lib::*;
use instllr_tst::check_installation::is_in_path;
use instllr_tst::install_loop::install_loop;
use instllr_tst::RunnerInfo;
// use std::error;
use std::io;
use std::process::{Command, Stdio};
// use std::process::ChildStdin;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_info = RunnerInfo::new();
    println!("host_info: {}", host_info);

    // brews::make_brew(BrewBase::Install).arg("sk").status()?;
    //
    // match brews::install("sk")?.status.code() {
    //     Some(status) => println!("status: {:?}", status),
    //     None => println!("status: None"),
    // }
    set_inp("use".to_string())?;

    // let mut child = Command::new("rev")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to spawn child process");
    //
    // let mut stdin = child.stdin.take().expect("Failed to open stdin");
    // std::thread::spawn(move || {
    //     stdin
    //         .write_all("Hello, world!".as_bytes())
    //         .expect("Failed to write to stdin");
    // });

    //////////////////////////
    println!();

    let to_echo = vec!["sk", "broot", "bat"];
    // let to_do = r#"echo \"oh my gosh did this work?!?!\""#;
    for lword in to_echo {
        run_fun! {
            echo "trying to install $lword";
            brew install $lword;
        }?;
        println!("success_state?: {:?}", is_in_path(lword)?);
    }

    println!("which sk is: {}", run_fun! {which sk}?);
    // println!("xcode res is: {}", run_fun!{xcode-select --install}?);

    println!("\n----------------------\n");

    let sk = is_in_path("sk")?;
    let broot = is_in_path("broot")?;
    println!("sk: {}", sk);
    println!("broot: {}", broot);

    println!("\n----------------------\n");

    let cmd_list = vec!["sk".to_string(), "broot".to_string()];
    install_loop(cmd_list);

    ///////////////////////////////////////////////

    // xcode

    let xcs = is_in_path("xcode-select")?;
    println!("xcs: {}", xcs);

    // brew

    // sub-brews

    Ok(())
}
// match Command::new("/bin/bash")
//     .stdin(Stdio::piped())
//     .arg("-c")
//     .arg(r#"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"#)
//     .spawn()

fn set_inp(inp: String) -> io::Result<()> {
    match Command::new("xargs")
        .stdin(Stdio::piped())
        .arg("rg")
        .spawn()
    {
        Ok(mut child) => {
            child
                .stdin
                .as_ref()
                .unwrap()
                .write_all(inp.as_bytes())
                .unwrap();

            child.wait().unwrap();
            Ok(())
        }
        Err(_e) => Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to execute process",
        )),
    }
}
