use inquire::Editor;
use std::fs;

use crate::pkgbuild::structure::PKGBUILD;

/// Prints Hello, World
pub fn pkgbuild() {
    println!("Hello, World");

    let mypkgbuild = PKGBUILD::default();

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
