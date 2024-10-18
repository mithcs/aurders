//! utils module includes all the utlity and helper functions
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;
use sha256::try_digest;
use tar::Builder;

/// input_string is a helper function to get string input from user efficiently
pub fn input_string(prompt: &str, default: &str) -> String {
    let mut input = String::new();

    print!("{}\n", prompt);
    print!("> ");
    io::stdout().flush().unwrap(); // Flush the output correctly

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => println!("Unable to take input: {}.", e),
    }

    if input.trim().is_empty() {
        return default.to_string();
    }

    // Trim the string to remove '\n'
    input.trim().to_string()
}

/// get_sha256 performs sha256 digest generation and returns it
pub fn get_sha256(tarball: &String) -> Option<String> {
    let input = Path::new(&tarball);
    let value_result = try_digest(input);

    match value_result {
        Ok(value) => return Some(value),
        Err(e) => {
            println!("Failed to get sha256: {}.\nUsing 'SKIP' as default value.", e);
            return None;
        }
    }
}

/// create_tarball creates tarball of given source and returns the name of tarball
pub fn create_tarball(source: &String) -> Result<String, std::io::Error> {
    let tarball_name = match source.split('/').last() {
        Some(output) => format!("aurders/{}.tar.gz", output),
        None => {
            // why warn to convert None to snake_case?
            println!("Failed to split string.");
            format!("aurders/{}.tar.gz", &source)
        }
    };

    let tar_gz = File::create(&tarball_name)?;

    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    match tar.append_dir_all(&source, &source) {
        Ok(_) => (),
        Err(e) => println!("Failed to append source to tar: {}.", e),
    };

    Ok(tarball_name)
}

/// select_arch functions allows user to choose from architectures easily
pub fn select_arch() -> Option<String> {
    print!("Select the target architecture for your package:");
    io::stdout().flush().unwrap(); // Flush the output correctly

    loop {
        print!("\n  [1] x86_64(Default)    [2] i686    [3] any    [4] Enter manually\n> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => println!("Invalid input: {}", e),
        }

        let arch: u8 = match input.trim().parse() {
            Ok(ip) => ip,
            Err(_) => 1,
        };

        match arch {
            1 => return Some("x86_64".to_string()),
            2 => return Some("i686".to_string()),
            3 => return Some("any".to_string()),
            4 => {
                print!("Enter target architecture: ");
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut input).expect("Failed to get input.");

                return Some(input.trim().to_string());
            }
            _ => {
                println!("Invalid input. Try again");
            },
        };
    };
}
