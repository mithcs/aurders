//! shared module contains the data that is shared among others
use crate::args::handle_args;
use crate::utils::{create_tarball, get_sha256, get_templates, input_string, select_arch};

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
    pub sha256sums: String,
}

/// get_information gets the required information about package from user and returns it
pub fn get_information() -> Option<Information> {
    let (source, get_template) = handle_args();

    if get_template {
        get_templates();
    }

    // Create tarball first as it is required for sha256sum
    let tarball = match create_tarball(&source) {
        Ok(output) => {
            println!("\nCreated tarball successfully.\n");
            output
        },
        Err(e) => {
            println!("\nFailed to generate tarball: {}.\n", e);
            "ERRRROOORRR".to_string()
        }
    };

    let pkginfo = Information {
        maintainer_name: input_string("Enter the name of maintainer", "_"),
        maintainer_email: input_string("Enter the email of maintainer", "_"),
        pkgname: input_string("Enter the name of package", "_"),
        pkgver: input_string("Enter the version of package(default: 1.0.0)", "1.0.0"),
        pkgrel: input_string("Enter the release number of package(default: 1)", "1"),
        pkgdesc: input_string("Enter the description about package", "_"),
        url: input_string("Enter the url of package", "_"),
        license: input_string("Enter the license of package", "_"),
        // TODO: Allow user to choose from list of common arch, or enter manually
        arch: match select_arch() {
            Some(s) => s,
            None => {
                println!("Architecture not selected. Using x86_64 as default.");
                "x86_64".to_string()
            },
        },
        depends: input_string("Enter the dependencies of package: ", ""),
        makedepends: input_string("Enter the make dependencies of package: ", ""),
        sha256sums: match get_sha256(&tarball) {
            Some(sha256) => sha256,
            None => "SKIP".to_string(), // why warn to convert None to snake_case?
        },
    };

    return Some(pkginfo);
}
