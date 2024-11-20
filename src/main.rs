mod pkgbuild;
mod srcinfo;

use pkgbuild::pkgbuild::pkgbuild;
use srcinfo::srcinfo;

fn main() {
    pkgbuild();
    srcinfo();
    println!("Hello World!")
}
