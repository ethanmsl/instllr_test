//! Objects for installation (and their relations)

#[derive(Debug)]
pub struct InstallObject {
    display_name: String,
    which_name: String,
    brew_install_arg: Vec<String>,
    dependencies: Vec<String>,
    known_installed: bool,
}

impl InstallObject {
    pub fn new_basic(display_name: String, dependencies: Vec<String>) -> Self {
        Self {
            display_name,
            which_name: display_name.clone(),
            brew_install_arg: vec![display_name.clone()],
            dependencies,
            known_installed: false,
        }
    }

    pub fn new_detailed(
        display_name: String,
        which_name: String,
        brew_install_arg: Vec<String>,
        dependencies: Vec<String>,
    ) -> Self {
        Self {
            display_name,
            which_name,
            brew_install_arg,
            dependencies,
            known_installed: false,
        }
    }
}
