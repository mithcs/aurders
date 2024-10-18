//! utils module includes all the utlity and helper functions
use std::fs::{self, File, remove_file};
use std::io::{self, ErrorKind, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha256::try_digest;
use tar::Builder;
use tar::Archive;

use std::io::Cursor;
use reqwest;

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
            println!("Failed to split string.");
            format!("aurders/{}.tar.gz", &source)
        }
    };

    let tar_gz = File::create(&tarball_name)?;

    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    match tar.append_dir_all(&source, &source) {
        Ok(_) => (),
        // Really wanted to do match e.kind() { NotADirectory };
        Err(e) => panic!("Failed to append source to tarball. Make sure source is a directory.\nGot: {}.", e),
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

/// create_directory creates directory according to given path
pub fn create_directory(path: String) {
    match fs::create_dir(&path) {
        Ok(_) => println!("Created directory {}.", &path),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => println!("Directory already exists."),
            ErrorKind::PermissionDenied => println!("Cannot create directory, permission denied"),
            _ => println!("Failed to create directory. Unknown error occurred.\nPath: {}.", &path),
        },
    };
}

/// decompress_tarball decompresses the tarball specified at tarball_path
fn decompress_tarball(tarball_path: String) -> Result<(), std::io::Error> {
    let tar_gz = File::open(tarball_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

/// fetch_data fetches the data from given url and writes to given filename
fn fetch_data(url: String, filename: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to fetch templates...");
    let response = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(filename)?;
    let mut content = Cursor::new(response);
    io::copy(&mut content, &mut file)?;
    println!("Fetched templates successfully.");

    Ok(())
}

/// get_templates retrieve the template by calling fetch_data() correctly
// TODO: Update url and filename
pub fn get_templates() {
    let url = "https://github.com/miteshhc/aurders/releases/download/template/templates.tar.gz";
    let filename = "templates.tar.gz";

    match fetch_data(url.to_string(), filename.to_string()) {
        Ok(_) => (),
        Err(e) => println!("Unable to fetch data: {}.", e),
    };

    match decompress_tarball(filename.to_string()) {
        Ok(_) => (),
        Err(e) => println!("Failed to decompress archive: {}.", e),
    };

    match remove_file(filename) {
        Ok(_) => println!("Removed file: {}.", filename),
        Err(e) => println!("Failed to remove {}: {}.", filename, e),
    };
}
