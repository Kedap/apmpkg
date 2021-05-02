# INSTALACION

La opcion mas recomendable es que se dirja a la [seccion de lanzamientos](https://github.com/Kedap/apmpkg/releases/tag/1.0) y descargar los binarios, de igual manera se intentara distribuir para la mayoria de destribuciones y plataformas ¡SE PUEDE INSTALAR DESDE APMPKG! De igual manera en este markdown dara las manera de llevar la instalacion de ApmPKG a tu maquina.

Tabla de contenido:
1. [Instalar desde los binarios](#instalacion-de-los-binarios)
	1. [apt](#apt)
	2. [dnf](#dnf)
	3. [pacman](#pacman)
	4. [zypper](#zypper)
	5. [apmpkg](#apmpkg)
	6. [yay](#yay)
	7. [binario](#binario)
2. [Instalacion manual (compilacion)](#compilacion)
	1. [Post-Instalacion](#post-instalacion)

# instalacion de los binarios
Este es solo una gia de como es recomentable instalar los binarios con los gestores de paquetes a continuacion:

## apt
Apt es el gestor de paquetes para debian y sus derivados, para hacer un instalacion utilizando apt solo sobre escribir los siguientes comandos en su terminal:
`# wget https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg_1.0_amd64.deb; apt install ./apmpkg_1.0_amd64.deb`

De esta manera apt buscara por las dependencias necesarias para ApmPKG

## dnf
Dnf, la siguiente generacion de yum, recomendamos altamente utilizar dnf para la instalacion de ApmPKG para que sea una instalacion deseada como se debe, para instalar con esta herramienta es necesario ejecutar lo siguiente:

`wget https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg-1.0-1.fc33.x86_64.rpm; dnf localinstall apmpkg-1.0-1.fc33.x86_64.rpm`

De esta manera usted ya tendra instalado ApmPKG en su equipo

## pacman
Pacman...<. el gestor de paquetes de archlinux, de igual manera se puede instalar con este gestor, solo falta que ejecute:

`wget https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg-1.0-1-x86_64.pkg.tar.zst; pacman -U apmpkg-1.0-1-x86_64.pkg.tar.zst`

## zypper
Zypper es el gestor de paquetes de OpenSUSE y para que ApmPKG sea instalado solo falta ejecutar lo siguiente:

`wget https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg-1.0-1.fc33.x86_64.rpm; zypper in apmpkg-1.0-1.fc33.x86_64.rpm`

## apmpkg
Un gestor de paquetes universal para linux escrito en rust y bash. como habiamos dicho antes que para instalar ApmPKG tambien se puede utilizar para descargar ApmPKG, obviamente no va a poder descargar ApmPKG en ApmPKG sin que antes lo tenga instalado, este metodo se utiliza mas para poder actualizar el ApmPKG, pues solo falta que escriba el siguiente comando.
`apmpkg instalar -u https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg-1.0.abi.tar.gz`

## yay
*Yet another yogurt* ApmPKG tambien esta en [AUR](aur.archlinux.org) y que mejor manera que instalarlo con yay, con el siguiente comando

`yay -S apmpkg`

De igual manera hay mas versiones ApmPKG en AUR.

## binario
En la seccion de [lanzamientos](https://github.com/Kedap/apmpkg/releases/tag/1.0) puede descargarlo e instalarlo con el siguiente comando:
`wget https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg-bin-x86_64; mv apmpkg-bin-x86_64 /usr/bin/apmpkg`
Pero si quiere ejecutarlo de manera portable puede hacerlo:
`wget https://github.com/Kedap/apmpkg/releases/download/1.0/apmpkg-bin-x86_64; ./apmpkg-bin-x86_64 --help`


# compilacion

Para la instalacion y compilacion de manera manual debera de cumplir con los siguientes requisitos:

- Dependencias para compilar: git cargo pkg-config y openssl, openssl puede variar en diferentes distribuciones, este es necesario para openssl rust, [mas informacion aqui](https://docs.rs/openssl/0.10.33/openssl/index.html#automatic)
- Dependencias de ApmPKG: pip3/pip2, bundle, curl, fakeroot y git

Para empezar con el proceso de compilacion debera de ejecutar lo siguiente:

```
$ git clone https://github.com/kedap/apmpkg
$ cd apmpkg
$ cargo build --release
# cp target/release/apmpkg /usr/bin
# mkdir -p /etc/apmpkg/iiabc
# cp -r src/iiabc /etc/apmpkg/iiabc
# mkdir -p /etc/apmpkg/paquetes
```
## post-instalacion
### Manual
Para instalar los manuales solo ejecute:
```
# mkdir -p /usr/local/share/man/man1
# cp apmpkg.1 /usr/local/share/man/man1
```
## Ejecucion
`apmpkg --help`
