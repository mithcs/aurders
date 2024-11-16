use inquire::Editor;
use std::fs;

use crate::pkgbuild::structure::PKGBUILD;
use crate::pkgbuild::user_input;

/// Prints Hello, World
pub fn pkgbuild() {
    println!("Hello, World");

    let mypkgbuild = PKGBUILD {
        maintainer_name: user_input::get_maintainer_name_input(),
        maintainer_email: user_input::get_maintainer_email_input(),
        pkgname: user_input::get_pkgname_input(),
        pkgver: user_input::get_pkgver_input(),
        pkgrel: user_input::get_pkgrel_input(),
        epoch: user_input::get_epoch_input(),
        pkgdesc: user_input::get_pkgdesc_input(),
        arch: user_input::get_arch_input(),
        url: user_input::get_url_input(),
        sources: user_input::get_sources_input(),
        checksums: user_input::get_checksums_input(),
        install: user_input::get_install_input(),
        changelog: user_input::get_changelog_input(),
        license: user_input::get_license_input(),
        depends: user_input::get_depends_input(),
        makedepends: user_input::get_makedepends_input(),
        checkdepends: user_input::get_checkdepends_input(),
        optdepends: user_input::get_optdepends_input(),
        conflicts: user_input::get_conflicts_input(),
        provides: user_input::get_provides_input(),
        replaces: user_input::get_replaces_input(),
        backup: user_input::get_backup_input(),

        prepare: user_input::get_prepare_input(),
        build: user_input::get_build_input(),
        check: user_input::get_check_input(),
        package: user_input::get_package_input(),
    };

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
