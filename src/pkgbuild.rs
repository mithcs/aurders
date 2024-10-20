//! pkgbuild module handles the generation of pkgbuild
use crate::utils::dead;
use crate::Information;

use std::fs::{self, File};
use std::io::{self, BufRead, Write};

/// generate_pkgbuild generates and returns the PKGBUILD
pub fn generate_pkgbuild(pkginfo: &Information) {
    let template = get_template();
    let pkgbuild: String;

    let build_commands = get_build_commands();
    let package_commands = get_package_commands();

    match template {
        Ok(output) => {
            println!("\nGot PKGBUILD template.");
            pkgbuild = output
                .replace("{maintainer_name}", &pkginfo.maintainer_name)
                .replace("{maintainer_email}", &pkginfo.maintainer_email)
                .replace("{pkgname}", &pkginfo.pkgname)
                .replace("{pkgver}", &pkginfo.pkgver)
                .replace("{pkgrel}", &pkginfo.pkgrel)
                .replace("{pkgdesc}", &pkginfo.pkgdesc)
                .replace("{arch}", &pkginfo.arch)
                .replace("{url}", &pkginfo.url)
                .replace("{license}", &pkginfo.license)
                .replace("{depends}", &pkginfo.depends)
                .replace("{makedepends}", &pkginfo.makedepends)
                .replace("{source}", &pkginfo.source)
                .replace("{sha256sums}", &pkginfo.sha256sums)
                .replace("{build}", &build_commands)
                .replace("{package}", &package_commands);

            save_pkgbuild(&pkgbuild);
        }
        Err(e) => {
            eprintln!("Failed to generate PKGBUILD from template: {}.", e);
            dead();
        }
    };
}

/// get_template retrieves and returns the contents of templates/PKGBUILD
fn get_template() -> std::io::Result<String> {
    let contents_vec = match fs::read("templates/PKGBUILD") {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to read templates/PKGBUILD: {}.", e);
            dead();
            return Err(e);
        }
    };

    let contents = match String::from_utf8(contents_vec) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert Vec<u8> to String: {}.", e);
            dead();
            "ERRROOORRR".to_string()
        }
    };

    Ok(contents)
}

/// save_pkgbuild is a helper function to save PKGBUILD to disk
fn save_pkgbuild(pkgbuild: &String) {
    // create_new because it creates new file in read-write mode; errror if the file exists
    // and making sure that possibly existing PKGBUILD does not get overwritten
    let file_result = File::create_new("aurders/PKGBUILD");

    match file_result {
        Ok(mut file) => match file.write_all(pkgbuild.as_bytes()) {
            Ok(_) => println!("Saved PKGBUILD to disk successfully."),
            Err(e) => {
                eprintln!("Failed to write to PKGBUILD: {}.", e);
                dead();
            }
        },
        Err(e) => {
            eprintln!("Failed to create new PKGBUILD: {}.", e);
            dead();
        }
    }
}

/// get_build_commads gets the build commands from user and returns it
fn get_build_commands() -> String {
    let mut build = String::new();
    let stdin = io::stdin();

    println!("\nEnter commands to add in build(). [\"qq\" or EOF signal to quit]");

    // lock the stdin and take multiline input correctly
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim() == "qq" {
                    break;
                }
                build.push_str(&input);
                build.push_str("\n");
            }
            Err(e) => {
                eprintln!("Error reading line: {}.", e);
                break;
            }
        }
    }

    build.trim().to_string()
}

/// get_package_commads gets the package commands from user and returns it
fn get_package_commands() -> String {
    let mut package = String::new();
    let stdin = io::stdin();

    println!("\nEnter commands to add in package(). [\"qq\" or EOF signal to quit]");

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim() == "qq" {
                    break;
                }
                package.push_str(&input);
                package.push_str("\n");
            }
            Err(e) => {
                eprintln!("Error reading line: {}.", e);
                break;
            }
        }
    }

    package.trim().to_string()
}
