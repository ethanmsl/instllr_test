//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

// use instllr_tst::brews::BrewBase;
use cmd_lib::*;
use instllr_tst::install_loop::install_loop;
use instllr_tst::{is_in_path, RunnerInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_info = RunnerInfo::new();
    println!("host_info: {}", host_info);

    // brews::make_brew(BrewBase::Install).arg("sk").status()?;
    //
    // match brews::install("sk")?.status.code() {
    //     Some(status) => println!("status: {:?}", status),
    //     None => println!("status: None"),
    // }
    println!();
    let to_echo = vec!["sk", "broot", "bat"];
    // let to_do = r#"echo \"oh my gosh did this work?!?!\""#;
    for lword in to_echo {
        run_cmd! {
            echo "trying to install $lword";
            brew install $lword;
        }?;
    }

    println!("\n----------------------\n");

    let sk = is_in_path("sk")?;
    let broot = is_in_path("broot")?;
    println!("sk: {}", sk);
    println!("broot: {}", broot);

    println!("\n----------------------\n");

    let cmd_list = vec!["sk".to_string(), "broot".to_string()];
    install_loop(cmd_list);

    // instllr_tst::saved_junk::old_main()?;
    Ok(())
}
