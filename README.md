# ApmPKG
[![version](https://img.shields.io/github/v/release/kedap/apmpkg)](https://github.com/Kedap/apmpkg/releases/) [![Build Status](https://travis-ci.com/Kedap/apmpkg.svg?branch=main)](https://travis-ci.com/Kedap/apmpkg) [![Paquete arch](https://img.shields.io/aur/version/apmpkg)](https://aur.archlinux.org/packages/apmpkg) [![AUR develop](https://img.shields.io/aur/version/apmpkg-git-dev)](https://aur.archlinux.org/packages/apmpkg-git-dev)

A Package Manager as a model: PKGBUILD
<p align="center">
  <img src="https://raw.githubusercontent.com/Kedap/apmpkg/main/img/logo.png" />
</p>

# NOTICIAS
- Se ha liberado la versión: v1.4.1 Para todos!!!
- Creacion de los binarios para todas las distribuciones disponibles
- Se a subido ApmPKG esta en [AUR](https://aur.archlinux.org/packages/apmpkg/)
- Se cambio slackpkg por slapt-get para administrar mejor los paquetes en Slackware
- Arreglo sobre npm para instalar modulos y no dependencias que no sean modulos de js
- Correcciones menores

* * *
Un gestor de paquetes que desea ser una poderosa herramienta universal para linux con el fin de la cracion e instalacion de paquetes.
![Captura](img/captura_prin.png)

## Crear paquetes

Esta es una herramienta escrita en rust y bash, que utiliza gestores de paquetes nativos para la resolucion de dependencias, se pueden crear paquetes desde un archivo simple con sintaxis TOML y hasta puedes de igual manera crearlo desde un PKGBUILD!
Los gestores que son soportado por ApmPKG:

- [x] Apt
- [x] Pacman
- [x] Dnf
- [x] Snap
- [x] Flatpak
- [x] Zypper
- [x] Yum
- [x] Apk
- [x] Pkg (termux)
- [x] Slapt-get
- [ ] Emerge
- [ ] Yay
- [ ] Nix

De igual manera se pueden crear binarios para una instalacion offline [binarios](doc/modos_de_instalacion.md/#instalacion-desde-un-archivo-binario-de-instalacion) para saber [mas infomarcion aqui](doc/modos_de_instalacion.md)

## Instalacion
* * *
Aunque sea algo dificl o raro, de igual manera podemos instalar apmpkg con el mismo apmpkg, aun porque creemos que la distribucion de paquetes es importante tratamos de poner a disposicion y de crear binarios nativos para cada distribucion en donde son soportados los gestores de paquetes,  pero primero debes de tener las depencias, entre ellos: 
- pip3/pip2
- npm
- bundle
- wget
- fakeroot
- git
- rsync

Para ello puedes dirijirte a la seccion de [lanzamientos](https://github.com/Kedap/apmpkg/releases/) en donde se suben los paquetes, si deseas tener mas informacion, [da click aqui](doc/instalacion.md)

# Caracteristicas
![Esquema](img/esquema.png)

- Creacion de paquetes de para lenguajes de scripting, un claro ejemplo es python, javascript/typescript y ruby. Con soporte con pip, bundle y npm [mas info aqui](doc/creando_paquetes.md/#adi)
- Facilidad de creacion de paquetes sobre el modelo de [PKGBUILD de archlinux](https://wiki.archlinux.org/index.php/PKGBUILD) es decir que tenemos soporte para AUR, aun que tenemos ciertas [limitaciones para decir que tenemos soporte para todos los PKGBUILD's existentes](doc/creando_paquetes.md/#complicaciones-abc) pero de igual manera estamos trabajando en eso
- Resolucion de dependencias con los gestores de paquetes antes mencionados, trabajaremos para que la lista sea mas extensa
- Descargar y/o compilar desde una url, ejemplo: `apmpkg instalar -u https://foo.com/bar`
- Extenciones propios de nuestra herramienta, *.adi, .abc y .abi.tar.gz* cada una tiene una funcion especial, [mas informacion aqui](doc/modos_de_instalacion.md)
- Crear un prototipo para un generar un archivo .adi o .abc, con el comando: `apmpkg crear adi foo` en el caso de crear un archivo .abc se debe de ejecutar lo siguiente: `apmpkg crear abc foo`
- Construir tus binarios, Con la llegada de la version 1.2 se puede construir binarios sin la necesidad de instalarlos, solo debes de ejecutar: `apmpkg construir foo.adi` o si es un paquete .abc: `apmpkg construir foo.abc`
- Con la llegada de la version 1.3 se puede ejecutar scripts post instalacion [mas info aqui](doc/creando_paquetes.md/#instalacion)
- En la version 1.4 se pueden ejecutar scripts pre instalacion

## Contribuir
Si tienes una buena idea o quieres contribuir con este proyecto puedes empezar por [aqui](https://github.com/Kedap/apmpkg/issues) y [leer esto](CONTRIBUTING.md)

## FAQ / Preguntas frecuentes

**¿Realmente es universal para TODAS las distribuciones gnu/linux disponibles?**

No, solo para aquellas que tenemos soporte para las dependencias, gestores de paquetes y arquitectura de estos.
de igual manera no todas las dependencias se llaman igual en todas las distribuciones que al igual se 
soluciona al preguntar por algun nombre de dependencias alternativas que se encuentre en sus gestor de paquete de preferencia

**¿Porque no existe un binario para X distribucion si el gestor X esta disponible?**

Aunque intentemos hacer eso no podemos, generamos los binario de las distribuciones mas utilizadas y aparte cada distribucion tiene sus arquitecturas y maneras de empaquetado, en el caso de que no brindemos dicho binario se recomienda altamente [compilarlo](doc/modos_de_instalacion.md)

**¿Existe una documentacion en ingles / Is there an English documentation?**

Esta en nuestro planes agregar una documentacion en ingles. It will soon be available 

**¿Porque no implentan X caracteristica?**

Por lo mismo, pues no compartes tu idea, estaremos felices de conocer tu idea, puedes apoyar [aqui](https://github.com/Kedap/apmpkg/issues) y/o si quieres mas informacion [aqui](CONTRIBUTING.md)
