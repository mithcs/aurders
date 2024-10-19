mod args;
mod pkgbuild;
mod shared;
mod srcinfo;
mod utils;

use pkgbuild::generate_pkgbuild;
use shared::get_information;
use srcinfo::generate_srcinfo;
use utils::dead;

use shared::Information;

fn main() {
    let info_result = get_information();
    let pkginfo: Information;

    match info_result {
        Some(info) => pkginfo = info,
        None => {
            eprintln!("Failed to get information.");
            dead();
            return; // rust made me do this
        }
    };

    generate_pkgbuild(&pkginfo);
    generate_srcinfo(&pkginfo);
}
