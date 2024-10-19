mod args;
mod pkgbuild;
mod shared;
mod srcinfo;
mod utils;
mod final_step;

use pkgbuild::generate_pkgbuild;
use shared::get_information;
use srcinfo::generate_srcinfo;
use utils::dead;
use final_step::{add_to_repo, execute_makepkg, setup_repo};

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
    execute_makepkg();
    setup_repo(&pkginfo.pkgname, &pkginfo.pkgver, &pkginfo.pkgrel);
    add_to_repo(&pkginfo.pkgname);
}
