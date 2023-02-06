//! an intallation loop

use crate::brews::{brew_action, BrewBase, BrewList};
use crate::check_installation::{is_brew_installed, is_in_path};

/// performs brew installs; looping up to `loop_tolerance` times
pub fn install(inp_list: &BrewList, loop_tolerance: u32) {
    announce_intent(inp_list);
    install_loop(inp_list, loop_tolerance);
}

/// mention items to be actioned (in brew)
fn announce_intent(inp_list: &BrewList) {
    match inp_list {
        BrewList::Taps(taps) => {
            println!("List of taps to be, uh... 'tapped':");
            for tap in *taps {
                print!("{}, ", tap);
            }
        }
        BrewList::Casks(casks) => {
            println!("List of casks to be installed:");
            for cask in *casks {
                print!("{}, ", cask);
            }
        }
        BrewList::Installs(installs) => {
            println!("List of commands to be installed:");
            for install in *installs {
                print!("{}, ", install);
            }
        }
    }
    println!();
}

/// while loop that circles through until things are installed
/// `loop_tolerance`: max number of loops to allow (inclusive)
fn install_loop(inp_list: &BrewList, loop_tolerance: u32) {
    let mut inp_pair = (inp_list.get_base(), inp_list.get_args().to_vec());
    let mut count = 0;

    while !inp_pair.1.is_empty() && count < loop_tolerance {
        println!("Checking installation status...");
        retain_uninstalled(&mut inp_pair);
        if inp_pair.1.is_empty() {
            break;
        }
        attempt_installation(&inp_pair);
        count += 1;
    }

    match inp_pair.1.is_empty() {
        true => println!("\n\nSuccess!"),
        false => println!("\n\nFailed to install all items"),
    }
}

/// check for installation/tappedness and remove from list if found
///
/// # Warning:
/// **mutates** input!
fn retain_uninstalled(inp_pair: &mut (BrewBase, Vec<&str>)) {
    let base = inp_pair.0;
    let vec = &mut inp_pair.1;

    match base {
        BrewBase::Install | BrewBase::Cask => {
            vec.retain(|cmd| {
                if is_in_path(cmd).expect("is_in_path failed")
                    || is_brew_installed(base, cmd).expect("is_brew_tapped failed")
                {
                    print!("{}✓ , ", cmd);
                    false
                } else {
                    print!("{}✗ , ", cmd);
                    true
                }
            });
        }
        BrewBase::Tap => {
            vec.retain(|cmd| {
                if is_brew_installed(base, cmd).expect("is_brew_tapped failed") {
                    print!("{}✓ , ", cmd);
                    false
                } else {
                    print!("{}✗ , ", cmd);
                    true
                }
            });
        }
        BrewBase::Info => panic!("BrewBase::Info is not a valid input for this function"),
    }
}

/// attempt to install/tap the elements of the BrewBase annotated list
fn attempt_installation(inp_pair: &(BrewBase, Vec<&str>)) {
    let base = inp_pair.0;
    let iter = inp_pair.1.iter();

    match base {
        BrewBase::Install => println!("\n\nInstalling brews..."),
        BrewBase::Cask => println!("\n\nInstalling brews..."),
        BrewBase::Tap => println!("\n\nTapping brews..."),
        BrewBase::Info => panic!("BrewBase::Info is not a valid input for this function"),
    }
    iter.for_each(|arg| {
        brew_action(base, arg).expect("brew_action failed");
    });
}
