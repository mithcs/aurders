use std::fmt;

/// Main structure type of PKGBUILD
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

