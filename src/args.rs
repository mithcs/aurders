//! args module handles arguments
use clap::{Arg, Command};

/// handle_args handles the arguments
pub fn handle_args() -> String {
    let matches = Command::new("aurders")
        // Will be shown only when custom help template is used (on clap 4.0 or later)
        // .author("Mitesh Soni, smiteshhc@gmail.com")
        .version("1.0.0")
        .about("aurders is a simple aur helper for developers to publish their packages easily on Arch User Repository.")
        .arg(
            Arg::new("source")
                // Do not set short() or long() to define positional argument
                // .short('s')
                // .long("source")
                .required(true)
                .help("Source folder of the packages")
        )
        .get_matches();

    let source = matches
        .get_one::<String>("source")
        .expect("Source folder is not specified. See --help.");

    source.to_string()
}
