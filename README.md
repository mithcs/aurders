<div align="center">
  <h3 align="center">AURDERS</h3>

  <h4 align="center">
    Keep your package in order for AUR with aurders
  </h4>
</div>

___

aurders is a simple helper for developers to make their packages
ready-to-upload for Arch User Repository.

#### ⚠️ Currently only single source is supported.

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

### External Source

If you want to use external source, ensure you have a local copy. Specify that
local copy in the source arguments, and provide the corresponding URL when
asked for the source.

## TODO
- [ ] Add support for multiple data sources
- [ ] Allow users to add multiple dependencies easily
- [ ] Fix `final_step` module doc
- [ ] Fix implementation for copying package
- [ ] Completely implement different methods for setting source(s)
- [ ] Test untested features


## Contributing
Contributions, issues, pull requests and suggestions are welcome!

## LICENSE
Distributed under the MIT license. See [LICENSE](./LICENSE) for more infomation.
