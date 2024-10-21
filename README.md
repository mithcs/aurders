
  <h3 align="center">AURDERS</h3>

  <h4 align="center">
    Keep your package in order for AUR with aurders
  </h4>
</div>

___

aurders is a simple helper for developers to make their packages
ready-to-upload for Arch User Repository.

## Get Started

### Installation

#### From crates

```bash
cargo install aurders
```

#### From Github

Head to [Releases](https://github.com/miteshhc/aurders/releases) section and
grab the latest binary.

#### Compile from source

```bash
git clone git@github.com:miteshhc/aurders.git
cd aurders
cargo build --release
```

Executable binary will built at `target/release/aurders`

### Beforehand steps

- Ensure you have your source directory correctly named according to
  following format: `PkgName-PkgVer-RelNum`

- Ensure you have following dependencies ready:
    - AUR account (ready-to-use)
    - git
    - makepkg


### How To Use

1. Run `aurders` with source directory as argument.

```bash
aurders source_dir --templates
```

2. Enter the values for required fields as prompted.

3. Enter the commands for build() and package() functions.

4. Decide whether you want to commit changes manually or let aurders do it.

5. Enter commit message (if you let aurders do it).

6. That's all.

#### Refer to this [blog post](https://miteshhc.netlify.app/blog/02-introducting-aurders/) for more details

## What aurders does?

#### aurders handles most of the things one has to do to make their package ready-to-upload on Arch User Repository.

- Generates tarball from source directory.
- Generates PKGBUILD from template.
- Generates .SRCINFO from template.
- Clones repository from aur​@aur.archlinux.org of `pkgname`.
- Adds PKGBUILD, .SRCINFO and source (\*.pkg.tar.zst) to git repository.
- Commits the changes in the git repository (if user wants aurders to).

## What aurders does not do?

#### aurders focuses only on preparing package for upload to Arch User Repository.

- Does NOT create Arch User Repository account or handles anything related to that.
- Does NOT interfere with anything inside the .ssh/ directory.
- Does NOT push-es changes from local git repository.

## Be mindful of following:

- aurders currently supports only one source.
- Ensure you have AUR account correctly setup.
- You can set external source easily, but you are required to have local copy of that same package.
- The PKGBUILD and .SRCINFO templates does not contain all the fields, it contains only what is required and/or is standard.

## Contributing
All kinds of contributions are welcome! Whether you're fixing bugs, improving
documentation, adding features, or even suggesting ideas, your help is greatly
appreciated.

## LICENSE
Distributed under the MIT license. See [LICENSE](./LICENSE) for more infomation.
