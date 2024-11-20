mod git;
mod pkgbuild;
mod srcinfo;

use git::git;
use pkgbuild::pkgbuild::pkgbuild;
use srcinfo::srcinfo;

fn main() {
    let pkgname = pkgbuild();
    srcinfo();
    git(pkgname);

    println!("That's all");
}
