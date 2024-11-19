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
    pub source: String,
    pub checksum_type: String,
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

/// Display implementation for PKGBUILD structure
impl fmt::Display for PKGBUILD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "# Maintainer: {} <{}>\n",
                "pkgname={}\n",
                "pkgver={}\n",
                "pkgrel={}\n",
                "epoch={}\n",
                "pkgdesc='{}'\n",
                "arch={}\n",
                "url='{}'\n",
                "source={}\n",
                "{}={}\n",
                "install='{}'\n",
                "changelog='{}'\n",
                "license={}\n",
                "depends={}\n",
                "makedepends={}\n",
                "checkdepends={}\n",
                "optdepends={}\n",
                "conflicts={}\n",
                "provides={}\n",
                "replaces={}\n",
                "backup={}\n\n",
                "prepare() {{\n{}\n}}\n\n",
                "build() {{\n{}\n}}\n\n",
                "check() {{\n{}\n}}\n\n",
                "package() {{\n{}\n}}"
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
            self.source,
            self.checksum_type,
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
    pub fn set_maintainer_details(&mut self) {
        self.maintainer_name = user_input::get_maintainer_name_input();
        self.maintainer_email = user_input::get_maintainer_email_input();
    }

    /// Setter for pkgname field
    pub fn set_pkgname(&mut self) {
        self.pkgname = user_input::get_pkgname_input();
    }

    /// Setter for pkgver field
    pub fn set_pkgver(&mut self) {
        self.pkgver = user_input::get_pkgver_input();
    }

    /// Setter for pkgrel field
    pub fn set_pkgrel(&mut self) {
        self.pkgrel = user_input::get_pkgrel_input();
    }

    /// Setter for epoch field
    pub fn set_epoch(&mut self) {
        self.epoch = user_input::get_epoch_input();
    }

    /// Setter for pkgdesc field
    pub fn set_pkgdesc(&mut self) {
        self.pkgdesc = user_input::get_pkgdesc_input();
    }

    /// Setter for arch field
    pub fn set_arch(&mut self) {
        let mut architectures: String = String::from("(");

        for architecture in user_input::get_arch_input() {
            architectures = format!("{}'{}' ", architectures, architecture);
        }

        architectures = architectures.trim().to_string();
        architectures.push_str(")");

        self.arch = architectures;
    }

    /// Setter for url field
    pub fn set_url(&mut self) {
        self.url = user_input::get_url_input();
    }

    /// Setter for sources field
    pub fn set_source(&mut self) {
        let mut sources: String = String::from("(");
        let mut temp: String;

        let spacing = " ".repeat(8); // source + = + (

        let mut is_first_line = true;
        for source in user_input::get_sources_input() {
            if is_first_line {
                if source.contains("$") {
                    temp = format!("\"{source}\"\n");
                    sources.push_str(&temp);
                } else {
                    temp = format!("'{source}'\n");
                    sources.push_str(&temp);
                }
                is_first_line = false;
            } else {
                if source.contains("$") {
                    temp = format!("{spacing}\"{source}\"\n");
                    sources.push_str(&temp);
                } else {
                    temp = format!("{spacing}'{source}'\n");
                    sources.push_str(&temp);
                }
            }
        }

        sources = sources.trim().to_string();
        sources.push_str(")");

        self.source = sources;
    }

    /// Setter for checksum type
    pub fn set_checksum_type(&mut self) {
        let c_type = user_input::get_checksum_type();

        let checksum: &str;
        match c_type.as_str() {
            "SHA256" => checksum = "sha256sums",
            "SHA512" => checksum = "sha512sums",
            "SHA224" => checksum = "sha224sums",
            "SHA384" => checksum = "sha384sums",
            t => panic!("Got unexpected checksum type: {t}"),
        };

        self.checksum_type = checksum.to_string();
    }

    /// Setter for checksums field
    pub fn set_checksums(&mut self) {
        let mut checksums: String = String::from("(");
        let mut temp: String;

        let spacing = " ".repeat(self.checksum_type.len() + 2);

        let mut is_first_line = true;
        for checksum in user_input::get_checksums_input() {
            if is_first_line {
                temp = format!("'{checksum}'\n");
                checksums.push_str(&temp);
                is_first_line = false;
            } else {
                temp = format!("{spacing}'{checksum}'\n");
                checksums.push_str(&temp);
            }
        }

        checksums = checksums.trim().to_string();
        checksums.push_str(")");

        self.checksums = checksums;
    }

    /// Setter for install field
    pub fn set_install(&mut self) {
        self.install = user_input::get_install_input();
    }

    /// Setter for changelog field
    pub fn set_changelog(&mut self) {
        self.changelog = user_input::get_changelog_input();
    }

    /// Setter for license field
    pub fn set_license(&mut self) {
        let mut licenses: String = String::from("(");

        for license in user_input::get_license_input() {
            licenses = format!("{}'{}' ", licenses, license);
        }

        licenses = licenses.trim().to_string();
        licenses.push_str(")");

        self.license = licenses;
    }

    /// Setter for depends field
    pub fn set_depends(&mut self) {
        let mut depends: String = String::from("(");

        for depend in user_input::get_depends_input() {
            if depend == "" {
                break;
            }
            depends = format!("{}'{}' ", depends, depend);
        }

        depends = depends.trim().to_string();
        depends.push_str(")");

        self.depends = depends;
    }

    /// Setter for makedepends field
    pub fn set_makedepends(&mut self) {
        let mut makedepends: String = String::from("(");

        for makedepend in user_input::get_makedepends_input() {
            if makedepend == "" {
                break;
            }
            makedepends = format!("{}'{}' ", makedepends, makedepend);
        }

        makedepends = makedepends.trim().to_string();
        makedepends.push_str(")");

        self.makedepends = makedepends;
    }

    /// Setter for checkdepends field
    pub fn set_checkdepends(&mut self) {
        let mut checkdepends: String = String::from("(");

        for checkdepend in user_input::get_checkdepends_input() {
            if checkdepend == "" {
                break;
            }
            checkdepends = format!("{}'{}' ", checkdepends, checkdepend);
        }

        checkdepends = checkdepends.trim().to_string();
        checkdepends.push_str(")");

        self.checkdepends = checkdepends;
    }

    /// Setter for optdepends field
    pub fn set_optdepends(&mut self) {
        let mut optdepends: String = String::from("(");
        let mut temp: String;

        let spacing = " ".repeat(12); // optdepends + = + (

        let mut is_first_line = true;
        for optdepend in user_input::get_optdepends_input() {
            if optdepend == "" {
                break;
            }
            if is_first_line {
                temp = format!("'{optdepend}'\n");
                optdepends.push_str(&temp);
                is_first_line = false;
            } else {
                temp = format!("{spacing}'{optdepend}'\n");
                optdepends.push_str(&temp);
            }
        }

        optdepends = optdepends.trim().to_string();
        optdepends.push_str(")");

        self.optdepends = optdepends;
    }

    /// Setter for conflicts field
    pub fn set_conflicts(&mut self) {
        let mut conflicts: String = String::from("(");

        for conflict in user_input::get_conflicts_input() {
            if conflict == "" {
                break;
            }
            conflicts = format!("{}'{}' ", conflicts, conflict);
        }

        conflicts = conflicts.trim().to_string();
        conflicts.push_str(")");

        self.conflicts = conflicts;
    }

    /// Setter for provides field
    pub fn set_provides(&mut self) {
        let mut provides: String = String::from("(");

        for provide in user_input::get_provides_input() {
            if provide == "" {
                break;
            }
            provides = format!("{}'{}' ", provides, provide);
        }

        provides = provides.trim().to_string();
        provides.push_str(")");

        self.provides = provides;
    }

    /// Setter for replaces field
    pub fn set_replaces(&mut self) {
        let mut replaces: String = String::from("(");

        for replace in user_input::get_replaces_input() {
            if replace == "" {
                break;
            }
            replaces = format!("{}'{}' ", replaces, replace);
        }

        replaces = replaces.trim().to_string();
        replaces.push_str(")");

        self.replaces = replaces;
    }

    /// Setter for backup field
    pub fn set_backup(&mut self) {
        let mut backups: String = String::from("(");
        let mut temp: String;

        let spacing = " ".repeat(8); // backup + = + (

        let mut is_first_line = true;
        for backup in user_input::get_backup_input() {
            if backup == "" {
                break;
            }
            if is_first_line {
                temp = format!("'{backup}'\n");
                backups.push_str(&temp);
                is_first_line = false;
            } else {
                temp = format!("{spacing}'{backup}'\n");
                backups.push_str(&temp);
            }
        }

        backups = backups.trim().to_string();
        backups.push_str(")");

        self.backup = backups;
    }

    /// Setter for prepare field
    pub fn set_prepare(&mut self) {
        let mut prepare: String = String::new();
        let mut temp: String;

        let prepare_input = user_input::get_prepare_input();

        if prepare_input.len() == 0 {
            temp = format!("    echo \"Nothing to prepare\"\n");
            prepare.push_str(&temp);
        }

        for statement in prepare_input {
            temp = format!("    {statement}\n");
            prepare.push_str(&temp);
        }

        self.prepare = prepare.trim_end().to_string();
    }

    /// Setter for build function
    pub fn set_build(&mut self) {
        let mut build: String = String::new();
        let mut temp: String;

        let build_input = user_input::get_build_input();

        if build_input.len() == 0 {
            temp = format!("    echo \"Nothing to build\"\n");
            build.push_str(&temp);
        }

        for statement in build_input {
            temp = format!("    {statement}\n");
            build.push_str(&temp);
        }

        self.build = build.trim_end().to_string();
    }

    /// Setter for check function
    pub fn set_check(&mut self) {
        let mut check: String = String::new();
        let mut temp: String;

        let check_input = user_input::get_check_input();

        if check_input.len() == 0 {
            temp = format!("    echo \"Nothing to check\"\n");
            check.push_str(&temp);
        }

        for statement in check_input {
            temp = format!("    {statement}\n");
            check.push_str(&temp);
        }

        self.check = check.trim_end().to_string();
    }

    /// Setter for package function
    pub fn set_package(&mut self) {
        let mut package: String = String::new();
        let mut temp: String;

        let package_input = user_input::get_package_input();

        if package_input.len() == 0 {
            temp = format!("    echo \"Nothing to package\"\n");
            package.push_str(&temp);
        }

        for statement in package_input {
            temp = format!("    {statement}\n");
            package.push_str(&temp);
        }

        self.package = package.trim_end().to_string();
    }
}
