//! srcinfo module handles the generation of srcinfo
use crate::Information;

use std::fs::File;
use std::io::{Read, Write};

/// generate_srcinfo generates and returns the SRCINFO
pub fn generate_srcinfo(pkginfo: &Information) {
    let template = get_template();
    let srcinfo: String;

    match template {
        Ok(output) => {
            println!("Got SRCINFO template");
            srcinfo = output
                .replace("{pkgbase}", &pkginfo.pkgname)
                .replace("{pkgdesc}", &pkginfo.pkgdesc)
                .replace("{pkgver}", &pkginfo.pkgver)
                .replace("{pkgrel}", &pkginfo.pkgrel)
                .replace("{pkgurl}", &pkginfo.url)
                .replace("{arch}", &pkginfo.arch)
                .replace("{license}", &pkginfo.license)
                .replace("{makedepends}", &pkginfo.makedepends)
                .replace("{source}", "SOURCE")
                .replace("{sha256sums}", &pkginfo.sha256sums)
                .replace("{pkgname}", &pkginfo.pkgname);

            save_srcinfo(&srcinfo);
        }
        Err(e) => println!("Failed to generate SRCINFO: {}.", e),
    }
}

/// get_template retrieves and returns the contents of templates/SRCINFO
fn get_template() -> std::io::Result<String> {
    let mut file = File::open("templates/SRCINFO")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// save_srcinfo is a helper function to save .SRCINFO to disk
fn save_srcinfo(srcinfo: &String) {
    // create_new because it creates new file in read-write mode; error if the file exists
    // and making sure that possibly existing SRCINFO does not get overwritten
    let file_result = File::create_new(".SRCINFO");

    match file_result {
        Ok(mut file) => match file.write_all(srcinfo.as_bytes()) {
            Ok(_) => println!("Saved .SRCINFO to disk successfully."),
            Err(e) => println!("Failed to write to .SRCINFO: {}.", e),
        },
        Err(e) => println!("Failed to create new .SRCINFO: {}.", e),
    }
}
