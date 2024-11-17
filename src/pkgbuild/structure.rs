use std::fmt;

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
    pub arch: Vec<String>,
    pub url: String,
    pub sources: Vec<String>,
    pub checksums: Vec<String>,
    pub install: String,
    pub changelog: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,

    pub prepare: Vec<String>,
    pub build: Vec<String>,
    pub check: Vec<String>,
    pub package: Vec<String>,
}

/// DISPLAY IMPLEMENTATION for PKGBUILD structure
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

    // ////////////////////////////////////////
    // SETTERS
    // ////////////////////////////////////////

    fn set_maintainer_name(&self) -> bool {
        return true;
    }

    fn set_maintainer_email(&self) -> bool {
        return true;
    }

    fn set_pkgname(&self) -> bool {
        return true;
    }

    fn set_pkgver(&self) -> bool {
        return true;
    }

    fn set_pkgrel(&self) -> bool {
        return true;
    }

    fn set_epoch(&self) -> bool {
        return true;
    }

    fn set_pkgdesc(&self) -> bool {
        return true;
    }

    fn set_arch(&self) -> bool {
        return true;
    }

    fn set_url(&self) -> bool {
        return true;
    }

    fn set_source(&self) -> bool {
        return true;
    }

    fn set_checksum(&self) -> bool {
        return true;
    }

    fn set_install(&self) -> bool {
        return true;
    }

    fn set_changelog(&self) -> bool {
        return true;
    }

    fn set_license(&self) -> bool {
        return true;
    }

    fn set_depends(&self) -> bool {
        return true;
    }

    fn set_makedepends(&self) -> bool {
        return true;
    }

    fn set_checkdepends(&self) -> bool {
        return true;
    }

    fn set_optdepends(&self) -> bool {
        return true;
    }

    fn set_conflicts(&self) -> bool {
        return true;
    }

    fn set_provides(&self) -> bool {
        return true;
    }

    fn set_replaces(&self) -> bool {
        return true;
    }

    fn set_backup(&self) -> bool {
        return true;
    }


    fn set_prepare(&self) -> bool {
        return true;
    }

    fn set_build(&self) -> bool {
        return true;
    }

    fn set_check(&self) -> bool {
        return true;
    }

    fn set_package(&self) -> bool {
        return true;
    }

    // ////////////////////////////////////////
    // GETTERS
    // ////////////////////////////////////////

    fn get_maintainer_name(&self) -> String {
        todo!();
    }

    fn get_maintainer_email(&self) -> String {
        todo!();
    }

    fn get_pkgname(&self) -> String {
        todo!();
    }

    fn get_pkgver(&self) -> String {
        todo!();
    }

    fn get_pkgrel(&self) -> String {
        todo!();
    }

    fn get_epoch(&self) -> String {
        todo!();
    }

    fn get_pkgdesc(&self) -> String {
        todo!();
    }

    fn get_arch(&self) -> Vec<String> {
        todo!();
    }

    fn get_url(&self) -> String {
        todo!();
    }

    fn get_source(&self) -> Vec<String> {
        todo!();
    }

    fn get_checksums(&self) -> Vec<String> {
        todo!();
    }

    fn get_install(&self) -> String {
        todo!();
    }

    fn get_changelog(&self) -> String {
        todo!();
    }

    fn get_license(&self) -> Vec<String> {
        todo!();
    }

    fn get_depends(&self) -> Vec<String> {
        todo!();
    }

    fn get_makedepends(&self) -> Vec<String> {
        todo!();
    }

    fn get_checkdepends(&self) -> Vec<String> {
        todo!();
    }

    fn get_optdepends(&self) -> Vec<String> {
        todo!();
    }

    fn get_conflicts(&self) -> Vec<String> {
        todo!();
    }

    fn get_provides(&self) -> Vec<String> {
        todo!();
    }

    fn get_replaces(&self) -> Vec<String> {
        todo!();
    }

    fn get_backup(&self) -> Vec<String> {
        todo!();
    }


    fn get_prepare(&self) -> Vec<String> {
        todo!();
    }

    fn get_build(&self) -> Vec<String> {
        todo!();
    }

    fn get_check(&self) -> Vec<String> {
        todo!();
    }

    fn get_package(&self) -> Vec<String> {
        todo!();
    }
}
