//! an intallation loop

use crate::brews;
use crate::check_installation::is_in_path;

pub fn install_loop(inst_list: Vec<String>) {
    println!("List of commands to be installed:");
    for cmd in &inst_list {
        print!("{}, ", cmd);
    }
    println!("\n");
    let mut inst_list = inst_list;

    while !inst_list.is_empty() {
        println!("Checking installation status...");
        inst_list.retain(|cmd| {
            if is_in_path(cmd).expect("is_in_path failed") {
                print!("{}✓ , ", cmd);
                false
            } else {
                print!("{}✗ , ", cmd);
                true
            }
        });

        // installations
        for cmd in &inst_list {
            println!("Installing {:?} ...", cmd);
            brews::install(cmd).expect("brews::install failed");
        }
    }

    println!("\n\nSuccess!");
}
