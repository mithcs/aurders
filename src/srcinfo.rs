//! srcinfo module handles the generation of srcinfo
use crate::utils::dead;
use crate::Information;

use std::fs::{self, File};
use std::io::Write;

/// generate_srcinfo generates and returns the SRCINFO
pub fn generate_srcinfo(pkginfo: &Information) {
    let template = get_template();
    let srcinfo: String;
    let source = format!(
        "{}-{}-{}.tar.gz",
        &pkginfo.pkgname, &pkginfo.pkgver, &pkginfo.pkgrel
    );

    match template {
        Ok(output) => {
            println!("\nGot SRCINFO template.");
            srcinfo = output
                .replace("{pkgbase}", &pkginfo.pkgname)
                .replace("{pkgdesc}", &pkginfo.pkgdesc)
                .replace("{pkgver}", &pkginfo.pkgver)
                .replace("{pkgrel}", &pkginfo.pkgrel)
                .replace("{pkgurl}", &pkginfo.url)
                .replace("{arch}", &pkginfo.arch)
                .replace("{license}", &pkginfo.license)
                .replace("{makedepends}", &pkginfo.makedepends)
                .replace("{source}", &source)
                .replace("{sha256sums}", &pkginfo.sha256sums)
                .replace("{pkgname}", &pkginfo.pkgname);

            save_srcinfo(&srcinfo);
        }
        Err(e) => {
            eprintln!("Failed to generate SRCINFO: {}.", e);
            dead();
        }
    };
}

/// get_template retrieves and returns the contents of templates/SRCINFO
fn get_template() -> std::io::Result<String> {
    let contents_vec = match fs::read("templates/SRCINFO") {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to read templates/SRCINFO: {}.", e);
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

/// save_srcinfo is a helper function to save .SRCINFO to disk
fn save_srcinfo(srcinfo: &String) {
    // create_new because it creates new file in read-write mode; error if the file exists
    // and making sure that possibly existing SRCINFO does not get overwritten
    let file_result = File::create_new("aurders/.SRCINFO");

    match file_result {
        Ok(mut file) => match file.write_all(srcinfo.as_bytes()) {
            Ok(_) => println!("Saved .SRCINFO to disk successfully."),
            Err(e) => {
                eprintln!("Failed to write to .SRCINFO: {}.", e);
                dead();
            }
        },
        Err(e) => {
            eprintln!("Failed to create new .SRCINFO: {}.", e);
            dead();
        }
    };
}
