//! pkgbuild module handles the generation of pkgbuild
use crate::utils::dead;
use crate::Information;

use std::fs::File;
use std::io::{Read, Write};

/// generate_pkgbuild generates and returns the PKGBUILD
pub fn generate_pkgbuild(pkginfo: &Information) {
    let template = get_template();
    let pkgbuild: String;

    match template {
        Ok(output) => {
            println!("Got PKGBUILD template");
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
                .replace("{sha256sums}", &pkginfo.sha256sums);

            save_pkgbuild(&pkgbuild);
        },
        Err(e) => {
            eprintln!("Failed to generate PKGBUILD: {}.", e);
            dead();
        },
    };

}

/// get_template retrieves and returns the contents of templates/PKGBUILD
fn get_template() -> std::io::Result<String> {
    let mut file = File::open("templates/PKGBUILD")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
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
            },
        },
        Err(e) => {
            eprintln!("Failed to create new PKGBUILD: {}.", e);
            dead();
        },
    }
}
