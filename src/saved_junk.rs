// #![allow(dead_code)]
// use crate::check_installation::is_in_path;
// use crate::install_loop::install_loop;
// use cmd_lib::*;
// use dirs::home_dir;
// use std::io;
// use std::io::Write;
// use std::process::{Command, Stdio};
// use std::{path::PathBuf, process::Output};
//
// fn set_inp(inp: String) -> io::Result<()> {
//     match Command::new("xargs")
//         .stdin(Stdio::piped())
//         .arg("rg")
//         .spawn()
//     {
//         Ok(mut child) => {
//             child
//                 .stdin
//                 .as_ref()
//                 .unwrap()
//                 .write_all(inp.as_bytes())
//                 .unwrap();
//
//             child.wait().unwrap();
//             Ok(())
//         }
//         Err(_e) => Err(io::Error::new(
//             io::ErrorKind::Other,
//             "failed to execute process",
//         )),
//     }
// }
// // use cmd_lib::{run_cmd, run_fun};
// // use instllr_tst::*;
// fn les_old_main() -> Result<(), Box<dyn std::error::Error>> {
//     // brews::make_brew(BrewBase::Install).arg("sk").status()?;
//     //
//     // match brews::install("sk")?.status.code() {
//     //     Some(status) => println!("status: {:?}", status),
//     //     None => println!("status: None"),
//     // }
//     set_inp("use".to_string())?;
//
//     // let mut child = Command::new("rev")
//     //     .stdin(Stdio::piped())
//     //     .stdout(Stdio::piped())
//     //     .spawn()
//     //     .expect("Failed to spawn child process");
//     //
//     // let mut stdin = child.stdin.take().expect("Failed to open stdin");
//     // std::thread::spawn(move || {
//     //     stdin
//     //         .write_all("Hello, world!".as_bytes())
//     //         .expect("Failed to write to stdin");
//     // });
//
//     //////////////////////////
//     println!();
//
//     let to_echo = vec!["sk", "broot", "bat"];
//     // let to_do = r#"echo \"oh my gosh did this work?!?!\""#;
//     for lword in to_echo {
//         run_fun! {
//             echo "trying to install $lword";
//             brew install $lword;
//         }?;
//         println!("success_state?: {:?}", is_in_path(lword)?);
//     }
//
//     println!("which sk is: {}", run_fun! {which sk}?);
//     // println!("xcode res is: {}", run_fun!{xcode-select --install}?);
//
//     println!("\n----------------------\n");
//
//     let sk = is_in_path("sk")?;
//     let broot = is_in_path("broot")?;
//     println!("sk: {}", sk);
//     println!("broot: {}", broot);
//
//     println!("\n----------------------\n");
//
//     let cmd_list = vec!["sk".to_string(), "broot".to_string()];
//     install_loop(cmd_list);
//
//     ///////////////////////////////////////////////
//
//     // xcode
//
//     let xcs = is_in_path("xcode-select")?;
//     println!("xcs: {}", xcs);
//
//     // brew
//
//     // sub-brews
//     Ok(())
// }
//
// pub fn proto_install_script() -> Result<(), Box<dyn std::error::Error>> {
//     // Note: `xcode-select --install` will **error** if already installed
//     // NOTE: using {} instead of () with `run_cmd!` causes `rust_fmt` to keep formatting
//     let output = run_cmd! {xcode-select --install};
//     println!("result of running 'xcode-select --install': {:?}", output);
//
//     run_cmd! {
//         brew tap wez/wezterm;
//         brew install --cask wez/wezterm/wezterm;
//         brew install starship zsh-autosuggestions zsh-syntax-highlighting bat choose-rust git-delta exa fd fzf navi ripgrep sd tealdeer viu zoxide eth-p/software/bat-extras-batman;
//         brew tap homebrew/cask-fonts;
//         brew install --cask font-anonymous-pro font-hack font-iosevka font-iosevka-slab font-major-mono-display font-syne-mono font-victor-mono;
//     }?;
//     Ok(())
// }
//
// /// does not echo to terminal that cargo runs it from, but output is printable
// /// after conversion from utf8
// pub fn echo_cmd(to_say: &str) -> io::Result<Output> {
//     let output = Command::new("echo").arg(to_say).output()?;
//
//     // assert_eq!(to_say.as_bytes(), output.stdout.as_slice());
//     Ok(output)
// }
//
// /// appears to succeed in getting current working directory
// pub fn pwd_cmd() -> io::Result<Output> {
//     let output = Command::new("pwd").output()?;
//     Ok(output)
// }
//
// /// fails
// pub fn up_cmd() -> io::Result<Output> {
//     let output = Command::new("..").output()?;
//     Ok(output)
// }
//
// pub fn old_main() -> Result<(), Box<dyn std::error::Error>> {
//     // Command::new("ls")
//     //     .arg("-l")
//     //     .arg("-a")
//     //     .spawn()
//     //     .expect("ls command failed to start");
//
//     // verify presence of zsh
//     let zsh = which::which_global("zsh").expect("zsh not found");
//     let zsh2 = Command::new("zsh")
//         .arg("--version")
//         .output()
//         .expect("zsh command failed to start");
//     let xcode_select = which::which("xcode-select").expect("xcode-select not found");
//     let brew = which::which("brew").expect("brew not found");
//     println!("zsh: {:?}", zsh);
//     println!("zsh: {:?}", zsh2);
//     // NOTE: (a) consumes the iterator (hence it can't be reused) and (b) the &(...) applies
//     // *after* the mapping
//
//     let zsh_all = which::which_all_global("zsh")
//         .expect("zsh not found")
//         .collect::<PathBuf>();
//     for elem in &zsh_all {
//         println!("zsh_all 111: {:?}", elem);
//     }
//     for elem in &zsh_all {
//         println!("zsh_all 222: {:?}", elem);
//     }
//     zsh_all
//         .iter()
//         .for_each(|elem| println!("zsh_all 333: {:?}", elem));
//     println!("zsh_all 444: {:?}", zsh_all);
//
//     println!("xcode-select: {:?}", xcode_select);
//     println!("brew: {:?}", brew);
//
//     println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
//
//     let home_dir = home_dir().expect("Home Directory ('~') not found");
//     println!("home_dir: {:?}", home_dir);
//
//     Command::new("exa")
//         .current_dir(home_dir)
//         .spawn()
//         .expect("ls command failed to start");
//
//     let mut xcode = Command::new("xcode-select");
//     xcode.arg("--version");
//     xcode.spawn().expect("xcode-select command failed to start");
//     xcode
//         .status()
//         .expect("xcode-select command failed to start");
//     let out = xcode
//         .output()
//         .expect("xcode-select command failed to start");
//     xcode.spawn().expect("xcode-select command failed to start");
//     println!("~~~~~~~~~~~~~~~~~~~");
//     println!("output is: {:?}", out);
//
//     println!("~~~~~~~~~~~~~~~~~~~");
//     println!("~~~~~~~~~~~~~~~~~~~");
//     println!("~~~~~~~~~~~~~~~~~~~");
//
//     if let Ok(output) = xcode.output() {
//         println!(
//             "output, which is apparently 'Ok()', has stdout: {:?}",
//             String::from_utf8_lossy(&output.stdout)
//         );
//     }
//
//     // this should fail (to compile) ... yep :)
//     // let fixed = Command::new("exa");
//     // fixed.arg("-l");
//
//     ///////// cmd_lib //////////
//     // proto_install_script()?;
//     // let output = run_cmd!(xcode -select --version);
//     // println!("result of running 'xcode-select --version': {:?}", output);
//     // println!();
//     //
//     // let output = Command::new("xcode-select").arg("--version").output()?;
//     // println!(
//     //     "result of running 'xcode-select --version' using std::process: {:?}",
//     //     &output.stderr
//     // );
//
//     ///////// cmd_lib //////////
//     // let msg = "I love rust";
//     // run_cmd!(echo $msg)?;
//     // run_cmd!(echo "This is the message: $msg")?;
//     // let res1 = run_cmd!(brew install procs);
//     // println!(
//     //     "result of running 'brew install procs' using `run_cmd!`: {:?}",
//     //     res1
//     // );
//     //
//     // let res2 = run_fun!(brew install procs);
//     // println!(
//     //     "result of running 'brew install procs' using `run_fun!`: {:?}",
//     //     res2
//     // );
//     //
//     // ///////// std::process //////////
//     // let output = Command::new("brew").arg("install").arg("procs").output()?;
//     // println!(
//     //     "result of running 'brew install procs' using std::process: {:?}",
//     //     String::from_utf8_lossy(&output.stdout)
//     // );
//     //
//     // Command::new("echo").arg("hi y'all").status().unwrap();
//
//     ///////// OK //////////
//     Ok(())
// }
