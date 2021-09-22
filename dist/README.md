### Description

This subproject is responsible for building the `deb` and `rpm` packages that are later distributed/released.

`deb` and `rpm` packages are underpinned by the `control` and `spec` files respectively. These contain much of the same information, which we centralize in a `properties.json` file. This file, alongside information extracted from the "core" cargo project, is used  to generate the needed `control` and `spec` files from templates. 
In other words, we have the templates under version control, and generate the real files during the build process leveraging a npm version of Liquid.

`deb` and `rpm` packaging have their own quirks. The process for each is described on the Node.js scripts. Scripts which are run on the build server (or locally) to create the `.deb` and/or `.rpm` packages

## Requirements to use locally

- Node v16
- Ubuntu distro
    - rpmbuild (not normally pre-installed)
    - dpkg-deb (normally pre-installed on Ubuntu)
    - strip (normally pre-installed on Ubuntu)