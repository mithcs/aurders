//! shared module contains the data that is shared among others
use crate::args::handle_args;
use crate::utils::{
    create_directory, create_tarball, get_sha256, get_source, get_templates, input_string, input_string_strict, select_arch
};

/// Information stores the required information about package
pub struct Information {
    pub maintainer_name: String,
    pub maintainer_email: String,
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub pkgdesc: String,
    pub url: String,
    pub license: String,
    pub arch: String,
    pub depends: String,
    pub makedepends: String,
    pub source: String,
    pub sha256sums: String,
}

/// get_information gets the required information about package from user and returns it
// this should go to utils module, right? keeping this here until I am sure about that
// utils module seems already packged. keeping it here, until I don't.
pub fn get_information() -> Option<Information> {
    let (source, get_template) = handle_args();

    create_directory("aurders".to_string());

    // Create tarball first as it is required for sha256sum
    let tarball = match create_tarball(&source) {
        Ok(output) => {
            println!("\nCreated tarball successfully.");
            output
        }
        Err(e) => {
            eprintln!("\nFailed to generate tarball: {}.\n", e);
            "ERRRROOORRR".to_string()
        }
    };

    let pkginfo = Information {
        maintainer_name: input_string_strict("Enter the name of maintainer"),
        maintainer_email: input_string_strict("Enter the email of maintainer"),
        pkgname: input_string_strict("Enter the name of package"),
        pkgver: input_string("Enter the version of package(default: 1.0.0)", "1.0.0"),
        pkgrel: input_string("Enter the release number of package(default: 1)", "1"),
        pkgdesc: input_string("Enter the description about package", ""),
        url: input_string("Enter the url of package", ""),
        license: input_string("Enter the license of package", ""),
        arch: match select_arch() {
            Some(s) => s,
            None => {
                println!("Architecture not selected. Using x86_64 as default.");
                "x86_64".to_string()
            }
        },
        depends: input_string("Enter the dependencies of package: ", ""),
        makedepends: input_string("Enter the make dependencies of package: ", ""),
        source: match get_source() {
            Some(s) => s,
            None => {
                println!("Using default source.\n");
                "$pkgname-$pkgver-$pkgrel.tar.gz".to_string()
            }
        },
        sha256sums: match get_sha256(&tarball) {
            Some(sha256) => sha256,
            None => "SKIP".to_string(),
        },
    };

    if get_template {
        get_templates();
    }

    return Some(pkginfo);
}
