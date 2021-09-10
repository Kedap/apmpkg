# Guide to creating packages using ApmPKG
In this guide you will learn everything you need to know when creating packages for ApmPKG, in this way we can extend its use much more and have packages for linux

[Spanish](./creando_paquetes.md)

Table of Contents
1. [Build a Archivo de Descarga e Instalacion (Download and Installation File): ADI](#adi)
	1. [Package data](#package)
	2. [External dependencies](#dependencies-adi)
	3. [Ruby gems](#gem)
	4. [Pip2 / pip3](#pip)
    5. [Npm](#npm)
	6. [download](#download)
	7. [installation](#installation)
2. [Compile and install from a Archivo de Bash y Compilando (Bash File and Compiling)](#abc)
	1. [Errors with abc](#complications-abc)
3. [Generate a file easily](#create-command)
4. [FAQs / Frequently Asked Questions](#frequent-questions)

# Adi
Your name from the acronym for (spanish):
**A**rchivo de
**D**escarga e
**I**nstalacion

**D**ownload and 
**I**nstallation 
**F**ile
(English)

This method was created with the purpose of being able to create applications written in python and / or ruby since ApmPKG has support for Pip and Bundle, in this way it seeks to be easier and more practical to create applications with these languages. First; what an ADI file looks like, then here is a prototype of this:
```
[paquete]

nombre = "foo"
version = "1.1"
rama = "estable" # git / beta
descrip = "Ejemplo para el prototipo de apmpkg"
pagina = "https://foo.com/"
licensia = "GPL-V3"
dependencias = ["ruby", "metasploit"]
#cmd_depen = ["ruby" , "msfconsole"]
#abi_dependencias = ["metasploit"]
#arch = "x86_64"
conflicto = "/opt/foo/"

#[dependencias_adi]
#metasploit = "https://foo/bar/alterntiva/metasploit.abi.tar.gz"

[gem]

gemfile = true
file = "Gemfile"
#gemfile = false
#gemas = ["colorized", "rails"]

#[pip]

#version = 3 / 2
#requirements = false / true
#packages = ["requests", "pytest-shutil", "objdict"]
#file = "requeriments.txt"

#[npm]
#package_json = true / false
#ruta_package_json = "package.json"
#modulos = ["angular", "electron"]

[descarga]

url = "https://foo.com/bar.tar.gz"
carpeta = "foo-bar"
#git = "https://serviciogit.com/foo/bar"
#local = "/path/de/las/fuentes/foo.tar.gz"
sha256sum = "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc" # SALTAR

[instalacion]

#opt_src = true
pre_install = "pre_apmpkg.sh"
files = ["main.rb" , "config.conf", "porfile_user_default.py"]
ruta = ["/usr/bin/foo", "/etc/foo/config.conf", ".local/share/porfile_app.py"]
post_install = "post_apmpkg.sh"
mensaje = "Para poder ejecutar, prueba con 'foo'!"
```
A lot of information, we go by steps, ADI has the TOML syntax to make it easier to create packages, in this way we are going to see each of the lines:

## Package
The package start is nothing more and nothing less than the data of the package to which to install, it is written with the start of `[paquete]`, something like that:
```
nombre = "foo"
version = "1.1"
rama = "estable" # git / beta
descrip = "Ejemplo para el prototipo de apmpkg"
pagina = "https://foo.com/"
licensia = "GPL-V3"
dependencias = ["ruby", "metasploit"]
#cmd_depen = ["ruby" , "msfconsole"]
conflicto = "/opt/foo/"
#abi_dependencias = ["metasploit"]
```
A little easier, right? From here we see basic things like the name and the version that are strings, nothing very important to discard, but we see something in the branch.
The **branch** variable is a string that is used to differentiate between what the package is, if it is a beta version, if it is from the git or development branch or if it is a stable version.

Let's go to the following which is **descrip**, and **license**. These are strings where you place a short description of the package and the license to specify what type of package license it is.

**dependencies** and **cmd-depen**: dependencies is an array where the name of the packages to which they should be installed are placed, and cmd_depen is something very curious, since to verify that the dependencies are installed, run a command; In other words, if the dependency is python after executing the installation command, `python` is executed and if an output of 127 is obtained, it is assumed that the dependency is installed, but nevertheless there are packages that are executed in different ways such as It is the example of `openssh` that is executed with` ssh` or in this case `metasploit` that is executed with` msfconsole` that is why this array was created. However, it is not necessary IF ALL the dependencies are executed with the same name with which it is installed, as is the case of `ruby`

**conflict**: This string must contain a path, if said path or file exists it cannot be installed, that is, it prevents a package from being installed when it is already installed with another package manager

**abi_dependencies** It is an array where dependencies are placed that **in the case** of not being found and/or resolved with native package managers, these dependencies will be installed externally by ApmPKG, see more information in [dependencies_adi](#dependencies-adi)

**arch** It is a string variable that must be placed in the event that said package is compatible only for said architecture, in the event that it is available for all architectures, this variable should not be placed, since if said variable it is not placed it is assumed that the package is built for any architecture

## Adi dependencies

This section was integrated with version 1.2.0, the function of this section is to grant the sources of dependencies that have not been resolved with the native package managers, either because they are not in the repository or because they are found broken and/or for some other reason ...

```toml
metasploit = "https://foo/bar/alterntiva/metasploit.abi.tar.gz"
```

We have a simple line, the variables in this section will have the name of the dependency that in the case of not being fulfilled will be installed through this option, in this case the **metasploit** dependency, the value of said variable will make a string containing the path from where to download said package in .abi.tar.gz format so that it can be installed by apmpkg

## Gem
This section was created to contain information regarding gems that the package depends on, of course ruby gems and that will be installed with `bundle` or` gem` This section begins with `[gem]`, this section is not mandatory, it is only placed If the project depends on gems, if the case is yes, here is an example:
```
gemfile = true
file = "Gemfile"
#gemfile = false
#gemas = ["colorized", "rails"]
```
The **gemfile** variable is a boolean (true/false) that it must be placed in a mandatory way, this is to know if the project has a Gemfile to download the gems with this file, in the case that the project It contains a Gemfile, this must be set as true and if not then as false.

**file** this variable is ONLY placed if the gemfile contains true, since here in this string the path of the Gemfile will be specified, in this case only "Gemfile" is set since it is found in the project folder

**gems** this is an array where the gems on which it depends are specified, it is recommended that it must be few, few gems how to create a Gemfile but not so many, it is designed for projects that depend on 2 or 3 gems

## Pip
Like ruby, python also has dependencies, these will be installed with pip by ApmPKG, in the same way this field is not mandatory unless the project requires it, we have support for pip2 and pip3, this is placed with `[pip] `below what it must contain:
```
version = 3 # 2
requirements = true # false
#packages = ["requests", "pytest-shutil", "objdict"]
file = "requeriments.txt"
```
In **version** it must contain an integer and the version number of python / pip with which the dependencies will be installed, in the example we can see that it will be installed with pip3.

En **requirements** es un boleano donde se debe de colocar true si es que el proyecto instala sus dependencias con el archivo requeriments.txt y false si no

In **packages** it is an array where the dependencies are specified, this should not be put if the requirements are set to false.

On the contrary **file** is a string where the path of the file requirements.txt is specified so that from here the dependencies are installed with pip

## Npm
In this section we can find a space to correctly manage the npm modules, in an adi file it can be seen like this:
```
#[npm]
#package_json = true / false
#ruta_package_json = "package.json"
#modulos = ["angular", "electron"]
```

In the variable **package.json** it is a boolean that indicates if the modules will be installed from a `package_json`

In the **package_json_path** it is a string of the path where the `package_json` is located within the project

On the array **modules** in case package_json is false it should contain the modules that should be installed

## Download
The download section is for this, where you specify the details of the download, it starts `[download] `in one of the examples:
```
url = "https://foo.com/bar.tar.gz"
#git = "https://serviciogit.com/foo/bar"
#local = "/path/local/de/las/fuentes"
carpeta = "foo-bar"
sha256sum = "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc" # SALTAR
```
The **url** variable is a string where the package download link is said, it must be compressed in `tar.gz` since it will be extracted with this compression format, but nevertheless in the git versions it has been created the **git** variable that is placed instead of url, when placing git the repository described here will be cloned, or failing that, if you have one of the sources on your computer, you can place its path in the variable **local**

In **folder** the directory that must be accessed once the tar.gz has been extracted or the git cloned is placed

In **sha256sums** you must enter the sha256 sums of the file to download, in the case that a git version is used, this must change its value as: `sha256sums =" SKIP "` in this way get verification by sha256

## Installation
Here the information regarding the installation path is focused, this starts with `[installation]` example:
```
#opt_src = true
pre_install = "pre_install.sh"
files = ["main.rb" , "config.conf", "porfile_user_default.py"]
ruta = ["/usr/bin/foo", "/etc/foo/config.conf", ".local/share/porfile_app.py"]
post_install = "post_apmpkg.sh"
mensaje = "Para poder ejecutar, prueba con 'foo'!"
```
The **opt_src** variable is a boolean that admits true or false if you want the entire directory obtained by git or by the download to be copied to the /opt folder, an example of this is the metasploit package that is installed in the opt folder.

**files** and **path** are both arrays that contain file paths, files select the files to be installed and path the path where they are to be installed, the first selected file will be installed with ` install -Dm 755` since the index 0 of both arrays is assumed to be binary. in case you want to place a file inside `/home` you must put a relative path (not root)
as in this example ".local/share/foo.bar" will be installed inside `/home` in any of the existing users

We will find with the **post_install** variable that is no longer a string variable that stores the path of the script written in bash that will be executed once the package is installed in the case of giving another output that is not exit code 0, it will be given by failed 

In the pre_install variable you will find the path where the bash script is, like the post_install script, it will be considered as failed if it returns an output other than 0

## Abc
Su nombre es el acronimo de (spanish):
**A**rchivo de
**B**ash y 
**C**ompilacion

(english)
**B**ash 
**F**ile and 
**C**ompilation

This type of file is based on and/or cloned from the [PKGBUILD](https://wiki.archlinux.org/index.php/PKGBUILD) of archlinux, it is not necessary to explain what this great archlinux model can do, but without However, we will focus on what ApmPKG cannot do with a PKGBUILD because we still cannot have absolute compatibility, but we are working so that it is not so in the future, so we present the limitations in relation to abc
## Complications abc
We currently do not have support with the following variables
- groups: We currently do not have groups for packages
- depends: As we saw that to create ADI and how dependencies are checked is [executing](# package), with abc files they are checked looking for the name of the dependency in `/bin` and in` /usr/bin` and if not then It is given as a dependency not installed. 
- optdepeds: We just don't use this variable like provides, conflicts, backups, options, install, changelog and all non-sha256 sums
In the same way, with the arrival of version 1.0.1, the `cmd_depen` variable was implemented so that the dependency can also be checked if this command is executed, giving 127 as output, it is considered not installed


# Create command

In the update of version 1.1 we have integrated a subcommand for the creation of a prototype of which it is necessary to create, which is the following:
```
apmpkg crear <abc | adi> <Nombre del paquete a crear>
```
So to facilitate the creation of a package

# Frequent questions
**Do PKGBUILD or .abc files work on any right-handed than archlinux?**
Yes, since a process similar to `makepkg` is carried out, but nevertheless` iiabc` (interpreter for the installation with .abc) does a similar but very different process, like the binaries that are generated are similar to a `pkg. tar.xz` but very different from `adi.tar.gz`

**I have dependency issues! In X distribution the dependencies are installed but in Y not**
This is a warning, that is to say that a solution will be found as quickly as possible when launching ApmPKG since there are packages with different names in different repositories, an example is the package `openssh` that in archlinux is called` openssh` but in debian it is call `ssh`. A temporary solution would be to modify the name of the dependencies for each distribution, but we are working to eliminate this problem. But with version 1.0.1 the solution was given of being able to ask the user for a package that is not found

**Is root permissions required?**
To install packages you need to be root, but nevertheless to create binaries it is not necessary
