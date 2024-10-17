use std::io::Read;
use std::fs::File;

fn main() {
    println!("Hello, world!");

    let pkgbuild = get_pkgbuild();

    match pkgbuild {
        Ok(_) => {
            println!("Got PKGBUILD template");
        }
        Err(e) => {
            println!("Failed to open templates/PKGBUILD: {}", e);
        }
    }

    let srcinfo = get_srcinfo();

    match srcinfo {
        Ok(_) => {
            println!("Got SRCINFO template");
        }
        Err(e) => {
            println!("Failed to open templates/SRCINFO: {}", e);
        }
    }
}

fn get_pkgbuild() -> std::io::Result<String> {
    let mut file = File::open("templates/PKGBUILD")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_srcinfo() -> std::io::Result<String> {
    let mut file = File::open("templates/SRCINFO")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}
