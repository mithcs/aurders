# aurders

## What is aurders?

aurders is simple tool which simplifies the process of uploading package to Arch User Repository. It handles the process of generating PKGBUILD, in a standard way, and .SRCINFO. aurders also clones the git repository from aur.archlinux.org and adds PKGBUILD and .SRCINFO in it.


## Dependencies

aurders is pretty minimal and depends on very few crates:

- **inquire**: For better user interaction
- **minreq**: For getting data from source to perform integrity checks
- **sha2**: For integrity checks

Excluding indirect dependencies.


## Installation

### Install from AUR:

```bash
yay -S aurders
```

### Install from crates:

```bash
cargo install aurders
```

### Get from Github:

Head to [releases](https://github.com/mithcs/aurders/releases) section and grab the latest executable.

### Compile from source:

```bash
git clone git@github.com:mithcs/aurders.git
cd aurders
cargo build --release
```
Executable will be built at `target/release/`


## Contributing

Contributions are always welcome!

## License

Distributed under the [MIT](https://choosealicense.com/licenses/mit/) license.

