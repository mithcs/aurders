<div align="center">
  <h3 align="center">AURDERS</h3>

  <h4 align="center">
    Keep your package in order for AUR with aurders
  </h4>
</div>

___

aurders is a simple helper for developers to make their packages
ready-to-upload for Arch User Repository.

## What aurders does?

#### aurders handles most of the things one has to do to make their package ready-to-upload on Arch User Repository.

- Generates tarball from source directory.
- Generates PKGBUILD from template.
- Generates .SRCINFO from template.
- Clones repository from aur@aur.archlinux.​org of `pkgname`.
- Adds PKGBUILD, .SRCINFO and source (\*.pkg.tar.zst) to git repository.
- Commits the changes in the git repository.

## What aurders does not do?

#### aurders focuses only on preparing package for upload to Arch User Repository.

- Does NOT create Arch User Repository account or handles anything related to that.
- Does NOT interfere with anything inside the .ssh/ directory.
- Does NOT push-es changes from local git repository.

## Be mindful of following:

- aurders currently supports only one source.
- You can set external source easily, but you are required to have local copy of that same package.
- The PKGBUILD and .SRCINFO templates does not contain all the fields, it contains only what is required and/or is standard.

## TODO
- [ ] Ask user before commiting changes in git repository
- [ ] Fix `final_step` module doc
- [ ] Add how to with examples section
- [ ] Add support for multiple data sources
- [ ] Refactor everything
- [X] Test untested features
- [X] Allow users to add multiple dependencies easily
- [X] Fix implementation for copying package


## Contributing
We welcome contributions of all kinds! Whether you're fixing bugs, improving
documentation, adding features, or even suggesting ideas, your help is greatly
appreciated.

## LICENSE
Distributed under the MIT license. See [LICENSE](./LICENSE) for more infomation.
