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

### What aurders does?

aurders is a tool to help developers easily publish their package to Arch User
Repository.

- Generates tarball from source directory.
- Generates PKGBUILD from template.
- Generates .SRCINFO from template.
- Clones repository from aur@aur.archlinux.org.
- Adds PKGBUILD, .SRCINFO and source (*.pkg.tar.zst) to git repository.
- Commits the changes in the git repository.

### What aurders does not do?

aurders just helps in making your package ready-to-upload to Arch User
Repository. And not tries to do more than that.

- Does not creates Arch User Repository account or handles anything related to that.
- Does not interferes with anything inside .ssh/ directory.
- Does not push-es changes from local git repository.

### External Source

If you want to set external source, make sure you have a local copy. You have
to specify that local copy in source arguments. And when the source part asks
for source specify that URL.


## TODO
- [ ] Add support for multiple data sources
- [ ] Allow users to add multiple dependencies easily
- [ ] Allow users to populate `build()` function in PKGBUILD
- [ ] Allow users to populate `package()` function in PKGBUILD
- [ ] Fix `final_step` module doc
- [ ] Fix implementation for copying package
- [ ] Completely implement different methods for setting source(s)
- [ ] Test untested features


## Contributing
Contributions, issues, pull requests and suggestions are welcome!

## LICENSE
Distributed under the MIT license. See [LICENSE](./LICENSE) for more infomation.
