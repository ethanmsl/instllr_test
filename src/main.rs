//! main.rs
//!

// use cmd_lib::{run_cmd, run_fun};
use dirs::home_dir;
// use instllr_tst::*;
use std::{path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Command::new("ls")
    //     .arg("-l")
    //     .arg("-a")
    //     .spawn()
    //     .expect("ls command failed to start");

    // verify presence of zsh
    let zsh = which::which_global("zsh").expect("zsh not found");
    let zsh2 = Command::new("zsh")
        .arg("--version")
        .output()
        .expect("zsh command failed to start");
    let xcode_select = which::which("xcode-select").expect("xcode-select not found");
    let brew = which::which("brew").expect("brew not found");
    println!("zsh: {:?}", zsh);
    println!("zsh: {:?}", zsh2);
    // NOTE: (a) consumes the iterator (hence it can't be reused) and (b) the &(...) applies
    // *after* the mapping

    let zsh_all = which::which_all_global("zsh").expect("zsh not found");
    // uhhh.... must be a better way
    // ... hmm ... I guess I just collect if I want to reuse.
    // No need to clone then.
    // That said: I'm not sure why `.iter()` doesn't consume the vector.
    let zsh_all_vec = zsh_all.collect::<PathBuf>();
    let zsh_all_again = zsh_all_vec.clone();
    zsh_all_again.iter().map(|elem| println!("zsh_all: {:?}", elem));
    zsh_all_vec.iter().map(|elem| println!("zsh_all: {:?}", elem));
    println!("zsh_all: {:?}", zsh_all_vec);



    println!("xcode-select: {:?}", xcode_select);
    println!("brew: {:?}", brew);

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let home_dir = home_dir().expect("Home Directory ('~') not found");
    println!("home_dir: {:?}", home_dir);

    Command::new("exa")
        .current_dir(home_dir)
        .spawn()
        .expect("ls command failed to start");

    let mut xcode = Command::new("xcode-select");
    xcode.arg("--version");
    xcode.spawn().expect("xcode-select command failed to start");
    xcode
        .status()
        .expect("xcode-select command failed to start");
    let out = xcode
        .output()
        .expect("xcode-select command failed to start");
    xcode.spawn().expect("xcode-select command failed to start");
    println!("~~~~~~~~~~~~~~~~~~~");
    println!("output is: {:?}", out);

    println!("~~~~~~~~~~~~~~~~~~~");
    println!("~~~~~~~~~~~~~~~~~~~");
    println!("~~~~~~~~~~~~~~~~~~~");

    if let Ok(output) = xcode.output() {
        println!(
            "output, which is apparently 'Ok()', has stdout: {:?}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    // this should fail (to compile) ... yep :)
    // let fixed = Command::new("exa");
    // fixed.arg("-l");

    ///////// cmd_lib //////////
    // proto_install_script()?;
    // let output = run_cmd!(xcode -select --version);
    // println!("result of running 'xcode-select --version': {:?}", output);
    // println!();
    //
    // let output = Command::new("xcode-select").arg("--version").output()?;
    // println!(
    //     "result of running 'xcode-select --version' using std::process: {:?}",
    //     &output.stderr
    // );

    ///////// cmd_lib //////////
    // let msg = "I love rust";
    // run_cmd!(echo $msg)?;
    // run_cmd!(echo "This is the message: $msg")?;
    // let res1 = run_cmd!(brew install procs);
    // println!(
    //     "result of running 'brew install procs' using `run_cmd!`: {:?}",
    //     res1
    // );
    //
    // let res2 = run_fun!(brew install procs);
    // println!(
    //     "result of running 'brew install procs' using `run_fun!`: {:?}",
    //     res2
    // );
    //
    // ///////// std::process //////////
    // let output = Command::new("brew").arg("install").arg("procs").output()?;
    // println!(
    //     "result of running 'brew install procs' using std::process: {:?}",
    //     String::from_utf8_lossy(&output.stdout)
    // );
    //
    // Command::new("echo").arg("hi y'all").status().unwrap();

    ///////// OK //////////
    Ok(())
}
