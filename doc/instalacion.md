# INSTALACIÓN

La opción mas recomendable es que se dirija a la [sección de
lanzamientos](https://github.com/Kedap/apmpkg/releases/) y descargar los
binarios, de igual manera se intentara distribuir para la mayoría de
distribuciones y plataformas ¡SE PUEDE INSTALAR DESDE APMPKG! De igual manera
en este Markdown dará las manera de llevar la instalación de ApmPKG a tu
maquina.

[English](./instalacion_en.md)

Tabla de contenido:
1. [Instalar desde los binarios](#instalacion-de-los-binarios)
	1. [apt](#apt)
	2. [dnf](#dnf)
	3. [pacman](#pacman)
	4. [zypper](#zypper)
	5. [apmpkg](#apmpkg)
	6. [yay](#yay)
	7. [apk](#apk)
2. [Instalación manual (compilación)](#compilacion)
	1. [Post-Instalación](#post-instalacion)

# Instalacion de los binarios
Este es solo una guia de como es recomendable instalar los binarios con los
gestores de paquetes a continuación:

## Apt
Apt es el gestor de paquetes para Debian y sus derivados, para hacer un
instalación utilizando Apt solo sobre escribir los siguientes comandos en su
terminal:

`wget https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-amd64.deb;
apt install ./apmpkg-amd64.deb`

De esta manera Apt buscara por las dependencias necesarias para ApmPKG

## Dnf
Dnf, la siguiente generación de Yum, recomendamos altamente utilizar Dnf para
la instalación de ApmPKG para que sea una instalación deseada como se debe, en
el caso de que no funcione la instalación con este binario se puede utilizar el
otro destinado para Zypper, para instalar con esta herramienta es necesario
ejecutar lo siguiente:

`wget https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-1.5.1-1.fc34.x86_64.rpm;
dnf localinstall apmpkg-1.5.1-1.fc34.x86_64.rpm`

De esta manera usted ya tendrá instalado ApmPKG en su equipo

## Pacman
Pacman el gestor de paquetes de archlinux, de igual manera se puede instalar
con este gestor, solo falta que ejecute:

`wget https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-1.5.1-1-x86_64.pkg.tar.zst;
pacman -U apmpkg-1.5.1-1-x86_64.pkg.tar.zst`

O de mejor manera puedes tener las ultimas versión con el repositorio
[krep0](https://krep0.bitbucket.io/archlinux/), si no lo tienes en tu
pacman.conf, deberás de colocar lo siguiente en `/etc/pacman.conf`:

```toml
[krep0]
Server = http://164.90.155.18/repository/archlinux
Server = https://$repo.bitbucket.io/archlinux/$arch
```

Después deberás de configurar las llaves publicas con las cuales están firmados
los paquetes, existen dos formas fáciles, la manual y la automatizada. La forma
automatizada es la de ejecutar el siguiente comando:

```sh
$ curl -O https://krep0.bitbucket.io/archlinux/key-krep0.sh
$ bash key-krep0.sh
```

O si quieres hacerlo de manera manual ejecuta:

```sh
$ curl -O https://krep0.bitbucket.io/archlinux/kedap.pub && sudo pacman-key -a kedap.pub
```

Y actualizar con `pacman -Syu`

Una vez que ya tengas krep0 en tu pacman.conf deberás de ejecutar lo siguiente para instalar apmpkg:

```sh
pacman -S apmpkg
```

En el caso de que quieras instalar la versión en desarrollo (no recomendado) deberás ejecutar:
```sh
pacman -S apmpkg-dev
```

## Zypper
Zypper es el gestor de paquetes de OpenSUSE y para que ApmPKG sea instalado solo falta ejecutar lo siguiente:

`wget https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-1.5.1-1.fc34.x86_64.rpm;
zypper in apmpkg-1.5.1-1.fc34.x86_64.rpm`

## ApmPKG
Un gestor de paquetes universal para Linux escrito en Rust y Bash. Como
habíamos dicho antes que para instalar ApmPKG también se puede utilizar para
descargar ApmPKG, obviamente no va a poder descargar ApmPKG en ApmPKG sin que
antes lo tenga instalado, este método se utiliza mas para poder actualizar el
ApmPKG, pues solo falta que escriba el siguiente comando.

`apmpkg instalar -u https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-1.5.1.abi.tar.gz`

## Yay
*Yet another yogurt* o con cualquier otro ayudador para instalar paquetes AUR,
ApmPKG también esta en [AUR](https://aur.archlinux.org/packages/apmpkg) y que
mejor manera que instalarlo con yay, con el siguiente comando

`yay -S apmpkg`

De igual manera hay mas versiones ApmPKG en AUR.

## Apk
Gestor de paquetes de alpine Linux, y en esta actualización tenemos soporte para esta,
por que no lo instalas con:

```
wget https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-1.5.1-r0.apk;
apk add --allow-untrusted apmpkg-1.5.1-r0.apk
```

Y si quieres tener la documentación instalada, prueba con:

```
wget https://github.com/Kedap/apmpkg/releases/download/1.5.1/apmpkg-doc-1.5.1-r0.apk;
apk add --allow-untrusted apmpkg-doc-1.5.1-r0.apk
```

O en su caso se puede instalar desde la rama testing de [alpine](https://wiki.alpinelinux.org/wiki/Alpine_Linux_package_management#Repository_pinning)

```sh
apk add apmpkg@testing
```

# Compilacion

Para la instalación y compilación de manera manual deberá de cumplir con los siguientes requisitos:

- Dependencias para compilar: git, cargo, pkg-config , openssl y openssl puede
  variar en diferentes distribuciones, este es necesario para openssl Rust,
  [mas informacion aqui](https://docs.rs/openssl/0.10.33/openssl/index.html#automatic)
- Dependencias de ApmPKG: pip3/pip2, bundle, wget, fakeroot, rsync, npm y git

Para empezar con el proceso de compilación deberá de ejecutar lo siguiente:

```
$ git clone https://github.com/kedap/apmpkg
$ cd apmpkg
$ cargo build --release
# cp target/release/apmpkg /usr/bin
# mkdir -p /etc/apmpkg/iiabc
# cp -r src/iiabc /etc/apmpkg/iiabc
# mkdir -p /etc/apmpkg/paquetes
```

## Post-instalacion
### Manual
Para instalar los manuales solo ejecute al igual se necesita tener instalado
man para poder leer las paginas manuales ya que muchas distribuciones no se
tiene instalado por defecto

```
# mkdir -p /usr/local/share/man/man1
# cp man/* /usr/local/share/man/man1
```

### Completions
Para instalar los completions solo necesitas ejecutar lo siguiente según tu shell

#### Bash
Para instalar los completions de Bash deberás ejecutar lo siguiente:

```
[user@pc-pro]$ install -Dm644 completions/apmpkg.bash-completion /usr/share/bash-completion/completions/apmpkg
```

#### Zsh
Para instalar en Zsh
```
% install -Dm644 completions/_apmpkg /usr/share/zsh/site-functions/_apmpkg
```

#### Fish
Si utiliza la shell de Fish lo que tendrá que ejecutar sera lo siguiente:

```
user@pc-pro ~ install -Dm644 completions/apmpkg.fish /usr/share/fish/vendor_completions.d/apmpkg.fish
```

## Ejecucion
`apmpkg --help`
