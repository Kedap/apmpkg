#!/bin/bash
# Codficado en utf-8
# Este archivo debe de estar en /etc/apmpkg/iiabc.sh

################################################################
#                                                              #
# Interpretador para la Instalacion con archivos .ABC (IIABC)  #
# v1.0-beta                                                    #
#                                                              #
# Autor / Contribudores                                        #
#                                                              #
# kedap <dxhqezk@hi2.in>                                       #
#                                                              #
################################################################

source /etc/apmpkg/iiabc/core_fn.sh


if [ -z "$1" ]
then
	echo "Este es un Interpretador para archivos escritos en bash y puente para ApmPKG"
	exit
fi


# Instalacion abi
install_abi(){
	clear
	banner
	echo
	echo
	msg1 "Leyendo el archivo $1..."
	
	# Si no se puede leer el archivo
	source $1 > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		error "Ocurrio un error al leer el archivo $1"
		exit 1
	fi

	# Dependencias...
	msg1 "Verificando dependencias para crear..."
	check_makedepen
	msg1 "Verificando dependencias..."
	check_depen 

	# Creando el espacio de trabajo y compilacion
	msg1 "Creando el paquete $pkgname..."
	pwd_dir= pwd > /dev/null 
	src_dir="$pwd_dir$pkgname.d/"
	mkdir -p $src_dir
	# Descargando
	msg1 "Obteniendo fuentes..."
	descargar_fuentes_curl "$source"
	msg1 "Extrayendo fuentes..."
	cd $src_dir
	tar -xf source.tar.gz

	# Ejecutando las funciones...
	msg1 "Iniciando build()..."
	build
	cd ..
	# La instalacion se debe de instalar como root, aqui
	msg1 "Iniciando package()..."
	if [ "$(id -u)" != '0' ]; then
		error "Para instalar dependencias se necesita de ser root"
		exit 1
	else
		package
	fi
	#Limpiando...
	warn "Borrando archivos fuente..."
	cd ..
	cd ..
	rm -r $src_dir
}

# Leyendo argumentos

# Saber si es un archivo .abc
if [ "$1" == "-abc" ]; then
	usuario= cat $2 | grep package > /dev/null
	if [ $? -eq 1 ]; then
		echo "false"
	else
		echo "true"
	fi
fi

if [ "$1" == "-i" ]; then
	echo "Iniciando instalacion..."
	install_abi $2
fi