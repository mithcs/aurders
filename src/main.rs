mod pkgbuild;
mod srcinfo;
mod git;

use pkgbuild::pkgbuild::pkgbuild;
use srcinfo::srcinfo;
use git::git;

fn main() {
    let pkgname = pkgbuild();
    srcinfo();
    git(pkgname);
    println!("Hello World!")
}
