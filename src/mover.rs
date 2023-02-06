//! for moving (renaming) files

use cmd_lib;
use dirs;
use std::time::SystemTime;

/// moves files to a constructed directory and appends them with what is
/// almost certainly a unique timestamp (to prevent overwriting past backups)
///
/// # Note
/// We don't do typical error handling here, and in fact set `pipefail` to `false`
/// as we do not expect all of the files we run `mv` commands against to be present
pub fn backup_zfiles() {
    let home = dirs::home_dir().unwrap();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    cmd_lib::set_pipefail(false);

    println!("Backing up .z* files to `~/z-backup-and-froze/`...");
    let _ = cmd_lib::run_cmd! {
    cd $home;
    mkdir z-backup-and-froze;
    mv .zshrc z-backup-and-froze/.zshrc_$now;
    mv .zshenv z-backup-and-froze/.zshenv_$now;
    mv .zprofile z-backup-and-froze/.zprofile_$now;
    mv .zlogin z-backup-and-froze/.zlogin_$now;
    mv .zlogout z-backup-and-froze/.zlogout_$now;

    };
    println!("Existing .z* files were backed up!");
    println!("the files were appended with a time based tag: _{}", now);
}

// ////////////////////////////////////////////////////////////////////////

/// impressive, but flawed function that CoPilot came up with
/// while `rename` may or may not work (the command isn't present on mac)
/// and it may increase maintenance & complexity by literal-diffing from existing instructions
/// but, most notably, it also errors out if the original fiel isn't there
/// but we ASSUME that almost no one will have all of those files present
/// we could add error handling for it, but ultimately not needed nor does there seem
/// to be a ton to gain
fn _copilot_backup_zfiles() {
    let home = dirs::home_dir().unwrap();
    let zfiles = vec![".zshrc", ".zshenv", ".zprofile", ".zlogin", ".zlogout"];
    let zbackup = home.join("z-backup-and-froze");
    std::fs::create_dir_all(&zbackup).unwrap();
    for file in zfiles {
        let path = home.join(file);
        let new_path = zbackup.join(file);
        std::fs::rename(&path, &new_path).unwrap();
    }
}
