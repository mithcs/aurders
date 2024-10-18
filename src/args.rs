//! args module handles arguments
use clap::{Arg, Command};

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
        .arg(
            Arg::new("get-template")
                .short('t')
                .long("get-template")
                .required(false)
                .num_args(0)  // Takes 0 arguments
                .help("Get templates")
        )
        .get_matches();

    let source = matches
            .get_one::<String>("source")
            .expect("Source folder is not specified. See --help.");

    let get_template = matches
            .get_one::<bool>("get-template");
                

    (source.to_string(), get_template.is_some())
}
