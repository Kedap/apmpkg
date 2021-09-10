# Guia para crear paquetes utilizando ApmPKG
En esta guia aprenderas todo lo que se debe de saber al crear paquetes para ApmPKG, de esta manera podemos extender su uso mucho mas y tener paquetes para linux

[English](./creando_paquetes_en.md)

Tabla de contenidos
1. [Contruyendo con un Archivo de Descarga e Instalacion: ADI](#adi)
	1. [Datos del paquete](#paquete)
	2. [Dependencias externas](#dependencias-adi)
	3. [Gemas de ruby](#gem)
	4. [Pip2 / pip3](#pip)
    5. [Npm](#npm)
	6. [descarga](#descarga)
	7. [instalacion](#instalacion)
2. [Compilando e instalando desde un Archivo de Bash y Compilando](#abc)
	1. [Errores con abc](#complicaciones-abc)
3. [Generar un archivo facilmente](#comando-de-creacion)
4. [FAQ's / Preguntas frecuentes](#preguntas-frecuentes)

# Adi
Su nombre del acronimo de:
**A**rchivo de
**D**escarga e
**I**nstalacion

Este metodo se creo con el proposito de poder crear aplicaciones escritas en python y/o ruby ya que ApmPKG tienes soporte para Pip y Bundle, de esta manera se busca ser mas facil y practico crear aplicaciones con estos lenguajes. Primero; como luce un archivo ADI, pues a continuacion un prototipo de este:
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
Mucha informacion, vamos por pasos, ADI tiene la sintaxis de TOML para que sea mas facil crear paquetes, de esta forma vamos a ver cada uno de las lineas:

## Paquete
El inicio de paquete son nada mas ni nada menos que los datos del paquete al cual instalar, se escribe con el incio de `[paquete]`, algo asi:
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
Un poco mas facil, ¿no? Apartir de aqui vemos cosas basicas como el nombre y la version que son strings, nada muy importante que descatar, pero vemos algo en rama. 
La variable **rama** es un string que se utiliza para diferenciar entre que es el paquete, si es una version beta, si es de la rama git o de desarrollo o si es una version estable.

Vamos a lo siguiente que es **descrip**, y **licensia**. Estos son strings donde uno se coloca una pequeña descripcion del paquete y la licensia para espeficar que tipo de licensia de paquete es.

**dependencias** y **cmd-depen**: dependencias es un array donde se colocan el nombre de los paquetes a los cuales se deben instalar, y cmd_depen es algo muy curioso, ya que para verificar que las dependencias estan instaladas se ejecuta un comando; es decir que si la dependencia es python despues de ejecutar el comando de instalacion se ejecuta `python`y si se obtiene una salida de 127 se da por hecho que esta instalado la dependencia, mas sin embargo existen paquetes que se ejecutan de diferente manera como es el ejemplo de `openssh` que se ejecuta con `ssh`o en este caso `metasploit` que se ejecuta con `msfconsole` es por ello que se creo este array. Mas sin embargo no es necesario SI TODAS las dependencias se ejecutan con el mismo nombre con el que se instala, como es el caso de `ruby`

**conflicto**: Este string debe de contener un path, si dicho path o archivo existe no se podra instalar, es por decir que evita que un paquete se instale cuando ya esta instalado con otro gestor de paquetes

**abi_dependencias** Es un array en donde se colocan las depedencias que **en el caso** de no encontrarse y/o resolverse con gestores de paquetes nativos, estas dependencias se instalaran de manera externa por ApmPKG, vease mas informacion en [dependencias_adi](#dependencias-adi)

**arch** Es una variable string que debe de colocarse en el caso de que dicho paquete sea compatible unicamente para dicha arquitectura, en el caso de que sea disponible para todas las arquitecturas esta variable no debe de colocarse, ya que si dicha variable no se coloca se da por hecho que el paquete esta construido para cualquier arquitectura

## Dependencias adi

Esta seccion se integro con la version 1.2.0, la funcion de esta seccion es la de otorgar las fuentes de dependencias que no se han podido resolver con los gestores de paquetes nativos, ya sea porque no se encuentran en el repositorio o porque estos se encuentran rotos y/o por alguna otra razon...

```toml
metasploit = "https://foo/bar/alterntiva/metasploit.abi.tar.gz"
```

Tenemos una simple linea, las variables de esta seccion llevaran por nombre la depedencia que en el caso de no cumplirse va a ser instalada mediante esta opcion, en este caso la dependencia **metasploit**, el valor de dicha variable va a hacer un string que contengan la ruta de donde descargar dicho paquete en formato .abi.tar.gz para que este sea instalado por apmpkg

## Gem
Esta seccion se creo para contener informacion referente a gemas que depende el paquete, claro gemas de ruby y que seran instalado con `bundle` o `gem` Esta seccion empieza con `[gem]`, esta seccion no es obligatoria, solo se coloca si el proyecto depende de gemas, si el caso es si, aqui un ejemplo:
```
gemfile = true
file = "Gemfile"
#gemfile = false
#gemas = ["colorized", "rails"]
```
La variable **gemfile** es un boleano (true/false) que esta se debe de colcar de forma obligatoria, este es para saber si el proyecto tiene un Gemfile para descargar las gemas con este archivo, en el caso de que el proyecto contenga un Gemfile este se debe de poner como true y si no pues como false.

**file** esta variable SOLO se coloca si el gemfile contiene true, ya que aqui en este string se espeficara la ruta del Gemfile, en este caso solo se pone "Gemfile" ya que se encuentra en la carpeta del proyecto

**gemas** este es un array donde se espficica las gemas de las cuales depende, se recomeinda que debe de ser pocas, pocas gemas como crear un Gemfile pero no tantas, esta pensado para proyectos que dependen con 2 o 3 gemas

## Pip
Al igual que ruby, python tambien tiene dependencias, estas van a ser instalados con pip por ApmPKG, de igual manera este campo no es obligatorio a menos que el proyecto lo requiera, tenemos soporte para pip2 y pip3, este se coloca con `[pip]` a continuacion lo que tiene que contener:
```
version = 3 # 2
requirements = true # false
#packages = ["requests", "pytest-shutil", "objdict"]
file = "requeriments.txt"
```
En **version** se debe de contener un numero entero y el numero de la version de python/pip con el cual se va a instalar las dependencias, en el ejemplo podemos ver que se va a instalar con pip3.

En **requirements** es un boleano donde se debe de colocar true si es que el proyecto instala sus dependencias con el archivo requeriments.txt y false si no

En **packages** es un array donde se especifican las dependencias, esta no se debe de poner si es que requeriments esta en false.

Por el contrario **file** es un string donde se especifica la ruta del archivo requeriments.txt para que apartir de aqui se instalan las dependencias con pip

## Npm
En este apartado podemos encontar un espacio para administrar de manera correcta los modulos de npm, en un archivo adi se puede observar asi:
```
#[npm]
#package_json = true / false
#ruta_package_json = "package.json"
#modulos = ["angular", "electron"]
```

En la variable **package_json** es booleano que indica si se instalaran los modulos desde un `package_json`

En la **ruta_package_json** es un string de la ruta en donde se encuentra el `package_json` dentro del proyecto

Sobre el array **modulos** en el caso de que package_json sea falso este debera contener los modulos que se deberan de instalar

## Descarga
La seccion de descarga es para ello, donde se especifica los detalles de la descarga, se inicia `[descarga]`en uno de los ejemplos:
```
url = "https://foo.com/bar.tar.gz"
#git = "https://serviciogit.com/foo/bar"
#local = "/path/local/de/las/fuentes"
carpeta = "foo-bar"
sha256sum = "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc" # SALTAR
```
La variable **url** es un string donde se dice el link de descarga del paquete, este debe estar comprimido en `tar.gz`ya que sera extraido con este formato de compresion, mas sin embargo en la versiones git se ha creado la variable **git** que se coloca en lugar de url, al colocar git se va a clonar el repositorio descrito aqui, o en su defecto si tiene una las fuentes en su equipo puede colocar la ruta de esta en la variable **local**

En **carpeta** se coloca el directorio al cual se debe de acceder una vez extraido el tar.gz o clonado el git

En **sha256sums** se debe de colcar las sumas sha256 del archivo a descargar, en el caso de que se utilice una version de git, este se debe de cambiar su valor como: `sha256sums = "SALTAR"`de esta forma se obite la verificacion por sha256

## Instalacion
Aqui se enfoca la informacion refrente a la ruta de instalacion, este se inicia con `[instalacion]` ejemplo:
```
#opt_src = true
pre_install = "pre_install.sh"
files = ["main.rb" , "config.conf", "porfile_user_default.py"]
ruta = ["/usr/bin/foo", "/etc/foo/config.conf", ".local/share/porfile_app.py"]
post_install = "post_apmpkg.sh"
mensaje = "Para poder ejecutar, prueba con 'foo'!"
```
La variable **opt_src** es un boleano que admite true o false si es que se desea que todo el directorio obtenido por git o por la descarga se copia a la carpeta /opt, un ejemplo de esto es el paquete metasploit que se instala en la carpeta opt.

**files** y **ruta** ambos son arrays que contienen ruta de archivos, files selecciona los archivos que se van a instalar y ruta la ruta donde estos van a ser instalados, el primer archivo seleccionado se va a instalar con `install -Dm 755` ya que se da por hecho que el index 0 de ambos array es un binario. en el caso de que quiera colocar un archivo dentro de `/home` debe de colocar una ruta relativa (no root) 
como en este ejemplo ".local/share/foo.bar" se instalara dentro de `/home` en cualquiera de los usuarios existentes

Encontraremos con la variable **post_install** que no es mas un variable string que almacena la ruta del script escrito en bash que se ejecutara una vez instalado el paquete en el caso de dar otra salida que no sea exit code 0, se dara por fallido

En la variable pre_install se encuentra la ruta en donde esta el script de bash, al igual que el script post_install este se dara por fallido si retorna otra salida que no sea 0

## Abc
Su nombre es el acronimo de:
**A**rchivo de
**B**ash y 
**C**ompilacion
Este tipo de archivo esta basado y/o clonado del [PKGBUILD](https://wiki.archlinux.org/index.php/PKGBUILD) de archlinux, no hace falta explicar lo que puede hacer este gran modelo de archlinux, mas sin embargo nos enfocaremos en lo que ApmPKG no puede hacer con un PKGBUILD porque aun no podemos tener compatibilidad absoluta, pero estamos trabajando para que no sea asi en un futuro, asi que le presentamos las limitaciones con relacion de abc
## Complicaciones abc
Actualmente no tenemos soporte con las siguientes variables
- groups: Actualmente no contamos con grupos para paquetes
- depends: Como vimos que para crear ADI y como se comprueban las dependencias es [ejecutandolas](#paquete), con los archivos abc se comprueban buscando el nombre de la dependencia en `/bin` y en `/usr/bin` y si no pues se da como dependencia no instalada. 
- optdepeds: Simplemente no usamos esta variable al igual que provides, conflicts, backups, options, install, changelog y todas las sumas que no sean sha256
De igual manera con la llegada de la version 1.0.1 se implemento la variable `cmd_depen ` para que tambien se pueda comprobar la dependencia si se ejecuta este comando dando como salida de 127 se da por no instalada


# Comando de creacion

En la actualizacion de la version 1.1 hemos integrado un subcomando para la creacion de un prototipo del cual sea necesario crear, del cual es el siguiente:
```
apmpkg crear <abc | adi> <Nombre del paquete a crear>
```
Asi para facilitar la creacion de un paquete

# Preguntas frecuentes
**¿PKGBUILD o archivos .abc funcionan en otro diestro que no sea archlinux?**
Si, ya que se realiza un proceso similar a `makepkg` mas sin embargo `iiabc`(interprete para la instalacion con .abc) hace un proceso similar pero muy diferente, al igual los binarios que se generan son similares a un `pkg.tar.xz` pero muy diferentes a `adi.tar.gz`

**¡Tengo problemas con dependencias! En X distribucion las dependencias se instala pero en Y no**
Este es una advertencia, es decir que se buscara una solucion lo mas rapido al lanzamiento de ApmPKG ya que existen paquetes con nombres diferentes en diferentes repositorios, un ejemplo es el paquete `openssh` que en archlinux se llama `openssh` pero en debian se llama `ssh`. Una solucion temporal seria modificar el nombre de las dependencias para cada distribucion, pero estamos trabajando para eliminar dicho problema. Pero con la version 1.0.1 se dio la solucion de poder preguntar al usuario a un paquete que no se encuentre

**¿Se requiere de permisos root?**
Para instalar paquetes se necesita ser root, mas sin embargo para crear binarios no es necesario
