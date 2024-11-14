use inquire::list_option::ListOption;
use inquire::validator::Validation;
use inquire::{min_length, required, Confirm, MultiSelect, Select, Text};

/// This function generates the pkgbuild
pub fn pkgbuild() {
    // ////////////////////
    // Fields
    // ////////////////////

    let (maintainer_name, maintainer_email) = get_maintainer();
    println!("Got {maintainer_name}, {maintainer_email}");

    let pkgname = get_pkgname();
    println!("Got {pkgname}");

    let pkgver = get_pkgver();
    println!("Got {pkgver}");

    let pkgrel = get_pkgrel();
    println!("Got {pkgrel}");

    let epoch = get_epoch();
    println!("Got {epoch}");

    let pkgdesc = get_pkgdesc();
    println!("Got {pkgdesc}");

    let arch = get_arch();
    println!("Got {arch}");

    let url = get_url();
    println!("Got {url}");

    let license = get_license();
    println!("Got {license}");

    let (sources, source_count) = get_source();
    println!("Total {source_count}");
    for source in sources {
        println!("Got {source}");
    }

    let sums = get_sums(source_count);
    for sum in sums {
        println!("Got {sum}");
    }

    let install = get_install();
    println!("Got {install}");

    let changelog = get_changelog();
    println!("Got {changelog}");

    let depends = get_depends();
    println!("Got {depends}");

    let makedepends = get_makedepends();
    println!("Got {makedepends}");

    let checkdepends = get_checkdepends();
    println!("Got {checkdepends}");

    let optdepends = get_optdepends();
    println!("Got {optdepends}");

    let conflicts = get_conflicts();
    println!("Got {conflicts}");

    let provides = get_provides();
    println!("Got {provides}");

    let replaces = get_replaces();
    println!("Got {replaces}");

    let backup = get_backup();
    println!("Got {backup}");

    // ////////////////////
    // Functions
    // ////////////////////

    let prepare = get_prepare();
    println!("Got {prepare}");

    let build = get_build();
    println!("Got {build}");

    let check = get_check();
    println!("Got {check}");

    let package = get_package();
    println!("Got {package}");
}

// ////////////////////
// Fields
// ////////////////////

/// This function gets maintainer name and email
fn get_maintainer() -> (String, String) {
    let name = Text::new("Enter maintainer name")
        .with_help_message("The name of the maintainer")
        .with_validator(required!())
        .prompt()
        .unwrap();

    let email = Text::new("Enter maintainer email")
        .with_help_message("The email of the maintainer")
        .with_validator(required!())
        .prompt()
        .unwrap();

    return (name, email);
}

/// This function gets pkgname from user and returns it
fn get_pkgname() -> String {
    return Text::new("Enter pkgname")
        .with_help_message("The name of the package")
        .with_validator(required!())
        .prompt()
        .unwrap();
}

/// This function gets pkgver from user and returns it
fn get_pkgver() -> String {
    return Text::new("Enter pkgver")
        .with_help_message("The version of software as released from the author")
        .with_validator(required!())
        .prompt()
        .unwrap();
}

/// This function gets pkgrel from user and returns it
fn get_pkgrel() -> String {
    return Text::new("Enter pkgrel")
        .with_default("1")
        .with_help_message("The release number specific to the distribution")
        .prompt()
        .unwrap();
}

/// This function gets epoch from user and returns it
fn get_epoch() -> String {
    return Text::new("Enter epoch")
        .with_help_message("Used to force package to be seen as newer than previous versino")
        .prompt()
        .unwrap();
}

/// This function gets pkgdesc from user and returns it
fn get_pkgdesc() -> String {
    return Text::new("Enter pkgdesc")
        .with_validator(required!())
        .with_help_message("Brief description of the package")
        .prompt()
        .unwrap();
}

/// This function gets target arch from user and returns it
fn get_arch() -> String {
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

    let mut arch = String::new();

    for architecture in architectures {
        match architecture {
            "any" => {
                arch = "'any'".to_string();
                break;
            }
            "Custom" => {
                let customs = Text::new("Enter custom architecture")
                    .with_validator(required!())
                    .prompt()
                    .unwrap();

                let custom_iter = customs.split(" ");

                for custom in custom_iter {
                    arch = arch + "'" + custom + "' ";
                }

                break;
            }
            _ => arch = arch + "'" + architecture + "' ",
        };
    }

    return arch.trim().to_string();
}

/// This function gets url from user and returns it
fn get_url() -> String {
    return Text::new("Enter url")
        .with_help_message("Enter the url associated with the software")
        .prompt()
        .unwrap();
}

/// This function gets sums from user and returns it
fn get_sums(source_count: u16) -> Vec<String> {
    let sum_types = vec!["MD5", "SHA256", "SHA512", "SHA1", "SHA224", "SHA386", "B2"];

    let sum_type = Select::new("Select type of checksum", sum_types)
        .with_help_message("Select which integrity checks to use")
        .with_vim_mode(true)
        .prompt()
        .unwrap();

    println!("Type -> {sum_type}");

    let mut checksums: Vec<String> = vec![];
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

/// This function gets install from user and returns it
fn get_install() -> String {
    return Text::new("Enter install")
        .with_help_message("Special install script that is to be included in the package.")
        .prompt()
        .unwrap();
}

/// This function gets source from user and returns it
fn get_source() -> (Vec<String>, u16) {
    let mut sources: Vec<String> = vec![];

    let mut source: String;
    let mut i: u16 = 0;

    loop {
        i += 1;
        source = Text::new(format!("Enter source {i}").as_str())
            .with_validator(if i > 1 { min_length!(0) } else { min_length!(1) })
            .with_help_message("Must reside in same directory as the PKGBUILD, or be a URL that makepkg can use to download the file.\nPress enter again to exit.")
            .prompt()
            .unwrap();

        if source == "" {
            i -= 1;
            break;
        } else {
            sources.push(source.clone());
        }
    }

    return (sources, i);
}

/// This function gets changelog from user and returns it
fn get_changelog() -> String {
    return Text::new("Enter changelog")
            .with_help_message("Changelog file that is to be included in the package. Should reside in same dir as PKGBUILD. No need to be included in source")
            .prompt().unwrap();
}

/// This function gets license from user and returns it
fn get_license() -> String {
    let licenses = vec![
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

    let license = Select::new("Select license", licenses)
        .with_vim_mode(true)
        .with_help_message("Select license of package")
        .prompt()
        .unwrap();

    match license {
        "Custom" => {
            return Text::new("Enter custom architecture")
                .with_validator(required!())
                .prompt()
                .unwrap();
        }
        _ => return license.to_string(),
    };
}

// should do?; groups

// ////////////////////
// Functions
// ////////////////////

/// This function gets depends from user and returns it
fn get_depends() -> String {
    return Text::new("Enter dependencies of this package")
        .with_help_message("Packages this package depends on to run")
        .prompt()
        .unwrap();
}

/// This function gets make depends from user and returns it
fn get_makedepends() -> String {
    return Text::new("Enter make dependencies of this package")
        .with_help_message(
            "Packages this package depends on to build but are not needed at runtime",
        )
        .prompt()
        .unwrap();
}

/// This function gets check depends from user and returns it
fn get_checkdepends() -> String {
    return Text::new("Enter check dependencies of this package")
        .with_help_message(
            "Packages this package depends on to run its test suite but are not needed at runtime",
        )
        .prompt()
        .unwrap();
}

/// This function gets opt depends from user and returns it
fn get_optdepends() -> String {
    return Text::new("Enter opt dependencies of this package")
            .with_help_message("Packages that are not essential for base functionality, but may be necessary to make full use of the contents of this package")
            .prompt().unwrap();
}

/// This function gets conflics from user and returns it
fn get_conflicts() -> String {
    return Text::new("Enter conflicts")
        .with_help_message("Packages that will conflict with this package")
        .prompt()
        .unwrap();
}

/// This function gets provides from user and returns it
fn get_provides() -> String {
    return Text::new("Enter provides")
        .with_help_message("Virtual provisions/packages that this package provides")
        .prompt()
        .unwrap();
}

/// This function gets provides from user and returns it
fn get_replaces() -> String {
    return Text::new("Enter replaces")
        .with_help_message(
            "Packages that this package should replace. Used to handle renamed/combined packages",
        )
        .prompt()
        .unwrap();
}

// should do?; options

/// This function gets backup from user and returns it
fn get_backup() -> String {
    return Text::new("Enter backup")
        .with_help_message("Files that should be backed up if the package is removed or upgraded.")
        .prompt()
        .unwrap();
}

/// This function gets prepare function from user and returns it
fn get_prepare() -> String {
    return "Hello from get_prepare()".to_string();
}

/// This function gets build function from user and returns it
fn get_build() -> String {
    return "Hello from get_build()".to_string();
}

/// This function gets check function from user and returns it
fn get_check() -> String {
    return "Hello from get_check()".to_string();
}

/// This function gets package function from user and returns it
fn get_package() -> String {
    return "Hello from get_package()".to_string();
}
