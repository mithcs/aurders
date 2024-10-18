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

    print!("{}", prompt);
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
pub fn get_sha256(source: &String) -> Option<String> {
    let input = Path::new(&source);
    let value_result = try_digest(input);

    match value_result {
        Ok(value) => return Some(value),
        Err(e) => {
            println!(
                "Failed to get sha256: {}.\nUsing 'SKIP' as default value.",
                e
            );
            return None;
        }
    }
}

/// create_tarball creates tarball of given source and returns the name of tarball
pub fn create_tarball(source: &String) -> Result<String, std::io::Error> {
    let tarball_name = match source.split('/').last() {
        Some(output) => format!("{}.tar.gz", output),
        None => {
            // why warn to convert None to snake_case?
            println!("Failed to split string.");
            format!("{}.tar.gz", &source)
        }
    };

    let tar_gz = File::create(&tarball_name)?;

    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    match tar.append_dir(".", &source) {
        Ok(_) => (),
        Err(e) => println!("Failed to append sources to tar: {}.", e),
    };

    Ok(tarball_name)
}
