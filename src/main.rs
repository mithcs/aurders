mod args;
mod pkgbuild;
mod srcinfo;
mod utils;
mod shared;

use pkgbuild::generate_pkgbuild;
use shared::get_information;
use srcinfo::generate_srcinfo;
use utils::create_directory;

use crate::shared::Information;
fn main() {
    create_directory("aurders".to_string());

    let info_result = get_information();
    let pkginfo: Information;

    match info_result {
        Some(info) => pkginfo = info,
        None => panic!("Failed to get information."),
    };

    generate_pkgbuild(&pkginfo);
    generate_srcinfo(&pkginfo);
}
