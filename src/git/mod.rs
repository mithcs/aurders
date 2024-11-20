use inquire::{required, Confirm, Text};
use std::env::set_current_dir;
use std::fs;
use std::process::Command;

pub fn git(pkgname: String) {
    clone_repo(&pkgname);
    add_to_repo(&pkgname);
    commit_changes();
}

fn clone_repo(pkgname: &String) {
    let url: String = format!("ssh://aur@aur.archlinux.org/{pkgname}.git");

    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .output()
        .expect("Failed to clone repository");

    if !output.status.success() {
        eprintln!("git clone failed with status: {}", output.status);
        panic!("Error message: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!(
        "{}",
        String::from_utf8(output.stdout).expect("Failed to convert `git clone` output to UTF-8")
    );
}

fn add_to_repo(pkgname: &String) {
    match fs::copy("PKGBUILD", format!("{pkgname}/PKGBUILD")) {
        Ok(_) => println!("Copied PKGBUILD"),
        Err(e) => eprintln!("Failed to copy PKGBUILD. Got {e}"),
    };

    match fs::copy(".SRCINFO", format!("{pkgname}/.SRCINFO")) {
        Ok(_) => println!("Copied .SRINFO"),
        Err(e) => eprintln!("Failed to copy .SRCINFO. Got {e}"),
    };

    match set_current_dir(pkgname) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to change current directory. Got {e}"),
    };

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to add files to repository");

    if !output.status.success() {
        eprintln!("git add . failed with status: {}", output.status);
        panic!("Error message: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!(
        "{}",
        String::from_utf8(output.stdout).expect("Failed to convert `git add` output to UTF-8")
    );
}

fn commit_changes() {
    if !Confirm::new("Do you want to commit changes to git?")
        .prompt()
        .unwrap()
    {
        return;
    }

    let commit_msg = Text::new("Enter commit message:")
        .with_help_message("Commit message for committing changes in git repository")
        .with_validator(required!())
        .prompt()
        .unwrap();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_msg)
        .output()
        .expect("Failed to commit changes in repository");

    if !output.status.success() {
        eprintln!("git commit failed with status: {}", output.status);
        panic!("Error message: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!(
        "{}",
        String::from_utf8(output.stdout).expect("Failed to convert `git commit` output to UTF-8")
    );
}
