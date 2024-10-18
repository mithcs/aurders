mod args;
mod pkgbuild;
mod srcinfo;
mod utils;
mod shared;

use pkgbuild::generate_pkgbuild;
use shared::get_information;
use srcinfo::generate_srcinfo;

use std::fs;

use crate::shared::Information;

fn main() {
    match fs::create_dir("aurders") {
        Ok(_) => println!("Created directory."),
        Err(e) => println!("Failed to create directory: {}.", e),
    };

    let info_result = get_information();
    let pkginfo: Information;

    match info_result {
        Some(info) => pkginfo = info,
        None => panic!("Failed to get information."),
    };

    generate_pkgbuild(&pkginfo);
    generate_srcinfo(&pkginfo);
}
