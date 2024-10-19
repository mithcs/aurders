//! args module handles arguments
use clap::{value_parser, Arg, ArgAction, Command};

/// handle_args handles the arguments
pub fn handle_args() -> (String, bool) {
    let matches = Command::new("aurders")
        // Will be shown only when custom help template is used (on clap 4.0 or later)
        // .author("Mitesh Soni, smiteshhc@gmail.com")
        .version("1.0.0")
        .about("aurders is a simple aur helper for developers to publish their packages easily on Arch User Repository.")
        .arg(
            Arg::new("source")
                // Do not set short() or long() as we want to define positional argument
                // .short('s')
                // .long("source")
                .required(true)
                .help("Source folder of the packages")
        )
        // ref: https://docs.rs/clap/latest/src/busybox/multicall-busybox.rs.html#21
        .arg(
            Arg::new("templates")
                .short('t')
                .long("templates")
                .help("Get templates")
                .action(ArgAction::SetTrue)
                .default_missing_value("true")
                .value_parser(value_parser!(bool))
        )
        .get_matches();

    let source = matches
            .get_one::<String>("source")
            .expect("Source folder is not specified. See --help.");

    let get_template = matches.get_one("templates").expect("Failed to get flag templates");

    (source.to_string(), *get_template)
}
