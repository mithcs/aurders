//! I (mitesh) could not think of any name for this module. And therefore, final_step exists. I'll
//! think of something ASAP and change this, if I remember to do so.
use std::process::Command;
use std::fs;
use std::env;

use crate::utils::dead;

/// execute_makepkg executes the makepkg command
pub fn execute_makepkg() {
    match env::set_current_dir(format!("aurders")) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to change current directory: {}.", e)
    };

    let output = Command::new("makepkg").output();

    match output {
        Ok(op) => {
            if op.status.success() {
                println!("Executed makepkg successfullly.");
            } else {
                if let Ok(stderr) = String::from_utf8(op.stderr) {
                    eprintln!("makepkg failed: {}.", stderr);
                    dead();
                } else {
                    eprintln!("Failed to read stderr.");
                }
            }
        },
        Err(e) => {
            eprintln!("makepkg failed: {}.", e);
            dead();
        }
    };
}

/// clone_aur_repo clones the repository of pkgname from aur.archlinux.org
fn clone_aur_repo(pkgname: &String) -> Option<()> {
    // requires aur.archlinux.org to be in known_hosts (probably?)
    let clone_url = format!("ssh://aur@aur.archlinux.org/{}.git", &pkgname);

    let output = Command::new("git").arg("clone").arg(clone_url).output();

    match output {
        Ok(op) => {
            if op.status.success() {
                println!("Cloned repository successfully.");
                return Some(());
            } else {
                if let Ok(stderr) = String::from_utf8(op.stderr) {
                    eprintln!("git clone failed: {}.", stderr);
                } else {
                    eprintln!("Failed to read stderr.");
                }
                return None;
            }
        },
        Err(e) => {
            eprintln!("Failed to clone repository: {}.", e);
            return None;
        }
    };
}

/// setup_repo sets up the repository to publish
pub fn setup_repo(pkgname: &String, pkgver: &String, pkgrel: &String) {
    match clone_aur_repo(&pkgname) {
        Some(_) => (),
        None => return
    }

    match fs::copy("PKGBUILD", format!("{}/PKGBUILD", &pkgname)) {
        Ok(_) => println!("Copied PKGBUILD."),
        Err(e) => eprintln!("Failed to copy PKGBUILD: {}.", e)
    };

    match fs::copy(".SRCINFO", format!("{}/.SRCINFO", &pkgname)) {
        Ok(_) => println!("Copied .SRCINFO."),
        Err(e) => eprintln!("Failed to copy .SRCINFO: {}.", e)
    };

    match fs::copy(format!("{}-{}-{}.tar.gz.zst", &pkgname, &pkgver, &pkgrel), format!("{}/{}-{}-{}.tar.gz.zst", &pkgname, &pkgname, &pkgver, &pkgrel)) {
        Ok(_) => println!("Copied package."),
        Err(e) => eprintln!("Failed to copy package: {}", e)
    };
}