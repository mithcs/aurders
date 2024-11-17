use inquire::list_option::ListOption;
use inquire::validator::Validation;
use inquire::{min_length, required, Confirm, MultiSelect, Select, Text};

const DELIMETER: &str = ",";

/// Gets maintainer name and return it
pub(in super) fn get_maintainer_name_input() -> String {
    return Text::new("Enter maintainer name")
        .with_help_message("The name of the maintainer")
        .with_validator(required!())
        .prompt()
        .unwrap();
}

/// Gets maintainer email and return it
pub(in super) fn get_maintainer_email_input() -> String {
    return Text::new("Enter maintainer email")
        .with_help_message("The email of the maintainer")
        .with_validator(required!())
        .prompt()
        .unwrap();
}

/// Gets pkgname from user and returns it
pub(in super) fn get_pkgname_input() -> String {
    return Text::new("Enter pkgname")
        .with_help_message("The name of the package")
        .with_validator(required!())
        .prompt()
        .unwrap();
}

/// Gets pkgver from user and returns it
pub(in super) fn get_pkgver_input() -> String {
    return Text::new("Enter pkgver")
        .with_help_message("The version of software as released from the author")
        .with_validator(required!())
        .prompt()
        .unwrap();
}

/// Gets pkgrel from user and returns it
pub(in super) fn get_pkgrel_input() -> String {
    return Text::new("Enter pkgrel")
        .with_default("1")
        .with_help_message("The release number specific to the distribution")
        .prompt()
        .unwrap();
}

/// Gets epoch from user and returns it
pub(in super) fn get_epoch_input() -> String {
    return Text::new("Enter epoch")
        .with_default("0")
        .with_help_message("Used to force package to be seen as newer than previous versino")
        .prompt()
        .unwrap();
}

/// Gets pkgdesc from user and returns it
pub(in super) fn get_pkgdesc_input() -> String {
    return Text::new("Enter pkgdesc")
        .with_validator(required!())
        .with_help_message("Brief description of the package")
        .prompt()
        .unwrap();
}

/// Gets target arch from user and returns it
pub(in super) fn get_arch_input() -> Vec<String> {
    let options = vec!["x86_64", "i686", "any", "Custom"];

    let validator = |a: &[ListOption<&&str>]| {
        if a.len() == 0 {
            return Ok(Validation::Invalid("Select atleast 1 architecture".into()));
        } else {
            return Ok(Validation::Valid);
        }
    };

    let architectures = MultiSelect::new("Select target architecture", options)
        .with_validator(validator)
        .with_vim_mode(true)
        .with_help_message("Select target architecture of package")
        .prompt()
        .unwrap();

    let mut arch: Vec<String> = Vec::new();

    for architecture in architectures {
        match architecture {
            "any" => arch = Vec::from(["any".to_string()]),
            "Custom" => {
                let customs = Text::new("Enter custom architecture").prompt().unwrap();

                let customs_split = customs.split(DELIMETER);

                for custom in customs_split {
                    arch.push(custom.to_string());
                }
            }
            _ => arch.push(architecture.to_string()),
        };
    }

    return arch;
}

/// Gets url from user and returns it
pub(in super) fn get_url_input() -> String {
    return Text::new("Enter url")
        .with_help_message("Enter the url associated with the software")
        .prompt()
        .unwrap();
}

/// Gets sums from user and returns it
pub(in super) fn get_checksums_input() -> Vec<String> {
    let source_count = 1; // TODO
    let sum_types = vec!["MD5", "SHA256", "SHA512", "SHA1", "SHA224", "SHA386", "B2"];

    let sum_type = Select::new("Select type of checksum", sum_types)
        .with_help_message("Select which integrity checks to use")
        .with_vim_mode(true)
        .prompt()
        .unwrap();

    println!("Type -> {sum_type}");

    let mut checksums: Vec<String> = Vec::new();
    for count in 1..=source_count {
        let should_continue =
            Confirm::new(format!("Perform integrity check for source {count}").as_str())
                .with_help_message("Press 'n' to SKIP")
                .prompt()
                .unwrap();

        if should_continue {
            checksums.push("CHECKSUM".to_string())
        } else {
            checksums.push("SKIP".to_string())
        }
    }

    return checksums;
}

/// Gets install from user and returns it
pub(in super) fn get_install_input() -> String {
    return Text::new("Enter install")
        .with_help_message("Special install script that is to be included in the package.")
        .prompt()
        .unwrap();
}

/// Gets source from user and returns it
pub(in super) fn get_sources_input() -> Vec<String> {
    let mut sources: Vec<String> = Vec::new();

    let mut source: String;
    let mut i: u8 = 0;

    loop {
        i += 1;
        source = Text::new(format!("Enter source {i}").as_str())
            .with_validator(if i > 1 { min_length!(0) } else { min_length!(1) })
            .with_help_message("Must reside in same directory as the PKGBUILD, or be a URL that makepkg can use to download the file.\nPress enter again to exit.")
            .prompt()
            .unwrap();

        if source == "" {
            break;
        } else {
            sources.push(source.clone());
        }
    }

    return sources;
}

/// Gets changelog from user and returns it
pub(in super) fn get_changelog_input() -> String {
    return Text::new("Enter changelog")
            .with_help_message("Changelog file that is to be included in the package. Should reside in same dir as PKGBUILD. No need to be included in source")
            .prompt()
            .unwrap();
}

/// Gets license from user and returns it
pub(in super) fn get_license_input() -> Vec<String> {
    let license_options = vec![
        "MIT",
        "GPL-3.0",
        "GPL-2.0",
        "unknown",
        "BSD-3",
        "BSD-2",
        "MPL-2.0",
        "CC0",
        "Apache-2.0",
        "LGPL-3.0",
        "AGPL-3.0",
        "Custom",
    ];

    let mut licenses_selected: Vec<String> = Vec::new();

    let licenses = MultiSelect::new("Select license(s)", license_options)
        .with_help_message("License(s) that apply to the package")
        .with_vim_mode(true)
        .prompt()
        .unwrap();

    // TODO: Implement multiple custom licenses
    for license in licenses {
        match license {
            "Custom" => licenses_selected.push(
                Text::new("Enter custom license(s)")
                    .with_validator(required!())
                    .prompt()
                    .unwrap(),
            ),
            _ => licenses_selected.push(license.to_string()),
        };
    }

    return licenses_selected;
}

// groups here

/// Gets depends from user and returns it
pub(in super) fn get_depends_input() -> Vec<String> {
    let depends = Text::new("Enter dependencies of this package")
        .with_help_message("Packages this package depends on to run. Separated by comma (,)")
        .prompt()
        .unwrap();

    let depends_split = depends.split(DELIMETER);

    let mut dependencies: Vec<String> = Vec::new();
    for dependency in depends_split {
        dependencies.push(dependency.to_string());
    }

    return dependencies;
}

/// Gets make depends from user and returns it
pub(in super) fn get_makedepends_input() -> Vec<String> {
    let makedepends = Text::new("Enter make dependencies of this package")
        .with_help_message("Packages this package depends on to build but are not needed at runtime. Separated by comma (,)")
        .prompt()
        .unwrap();

    let makedepends_split = makedepends.split(DELIMETER);

    let mut make_dependencies: Vec<String> = Vec::new();
    for dependency in makedepends_split {
        make_dependencies.push(dependency.to_string());
    }

    return make_dependencies;
}

/// Gets check depends from user and returns it
pub(in super) fn get_checkdepends_input() -> Vec<String> {
    let checkdepends = Text::new("Enter check dependencies of this package")
        .with_help_message("Packages this package depends on to run its test suite but are not needed at runtime. Separated by comma (,)")
        .prompt()
        .unwrap();

    let checkdepends_split = checkdepends.split(DELIMETER);

    let mut check_dependencies: Vec<String> = Vec::new();
    for dependency in checkdepends_split {
        check_dependencies.push(dependency.to_string());
    }

    return check_dependencies;
}

/// Gets opt depends from user and returns it
pub(in super) fn get_optdepends_input() -> Vec<String> {
    let optdepends = Text::new("Enter opt dependencies of this package")
        .with_help_message("Packages that are not essential for base functionality, but may be necessary to make full use of the contents of this package. Separated by comma (,)")
        .prompt()
        .unwrap();

    let optdepends_split = optdepends.split(DELIMETER);

    let mut opt_dependencies: Vec<String> = Vec::new();
    for dependency in optdepends_split {
        opt_dependencies.push(dependency.to_string());
    }

    return opt_dependencies;
}

/// Gets conflics from user and returns it
pub(in super) fn get_conflicts_input() -> Vec<String> {
    let conflicts = Text::new("Enter conflicts")
        .with_help_message("Packages that will conflict with this package. Separated by comma (,)")
        .prompt()
        .unwrap();

    let conflicts_split = conflicts.split(DELIMETER);

    let mut conflicts_vec: Vec<String> = Vec::new();

    for conflict in conflicts_split {
        conflicts_vec.push(conflict.to_string());
    }

    return conflicts_vec;
}

/// Gets provides from user and returns it
pub(in super) fn get_provides_input() -> Vec<String> {
    let provides = Text::new("Enter provides")
        .with_help_message(
            "Virtual provisions/packages that this package provides. Separated by comma (,)",
        )
        .prompt()
        .unwrap();

    let provides_split = provides.split(DELIMETER);

    let mut provides_vec: Vec<String> = Vec::new();

    for provide in provides_split {
        provides_vec.push(provide.to_string());
    }

    return provides_vec;
}

/// Gets provides from user and returns it
pub(in super) fn get_replaces_input() -> Vec<String> {
    let replaces = Text::new("Enter replaces")
        .with_help_message(
            "Packages that this package should replace. Used to handle renamed/combined packages. Separated by comma (,)",
        )
        .prompt()
        .unwrap();

    let replaces_split = replaces.split(DELIMETER);

    let mut replaces_vec: Vec<String> = Vec::new();

    for replace in replaces_split {
        replaces_vec.push(replace.to_string());
    }

    return replaces_vec;
}

// options here

/// Gets backup from user and returns it
pub(in super) fn get_backup_input() -> Vec<String> {
    let backups = Text::new("Enter backup")
        .with_help_message("Files that should be backed up if the package is removed or upgraded. Separated by comma (,)")
        .prompt()
        .unwrap();

    let backups_split = backups.split(DELIMETER);

    let mut backups_vec: Vec<String> = Vec::new();

    for backup in backups_split {
        backups_vec.push(backup.to_string());
    }

    return backups_vec;
}

/// Gets prepare function from user and returns it
pub(in super) fn get_prepare_input() -> Vec<String> {
    let mut prepare_vec: Vec<String> = Vec::new();
    let mut prepare: String;
    let mut i: u8 = 1;

    loop {
        prepare = Text::new(format!("Enter statement {i} in prepare()").as_str())
            .with_help_message("It is specified in which operations to prepare the sources for building, such as patching, are performed.")
            .prompt()
            .unwrap();

        i += 1;

        if prepare == "" {
            break;
        } else {
            prepare_vec.push(prepare);
        }
    }

    return prepare_vec;
}

/// Gets build function from user and returns it
pub(in super) fn get_build_input() -> Vec<String> {
    let mut build_vec: Vec<String> = Vec::new();
    let mut build: String;
    let mut i: u8 = 1;

    loop {
        build = Text::new(format!("Enter statement {i} in build()").as_str())
            .with_help_message("It is used to compile and/or adjust the source in preparetion to be installed by this function.")
            .prompt()
            .unwrap();

        i += 1;

        if build == "" {
            break;
        } else {
            build_vec.push(build);
        }
    }

    return build_vec;
}

/// Gets check function from user and returns it
pub(in super) fn get_check_input() -> Vec<String> {
    let mut check_vec: Vec<String> = Vec::new();
    let mut check: String;
    let mut i: u8 = 1;

    loop {
        check = Text::new(format!("Enter statement {i} in check()").as_str())
            .with_help_message("It is specified in which a package's test-suite may run. It is ran between the build() and package() function.")
            .prompt()
            .unwrap();

        i += 1;

        if check == "" {
            break;
        } else {
            check_vec.push(check);
        }
    }

    return check_vec;
}

/// Gets package function from user and let
pub(in super) fn get_package_input() -> Vec<String> {
    let mut package_vec: Vec<String> = Vec::new();
    let mut package: String;
    let mut i: u8 = 1;

    loop {
        package = Text::new(format!("Enter statement {i} in package()").as_str())
            .with_help_message("This function is used to install files into the directory that will become the root directory of the built package.")
            .prompt()
            .unwrap();

        i += 1;

        if package == "" {
            break;
        } else {
            package_vec.push(package);
        }
    }

    return package_vec;
}
