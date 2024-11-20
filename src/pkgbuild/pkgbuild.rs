use inquire::Editor;
use std::fs;

use crate::pkgbuild::structure;

/// Generates PKGBUILD
pub fn pkgbuild() -> String {
    let mut mypkgbuild = structure::PKGBUILD::default();
    mypkgbuild.set_values();

    let buffer = format!("{}", mypkgbuild);
    let pkgbuild_ = Editor::new("Check/Update PKGBUILD?")
        .with_predefined_text(&buffer)
        .prompt()
        .unwrap();

    match fs::write("PKGBUILD", pkgbuild_) {
        Ok(_) => println!("Wrote to PKGBUILD"),
        Err(e) => panic!("Failed to write to file. Got {e}"),
    }

    return mypkgbuild.pkgname;
}

