# INSTALLATION MODES

In ApmPKG we have 2 ways to install packages, obviously already made before as
stated in [creating packages](./creando_paquetes_en.md) to share these packages
can be done from a [.adi file](creating_packages.md#adi), [file
.abc](creating_packages.md#abc) and/or from a binary file (.abi.tar.gz) made by
yourself or someone else. Although it is the same for all possible files. Let's
see the installation modes:

[Spanish](./modos_de_instalacion.md)

Table of Contents:

1. [Install from an ADI file](#installing-from-a-download-and-installation-file)
   1. [Installing from a web-hosted ADI file](#from-web)
2. [installation from an ABC file](#install-from-a-file-abc)
   1. [Installation from an ABC file hosted on the web](#abc-from-web)
3. [Install from a file ADI.TAR.GZ](#installing-from-an-installation-binary-file)
   1. [Install from a file ADI.TAR.GZ](#binary-from-the-internet)

## Installing from a download and installation file

As we will remember, a .adi file is nothing more than a document with TOML
syntax for the creation of packages written in some scripting language, also
for other languages, but this is its recommended use. To install from a .adi
file you can type the following command:

`# apmpkg instalar foo.adi`

The installation of said packages will begin immediately, what this process
does is download and install dependencies as well as dependencies of pip or
bundle, as the case may be, once this step is carried out, the next thing to do
is install the files specified in this file and thus conclude with the
installation.

### From web

If you want to install from a web-hosted .adi file, you can perform the following command:

`# apmpkg instalar -u https://foo.com/bar.adi`

This only downloads the specified file and will proceed to the install function as in the process above.

## Install from a file abc

The .ABC files are the ones that will be seen the most from the panorama from
this tool, this type of files are read by [iiabc](./creando_paquetes_en.md#abc)
that we had agreed in [creating
packages](./creando_paquetes_en.md#complications-abc) this is a file similar or
the same as a [PKGBUILD](https://wiki.archlinux.org/index.php/PKGBUILD) from
archlinux, but of course we have [certain problems](./creando_paquetes_en.md)
with the compatibility of ALL existing PKGBUILD's, but we will continue working
so that it is not so. To install from an .ABC file, we can execute the
following command:
`# apmpkg instalar foo.abc`

### Abc from web

In the same way, the same command is used:
`# apmpkg instalar -u https://foo.com/bar.abc`

## Installing from an installation binary file

The .ABI.TAR.GZ files are for an offline installation to be possible, this is
where you have all the files necessary for an installation and metadata of the
package to be interpreted by apmpkg, as everything here with this article is
used the same command for everything, even so in case there is any doubt we
must make it clear. For installation from an ABI.TAR.GZ file:

`# apmpkg instalar foo.abi.tar.gz`

### Binary from the internet

Similarly, the command is the same. ApmPKG downloads the specified file and
starts the installation process with this type of file, in a normal way, as it
does not affect whether it has been created from an ADI or an ABC, ApmPKG will
know what type of file is with which this package was made And in the same way
it will install the files, to install with the ADI file that is hosted on some
website can be done with the following command:
`# apmpkg instalar -u https://foo.com/bar.abi.tar.gz`

conclusion:
`# apmpkg instalar foo.bar` if it is a local installation, that is, you already
have the file on your computer and if not: `# apmpkg instalar -u https://foo.com/bar.bar`
