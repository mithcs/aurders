use std::fmt;

use crate::pkgbuild::user_input;

/// Main structure type of PKGBUILD
#[derive(Default)]
pub struct PKGBUILD {
    pub maintainer_name: String,
    pub maintainer_email: String,
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub epoch: String,
    pub pkgdesc: String,
    pub arch: String,
    pub url: String,
    pub sources: String,
    pub checksums: String,
    pub install: String,
    pub changelog: String,
    pub license: String,
    pub depends: String,
    pub makedepends: String,
    pub checkdepends: String,
    pub optdepends: String,
    pub conflicts: String,
    pub provides: String,
    pub replaces: String,
    pub backup: String,

    pub prepare: String,
    pub build: String,
    pub check: String,
    pub package: String,
}

/// DISPLAY implementation for PKGBUILD structure
impl fmt::Display for PKGBUILD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "# Maintainer: {} <{}>\n",
                "pkgname={:?}\n",
                "pkgver={:?}\n",
                "pkgrel={:?}\n",
                "epoch={:?}\n",
                "pkgdesc={:?}\n",
                "arch={:?}\n",
                "url={:?}\n",
                "sources={:?}\n",
                "checksums={:?}\n",
                "install={:?}\n",
                "changelog={:?}\n",
                "license={:?}\n",
                "depends={:?}\n",
                "makedepends={:?}\n",
                "checkdepends={:?}\n",
                "optdepends={:?}\n",
                "conflicts={:?}\n",
                "provides={:?}\n",
                "replaces={:?}\n",
                "backup={:?}\n",
                "prepare() {{\n{:?}\n}}\n",
                "build() {{\n{:?}\n}}\n",
                "check() {{\n{:?}\n}}\n",
                "package() {{\n{:?}\n}}\n"
            ),
            self.maintainer_name,
            self.maintainer_email,
            self.pkgname,
            self.pkgver,
            self.pkgrel,
            self.epoch,
            self.pkgdesc,
            self.arch,
            self.url,
            self.sources,
            self.checksums,
            self.install,
            self.changelog,
            self.license,
            self.depends,
            self.makedepends,
            self.checkdepends,
            self.optdepends,
            self.conflicts,
            self.provides,
            self.replaces,
            self.backup,
            self.prepare,
            self.build,
            self.check,
            self.package
        )?;
        Ok(())
    }
}

#[allow(dead_code)]
impl PKGBUILD {
    /// Setter for maintainer_name, and maintainer_email fields
    fn set_maintainer_details(&mut self) {
        self.maintainer_name = user_input::get_maintainer_name_input();
        self.maintainer_email = user_input::get_maintainer_email_input();
    }

    /// Setter for pkgname field
    fn set_pkgname(&mut self) {
        self.pkgname = user_input::get_pkgname_input();
    }

    /// Setter for pkgver field
    fn set_pkgver(&mut self) {
        self.pkgver = user_input::get_pkgver_input();
    }

    /// Setter for pkgrel field
    fn set_pkgrel(&mut self) {
        self.pkgrel = user_input::get_pkgrel_input();
    }

    /// Setter for epoch field
    fn set_epoch(&mut self) {
        self.epoch = user_input::get_epoch_input();
    }

    /// Setter for pkgdesc field
    fn set_pkgdesc(&mut self) {
        self.pkgdesc = user_input::get_pkgdesc_input();
    }

    /// Setter for arch field
    fn set_arch(&mut self) {
        let mut architectures: String = String::from("(");

        for architecture in user_input::get_arch_input() {
            architectures = format!("{}'{}' ", architectures, architecture);
        }

        architectures = architectures.trim().to_string();
        architectures.push_str(")");

        self.arch = architectures;
    }

    /// Setter for url field
    fn set_url(&mut self) {
        self.url = user_input::get_url_input();
    }

    // TODO: handle whitespaces correctly after \n, for indentation.

    /// Setter for sources field
    fn set_sources(&mut self) {
        let mut sources: String = String::from("(");

        let mut temp: String;
        for source in user_input::get_sources_input() {
            if source.contains("$") {
                temp = format!("\"{source}\"\n");
                sources.push_str(&temp);
            } else {
                temp = format!("'{source}'\n");
                sources.push_str(&temp);
            }
        }

        sources = sources.trim().to_string();
        sources.push_str(")");

        self.sources = sources;
    }

    /// Setter for checksums field
    fn set_checksums(&mut self) {
        let mut checksums: String = String::from("(");

        for checksum in user_input::get_checksums_input() {
            checksums = format!("{}'{}' ", checksums, checksum);
        }

        checksums = checksums.trim().to_string();
        checksums.push_str(")");

        self.checksums = checksums;
    }

    /// Setter for install field
    fn set_install(&mut self) {
        self.install = user_input::get_install_input();
    }

    /// Setter for changelog field
    fn set_changelog(&mut self) {
        self.changelog = user_input::get_changelog_input();
    }

    /// Setter for license field
    fn set_license(&mut self) {
        let mut licenses: String = String::from("(");

        for license in user_input::get_license_input() {
            licenses = format!("{}'{}' ", licenses, license);
        }

        licenses = licenses.trim().to_string();
        licenses.push_str(")");

        self.license = licenses;
    }

    /// Setter for depends field
    fn set_depends(&mut self) {
        let mut depends: String = String::from("(");

        for depend in user_input::get_depends_input() {
            depends = format!("{}'{}' ", depends, depend);
        }

        depends = depends.trim().to_string();
        depends.push_str(")");

        self.depends = depends;
    }

    /// Setter for makedepends field
    fn set_makedepends(&mut self) {
        let mut makedepends: String = String::from("(");

        for makedepend in user_input::get_makedepends_input() {
            makedepends = format!("{}'{}' ", makedepends, makedepend);
        }

        makedepends = makedepends.trim().to_string();
        makedepends.push_str(")");

        self.makedepends = makedepends;
    }

    /// Setter for checkdepends field
    fn set_checkdepends(&mut self) {
        let mut checkdepends: String = String::from("(");

        for checkdepend in user_input::get_checkdepends_input() {
            checkdepends = format!("{}'{}' ", checkdepends, checkdepend);
        }

        checkdepends = checkdepends.trim().to_string();
        checkdepends.push_str(")");

        self.checkdepends = checkdepends;
    }

    /// Setter for optdepends field
    fn set_optdepends(&mut self) {
        let mut optdepends: String = String::from("(");

        let mut temp: String;
        for optdepend in user_input::get_optdepends_input() {
            temp = format!("'{optdepend}'\n");
            optdepends.push_str(&temp);
        }

        optdepends = optdepends.trim().to_string();
        optdepends.push_str(")");

        self.optdepends = optdepends;
    }

    /// Setter for conflicts field
    fn set_conflicts(&mut self) {
        let mut conflicts: String = String::from("(");

        for conflict in user_input::get_conflicts_input() {
            conflicts = format!("{}'{}' ", conflicts, conflict);
        }

        conflicts = conflicts.trim().to_string();
        conflicts.push_str(")");

        self.conflicts = conflicts;
    }

    /// Setter for provides field
    fn set_provides(&mut self) {
        let mut provides: String = String::from("(");

        for provide in user_input::get_provides_input() {
            provides = format!("{}'{}' ", provides, provide);
        }

        provides = provides.trim().to_string();
        provides.push_str(")");

        self.provides = provides;
    }

    /// Setter for replaces field
    fn set_replaces(&mut self) {
        let mut replaces: String = String::from("(");

        for replace in user_input::get_replaces_input() {
            replaces = format!("{}'{}' ", replaces, replace);
        }

        replaces = replaces.trim().to_string();
        replaces.push_str(")");

        self.replaces = replaces;
    }

    /// Setter for backup field
    fn set_backup(&mut self) {
        let mut backups: String = String::from("(");

        let mut temp: String;
        for backup in user_input::get_backup_input() {
            temp = format!("'{backup}'\n");
            backups.push_str(&temp);
        }

        backups = backups.trim().to_string();
        backups.push_str(")");

        self.backup = backups;
    }

    /// Setter for prepare field
    fn set_prepare(&mut self) {
        let mut prepare: String = String::new();

        let mut temp: String;
        for statement in user_input::get_prepare_input() {
            temp = format!("    {statement}\n");
            prepare.push_str(&temp);
        }

        self.prepare = prepare.trim().to_string();
    }

    /// Setter for build field
    fn set_build(&mut self) {
        let mut build: String = String::new();

        let mut temp: String;
        for statement in user_input::get_build_input() {
            temp = format!("    {statement}\n");
            build.push_str(&temp);
        }

        self.build = build.trim().to_string();
    }

    /// Setter for check field
    fn set_check(&mut self) {
        let mut check: String = String::new();

        let mut temp: String;
        for statement in user_input::get_check_input() {
            temp = format!("    {statement}\n");
            check.push_str(&temp);
        }

        self.check = check.trim().to_string();
    }

    /// Setter for package field
    fn set_package(&mut self) {
        let mut package: String = String::new();

        let mut temp: String;
        for statement in user_input::get_package_input() {
            temp = format!("    {statement}\n");
            package.push_str(&temp);
        }

        self.package = package.trim().to_string();
    }
}
