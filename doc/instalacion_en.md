# INSTALLATION

The most recommended option is to go to the [releases section](https://github.com/Kedap/apmpkg/releases/) and download the binaries, in the same way it will try to distribute for most distributions and platforms! IT CAN BE INSTALLED FROM APMPKG! In the same way, in this markdown it will give the way to take the ApmPKG installation to your machine.

[Spanish](./instalacion_en.md)

Table of Contents:
1. [Install from binaries](#installation-of-binaries)
	1. [apt](#apt)
	2. [dnf](#dnf)
	3. [pacman](#pacman)
	4. [zypper](#zypper)
	5. [apmpkg](#apmpkg)
	6. [yay](#yay)
	7. [apk](#apk)
	8. [binary](#binary)
2. [Manual installation (build)](#build)
	1. [Post-Installation](#post-installation)

# Installation of binaries
This is just a guide on how it is recommended to install the binaries with the package managers below:

## Apt
Apt is the package manager for debian and its derivatives, to do an installation using apt just by typing the following commands in your terminal:
`wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-1.3.0-amd64.deb; apt install ./apmpkg-1.3.0-amd64.deb`

In this way apt searches for the necessary dependencies for ApmPKG

## Dnf
Dnf, the next generation of yum, we highly recommend using dnf for the installation of ApmPKG so that it is a desired installation as it should, in the event that the installation does not work with this binary you can use the other one destined for zypper, to install with this tool it is necessary to execute the following:

`wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-1.3.0-1.fc34.x86_64.rpm; dnf localinstall apmpkg-1.3.0-1.fc34.x86_64.rpm`

In this way you will already have Apm PKG installed on your computer

## Pacman
Pacman ...<. the archlinux package manager, in the same way it can be installed with this manager, you just need to run:

`wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-1.3.0-3-x86_64.pkg.tar.zst; pacman -U apmpkg-1.3.0-3-x86_64.pkg.tar.zst`

Or in a better way you can have the latest versions with the repository [krep0](https://krep0.bitbucket.io/archlinux/), if you don't have it in your pacman.conf, you should do the following:
Place the following lines in `/etc/pacman.conf`:
```toml
[krep0]
SigLevel = Optional TrustAll
Server = https://$repo.bitbucket.io/archlinux/$arch
```
And update with `pacman -Syu`

Once you have krep0 in your pacman.conf you must execute the following to install apmpkg:

```sh
pacman -S apmpkg
```

In case you want to install the development version (not recommended) you should run:
```sh
pacman -S apmpkg-dev
```

## Zypper
Zypper is the OpenSUSE package manager and for ApmPKG to be installed, you just need to execute the following:

`wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-1.3.0-1.fc34.x86_64.rpm; zypper in apmpkg-1.3.0-1.fc34.x86_64.rpm`

## Apmpkg
A universal package manager for linux written in rust and bash. As we said before that to install ApmPKG it can also be used to download ApmPKG, obviously you will not be able to download ApmPKG in ApmPKG without first having it installed, this method is used more to update ApmPKG, because you just need to write the following command.
`apmpkg instalar -u https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-1.3.0.abi.tar.gz`

## Yay
*Yet another yogurt* or with any other helper to install AUR packages, ApmPKG is also in [AUR](https://aur.archlinux.org/packages/apmpkg) and what better way than to install it with yay, with the following command

`yay -S apmpkg`

Similarly there are more ApmPKG versions in AUR.

## Apk
Alpine linux package manager, and in this update we have support for it,
why don't you install it with:
```
wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-1.3.0-r0.apk; apk add --allow-untrusted apmpkg-1.3.0-r0.apk
```
And if you want to have the documentation installed, try with:
```
wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-doc-1.3.0-r0.apk;apk apk add --allow-untrusted apmpkg-doc-1.3.0-r0.apk
```
Or in your case it can be installed from the testing branch of [alpine](https://wiki.alpinelinux.org/wiki/Alpine_Linux_package_management#Repository_pinning)

```sh
apk add apmpkg@testing
```

## Binary
In the [releases section](https://github.com/Kedap/apmpkg/releases/) you can download and install it with the following command:
`wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-bin-x86_64; mv apmpkg-bin-x86_64 /usr/bin/apmpkg`
But if you want to run it in a portable way you can do it:
`wget https://github.com/Kedap/apmpkg/releases/download/1.3.0/apmpkg-bin-x86_64; ./apmpkg-bin-x86_64 --help`


# Build

For manual installation and compilation, you must meet the following requirements:

- Dependencies to compile: git, cargo, pkg-config, openssl and openssl can vary in different distributions, this is necessary for openssl rust, [more information here](https://docs.rs/openssl/0.10.33/openssl/ index.html # automatic)
- ApmPKG dependencies: pip3 / pip2, bundle, wget, fakeroot, rsync, npm and git

To start the compilation process, you must execute the following:

```
$ git clone https://github.com/kedap/apmpkg
$ cd apmpkg
$ cargo build --release
# cp target/release/apmpkg /usr/bin
# mkdir -p /etc/apmpkg/iiabc
# cp -r src/iiabc /etc/apmpkg/iiabc
# mkdir -p /etc/apmpkg/paquetes
```
## Post-installation
### Manually
To install the manuals just run, as you need to have man installed to be able to read the manual pages since many distributions do not have it installed by default
```
# mkdir -p /usr/local/share/man/man1
# cp man/* /usr/local/share/man/man1
```
## Execute
`apmpkg --help`
