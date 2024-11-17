use inquire::Editor;
use std::fs;

use crate::pkgbuild::structure;

/// Prints Hello, World
pub fn pkgbuild() {
    println!("todo: update checksum field");
    println!("known issue: 1st line in every func is not indented correctly");
    println!("Hello, World");

    let mut mypkgbuild = structure::PKGBUILD::default();
    set_values(&mut mypkgbuild);

    let buf = format!("{}", mypkgbuild);
    let pkgbuild_ = Editor::new("Check PKGBUILD?")
        .with_predefined_text(&buf)
        .prompt()
        .unwrap();
    let wrote = fs::write("PKGBUILD", pkgbuild_);
    match wrote {
        Ok(_) => println!("Wrote to PKGBUILD"),
        Err(e) => panic!("Failed to write to file. Got {e}"),
    }
}

/// Sets values in given PKGBUILD
pub fn set_values(mypkgbuild: &mut structure::PKGBUILD) {
    mypkgbuild.set_maintainer_details();
    mypkgbuild.set_pkgname();
    mypkgbuild.set_pkgver();
    mypkgbuild.set_pkgrel();
    mypkgbuild.set_epoch();
    mypkgbuild.set_pkgdesc();
    mypkgbuild.set_arch();
    mypkgbuild.set_url();
    mypkgbuild.set_source();
    mypkgbuild.set_checksums();
    mypkgbuild.set_install();
    mypkgbuild.set_changelog();
    mypkgbuild.set_license();
    mypkgbuild.set_depends();
    mypkgbuild.set_makedepends();
    mypkgbuild.set_checkdepends();
    mypkgbuild.set_optdepends();
    mypkgbuild.set_conflicts();
    mypkgbuild.set_provides();
    mypkgbuild.set_replaces();
    mypkgbuild.set_backup();

    mypkgbuild.set_prepare();
    mypkgbuild.set_build();
    mypkgbuild.set_check();
    mypkgbuild.set_package();
}
