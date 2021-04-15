#!/bin/bash
# Codficado en utf-8
# Este archivo debe de estar en /etc/apmpkg/iiabc.sh

################################################################
#                                                              #
# Interpretador para la Instalacion con archivos .ABC (IIABC)  #
#                                                              #
# Autor / Contribudores                                        #
#                                                              #
# kedap <dxhqezk@hi2.in>                                       #
#                                                              #
################################################################


if [ -z "$1" ]
then
	echo "Este es un Interpretador para archivos escritos en bash y puente para ApmPKG"
	exit
fi

# Leyendo argumentos

# Saber si es un archivo .abc
if [ "$1" == "-abc" ]; then
	usuario= cat $2 | grep package > /dev/null
	if [ $? == 1 ]; then
		echo "false"
	else
		echo "true"
	fi
fi