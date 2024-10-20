//! I (mitesh) could not think of any name for this module. And therefore, final_step exists. I'll
//! think of something ASAP and change this, if I remember to do so.
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::{self, Write};
use std::process::Command;

use crate::utils::dead;
use crate::utils::dead_probably;

/// execute_makepkg executes the makepkg command
pub fn execute_makepkg() {
    match env::set_current_dir(format!("aurders")) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to change current directory: {}.", e),
    };

    println!("\nExecuting makepkg...");
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
        }
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
        }
        Err(e) => {
            eprintln!("Failed to clone repository: {}.", e);
            return None;
        }
    };
}

/// add_to_repo adds and commits the files to aur@aur.archlinux.org repository
pub fn add_to_repo(pkgname: &String) {
    match env::set_current_dir(pkgname) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to change current directory: {}.", e);
            return;
        }
    };

    let output = Command::new("git").arg("add").arg(".").output();

    match output {
        Ok(op) => {
            if op.status.success() {
                println!("Added files to git.");
            } else {
                if let Ok(stderr) = String::from_utf8(op.stderr) {
                    eprintln!("git add failed: {}.", stderr);
                    dead();
                } else {
                    eprintln!("Failed to read stderr.");
                }
            }
        }
        Err(e) => {
            eprintln!("git add failed: {}.", e);
            dead();
        }
    };

    let commit_message = get_commit_message();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output();

    match output {
        Ok(op) => {
            if op.status.success() {
                println!("Executed git commit successfully.");
            } else {
                if let Ok(stderr) = String::from_utf8(op.stderr) {
                    eprintln!("git commit failed: {}.", stderr);
                    dead();
                } else {
                    eprintln!("Failed to read stderr.");
                }
            }
        }
        Err(e) => {
            eprintln!("git commit failed: {}.", e);
            dead();
        }
    };
}

/// setup_repo sets up the repository to publish
pub fn setup_repo(pkgname: &String, pkgver: &String, pkgrel: &String) {
    println!("\nSetting up git repository...");

    match clone_aur_repo(&pkgname) {
        Some(_) => (),
        None => return,
    }

    match fs::copy("PKGBUILD", format!("{}/PKGBUILD", &pkgname)) {
        Ok(_) => println!("\nCopied PKGBUILD."),
        Err(e) => eprintln!("Failed to copy PKGBUILD: {}.", e),
    };

    match fs::copy(".SRCINFO", format!("{}/.SRCINFO", &pkgname)) {
        Ok(_) => println!("Copied .SRCINFO."),
        Err(e) => eprintln!("Failed to copy .SRCINFO: {}.", e),
    };

    let arch = match env::consts::ARCH {
        "x86_64" => "x86_64",
        // *Untested*
        "x86" => "i686", // arch dropped support in 2017, unofficial port is available
        "arm" => "arm",  // unofficial port is available
        "aarch64" => "aarch64", // again, unofficial port is available (ARM)
        _ => {
            eprintln!("Architecture is not supported by Arch Linux.");
            eprintln!("You might want to modify the file name of package (.pkg.tar.zst).");
            dead_probably();
            "UNSUPPORTED"
        }
    };

    match fs::copy(
        format!("{}-{}-{}-{}.pkg.tar.zst", &pkgname, &pkgver, &pkgrel, &arch),
        format!(
            "{}/{}-{}-{}-{}.pkg.tar.zst",
            &pkgname, &pkgname, &pkgver, &pkgrel, &arch
        ),
    ) {
        Ok(_) => println!("Copied package."),
        Err(e) => eprintln!("Failed to copy package: {}.", e),
    };
}

/// get_commit_message gets commit message from user and returns it
fn get_commit_message() -> String {
    let stdin = io::stdin();
    let mut message = String::new();

    println!("\nEnter commit message [\"qq\" or EOF signal to quit]");

    // lock the stdin and take multiline input correctly
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim() == "qq" {
                    break;
                }
                message.push_str(&input);
                message.push_str("\n");
            }
            Err(e) => {
                eprintln!("Error reading line: {}.", e);
                break;
            }
        }
    }

    message.trim().to_string()
}
