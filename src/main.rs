mod args;
mod pkgbuild;
mod srcinfo;
mod utils;

use args::handle_args;
use pkgbuild::{generate_pkgbuild, save_pkgbuild};
use srcinfo::{generate_srcinfo, save_srcinfo};
use utils::{create_tarball, get_sha256, input_string};

/// Information stores the required information about package
struct Information {
    maintainer_name: String,
    maintainer_email: String,
    pkgname: String,
    pkgver: String,
    pkgrel: String,
    pkgdesc: String,
    url: String,
    license: String,
    arch: String,
    depends: String,
    makedepends: String,
    sha256sums: String,
}

fn main() {
    let source = handle_args();
    let tarball = match create_tarball(&source) {
        Ok(output) => output,
        Err(e) => {
            println!("Failed to generate tarball: {}.", e);
            "ERRRROOORRR".to_string()
        }
    };

    let pkginfo = Information {
        maintainer_name: input_string("Enter the name of maintainer: ", "_"),
        maintainer_email: input_string("Enter the email of maintainer: ", "_"),
        pkgname: input_string("Enter the name of package: ", "_"),
        pkgver: input_string("Enter the version of package(default: 1.0.0): ", "1.0.0"),
        pkgrel: input_string("Enter the release number of package(default: 1): ", "1"),
        pkgdesc: input_string("Enter the description about package: ", "_"),
        url: input_string("Enter the url of package: ", "_"),
        license: input_string("Enter the license of package: ", "_"),
        // TODO: Allow user to choose from list of common arch, or enter manually
        arch: input_string("Enter the architecture of package(default: x86_64): ", "x86_64"),
        depends: input_string("Enter the dependencies of package: ", ""),
        makedepends: input_string("Enter the make dependencies of package: ", ""),
        sha256sums: match get_sha256(&tarball) {
            Some(sha256) => sha256,
            None => "SKIP".to_string(), // why warn to convert None to snake_case?
        },
    };

    let pkgbuild_result = generate_pkgbuild(&pkginfo);

    match pkgbuild_result {
        Ok(pkgbuild) => {
            println!("Successfully Generated PKGBUILD.");
            save_pkgbuild(&pkgbuild);
        }
        Err(e) => {
            println!("Failed to generate PKGBUILD: {}.", e);
        }
    }

    let srcinfo_result = generate_srcinfo(&pkginfo);

    match srcinfo_result {
        Ok(srcinfo) => {
            println!("Successfully Generated SRCINFO.");
            save_srcinfo(&srcinfo);
        }
        Err(e) => {
            println!("Failed to generate SRCINFO: {}.", e);
        }
    }
}
