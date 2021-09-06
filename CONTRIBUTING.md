# Contribuyendo a ApmPKG
Hey! Veo que quieres unirte al desarrollo de esto, pues vale, manos a la obra

Tabla de contenido
1. [Peticiones de ideas](#ideas)
2. [Reporte de bugs](#reportes-de-bugs)
3. [Flujo de colaboracion](#flujo-de-trabajo)
4. [Caja de descripcion](#caja-de-descripcion)
5. [Documentacion](#documentacion)
6. [Testers](#testers)
7. [Pasos de lanzamiento](#lanzamiento)

## Ideas
Todas las solitudes de nuevas de funciones deben de ser enviadas a [el reatreador de problemas de ApmPKG](https://github.com/Kedap/apmpkg/issues). Esto para reducir la cantidad de duplicados.

## Reportes de bugs

Todos los parches deben enviarse a github como [pull requests](https://github.com/Kedap/apmpkg/pulls) si usted deseea empezar a contribuir en ApmPKG puede empezar a atender [Buscando ayuda](https://github.com/Kedap/apmpkg/labels/help%20wanted%20%3Asos%3A), [Solicitud de funciones](https://github.com/Kedap/apmpkg/labels/enhancement%20%20%E2%AD%90%EF%B8%8F) o en [la pagina de proyectos](https://github.com/Kedap/apmpkg/projects)

## Flujo de trabajo

Primero y antes que nada debe de checar los iusses para empezar a trabajar. Si lo que usted desea es empezar a colaborar desde ahora como lo dice aqui arriba puede empezar a atender el [rastreador de problemas](https://github.com/Kedap/apmpkg/issues) tomando como prioridad a los que contiene la etqueta de [bug](https://github.com/Kedap/apmpkg/labels/bug%20%3Alady_beetle%3A). Pero esto haciendo un nuevo fork a partir del ultimo commit de la rama develop y NO desde master. De manera no obligatorio **pero se recomienda modificar la [caja de descripcion](#caja-de-descripcion)** colocando los datos correspondientes. Una ves que hayas terminado puedes hacer el pull requests pero a la rama develop en donde un tester debe de aceptar dicha mejora, si tu contribuyes a algo beneficioso es muy probable que sea aceptada
para mejor organizacion consulte [la pagina de proyectos](https://github.com/Kedap/apmpkg/projects)

## Caja de descripcion

La caja de descripcion es aquella que se ubica en la parte superior de cada archivo de codigo y debe de ser modificada cada que hace un cambio para el mejor manejo de esta no aplica en los archivos de documentacion, para ello consulte [documentacion](#documentacion), dicha caja esta comentada para que interfiera con el codigo. Una caja de descripcion luce algo asi tomando como ejemplo del archivo antes:
```
#!/bin/bash

##################################################
##                                              ##
## Comprobando conexion a github (ping) v1.0.0  ##
##                                              ##
## Autores:                                     ##
## kedap (דנטה) <kedap.dev@protonmail.com>       ##
##                                              ##
##################################################
echo "Checando conexion con github"
ping github.com -c3
echo "Finalizando el chequeo de conexion"
```
Despues:
```
#!/bin/bash

##################################################
##                                              ##
## Comprobando conexion a github (ping) v1.0.0  ##
##                                              ##
## Autores:                                     ##
## kedap (דנטה) <kedap.dev@protonmail.com>      ##
## foo (bar) <example@example>                  ##
##                                              ##
## [iusse #1]                                   ##
##                                              ##
##################################################
echo "Checando conexion con github"
ping github.com -c3

# Solucion por foo
if [ $? = 0 ];then
  echo "Tienes conexion"
else
  echo "No tienes conexion"
fi
# Termina la solucion de foo

echo "Finalizando el chequeo de conexion"
```
Ahora analizaremos la caja de descripcion no es algo dificil, En la primera linea tenemos al mitico

`!/bin/bash`

Esto forma parte del codigo asi que lo dejamos tal cual, en la siguiente linea nos encontramos a:

`Comprobando conexion a github (ping) v1.0.0`

Esto es un resumen del archivo y su version, nos enfocaremos mas en la version ya que cuando comparamos el antes y el despues nos damos cuenta que la version tampoco coinside ya que cada vez que haces una nueva mejora en el codigo este va a aumentar un 1 al ultimo, recuerda siempre aumentar ese 1 cada vez que trabajes en una nueva mejora en el codigo. En las siguientes lineas tenemos:

```
Autores:
kedap (דנטה) <kedap.dev@protonmail.com>
foo (bar) <example@example>
```

Pues como es logico en esta parte se coloca el nombre de usuario de github y si deseas en parentesis un alias, desde luego dentro de < > debes de colocar una direccion de correo electronico, esto con la finalidad de que tengamos la necesidad de contactarte referente al codigo. En la siguiente linea encontraremos:

`[iusse #1]`

Esta linea como todo creemos que es, es el iusse que esta atendiendo. Si navegamos mas en el codigo nos encontraremos con lo siguiente:

`Solucion por foo`

y

`Termina la solucion`

Esto se hace para que a nuestros [testers](#testers) sea mas practico encontrar la implementacion que agregaste

## Documentacion
El codigo debe de documentarse cuando sea apropiado. Si usted cree que realizo un cambio que necesita ser documentando o simplemente quiere extender la documentacion puede agregar dicha documentacion en el [directorio docs](doc/) e indexarla desde el README. Claro en el fork que usted creo y despues hacer el pull requests a la rama develop en donde aceptaremos tu documentacion

## Testers

Son una parte fudamental en el crecimiento de este proyecto, son aquellos que se encargan de checar todos los pulls requests, probarlos y aceptarlos, si tu quieres ser parte los testers puede mandar un mensaje a [telegram](https://t.me/Kedap_Develop) o email

## Lanzamiento

El ciclo de lanzamiento es el siguiente 
1. Iniciamos con la version 0.1-beta desde la rama master 
2. Se crea una rama de desarrollo llamada develop
3. Se solucionan algunos issues implementacion de mejoras tomando las seleccinadas para la siguiente version (consulte la [la pagina de proyectos](https://github.com/Kedap/apmpkg/projects) para mas informacion) 
4. Es posible que algunos iusses no se les tome importancia, revisaste y no se soluciona dicho iusse que quieres asi que tu decides colaborar 
5. Creas un fork a partir del ultimo commit de la rama de develop y empiezas a trabajar 
6. Terminas de trabajar en aquel issues y haces el pull requests a la rama develop y es aceptada. La rama que creaste es elminada pero implementada a la develop 
7. Ahora se terminan los detalles en la rama develop para hacer pull request a la rama master 
8. Se hace pull request a master y se afinan mas detalles
9. Se lanza la siguiente version!
