# MODOS DE INSTALACION
En ApmPKG tenemos 2 formas de instalar paquetes, obviamente antes ya hechos
como se dice en [creando paquetes](creando_paquetes.md) para compartir estos
paquetes se pueden hacer desde un [archivo .adi](creando_paquetes.md#adi),
[archivo.abc](creando_paquetes.md#abc) y/o desde un archivo binario
(.abi.tar.gz) realizados por usted mismo o por otra persona. Aunque es lo mismo
para todos lo archivos posibles. Veamos los modos de instalación:

[English](./modos_de_instalacion_en.md)

Tabla de contenido:
1. [Instalacion desde un archivo ADI](#instalacion-desde-un-archivo-de-descarga-e-instalacion)
	1. [Instalacion desde un archivo ADI alojado en la web](#desde-la-web)
2. [instalacion desde un archivo ABC](#instalacion-desde-un-archivo-de-bash-y-compilacion)
	1. [Instalacion desde un archivo ABC alojado en la web](#abc-desde-internet)
3. [Instalacion desde un archivo ADI.TAR.GZ](#instalacion-desde-un-archivo-binario-de-instalacion)
	1. [Instalacion desde un archivo ADI.TAR.GZ](#binario-desde-el-internet)
	
	


## Instalacion desde un archivo de descarga e instalacion
Como recordaremos un archivo .adi no es mas que un documento con sintaxis TOML
para la creación de paquetes escritos en algún lenguaje de scripting, también
para otros lenguajes, pero este es su uso recomendado. Para instalar desde un
archivo .adi puede escribir el siguiente comando:

`# apmpkg instalar foo.adi`

En seguida se iniciara la instalación de dicho paquetes, lo que hace este
proceso es descargar e instalar dependencias así como dependencias de pip o
bundle, según sea el caso, una vez realizado este paso lo siguiente que sera es
instalar los archivos especificados en este archivo y asi concluir con la
instalación.

### Desde la web

Si desea instalar desde un archivo .adi alojado en la web, puede realizar el siguiente comando:

`# apmpkg instalar -u https://foo.com/bar.adi`

Esto solamente descargara el archivo especificado y pasara a la función de
instalar como en el proceso de acá arriba


## Instalacion desde un archivo de bash y compilacion

Los archivos .ABC son los que se mas se van a ver desde el panorama desde esta
herramienta, este tipo de archivos son leídos por
[iiabc](creando_paquetes.md#abc) que habíamos acordado en [la creacion de
paquetes](creando_paquetes.md) este es un archivo similar o igual que un
[PKGBUILD](https://wiki.archlinux.org/index.php/PKGBUILD) de archlinux, pero
claro tenemos [ciertos problemas](creando_paquetes.md#complicaciones-abc) con
la compatibilidad de TODOS los PKGBUILD's existentes, pero seguiremos trabajando
para que no sea así. Para instalar desde un archivo .ABC, podemos ejecutar el
siguiente comando:

`# apmpkg instalar foo.abc`

### Abc desde Internet

De igual manera se utiliza el mismo comando:
`# apmpkg instalar -u https://foo.com/bar.abc`

## Instalacion desde un archivo binario de instalacion

Los archivos .ABI.TAR.GZ es para que se posible una instalación offline, aquí
es donde se tiene todos los archivos necesarios para una instalación y
meta datos del paquetes para que sea interpretado por apmpkg, como todo aquí con
este articulo se utiliza el mismo comando para todo, aun así por si queda duda
hay que dejarlo en claro. Para la instalación desde un archivo ABI.TAR.GZ:

`# apmpkg instalar foo.abi.tar.gz`

### Binario desde el internet

De igual manera, el comando es el mismo. ApmPKG descarga el archivo especificado
e inicia el proceso de instalación con este tipo de archivo, de manera normal,
pues no afecta si este a sido creado desde un ADI o un ABC, ApmPKG sabrá que
tipo de archivo es con el que se realizo este paquete y de igual manera
instalara los archivos, para instalar con el archivo ADI que se encuentra
alojado en algún sitio web se puede realizar con el siguiente comando:
`# apmpkg instalar -u https://foo.com/bar.abi.tar.gz`

En conclusión: 
`# apmpkg instalar foo.bar` si es una instalación local, es decir que ya tiene
en su equipo el archivo y si no: `# apmpkg instalar -u https://foo.com/bar.bar`
