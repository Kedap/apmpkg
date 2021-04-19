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

source /etc/apmpkg/iiabc/core_fn.sh > /dev/null 2>&1
if [ $? -eq 1 ]; then
	echo "Al parecer no se encuentra el archivo /etc/apmpkg/iiabc/core_fn.sh!!!"
	exit 1
fi


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

	# Procesador
	check_arch

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

	# Sumas
	if [ "$sha256sums" == "SKIP" ]; then
		warn "Omitiendo la suma sha256"
	else
		msg1 "Verificando la integridad de los archivos..."
		echo $sha256sums $pkgname.d/source.tar.gz | sha256sum -c > /dev/null 2>&1
		if [ $? -eq 1 ]; then
			error "Las sumas no coinciden..."
			exit 1
		else
			msg1 "Las sumas coinciden!"
		fi
	fi

	# Extrayendo?
	if [ -z "$noextract" ]; then
		msg1 "Extrayendo fuentes..."
		cd $src_dir
		tar -xf source.tar.gz
	else
		warn "No se extraen las fuentes..."
	fi

	# Prepare
	msg1 "iniciando prepare()..."
	prepare > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		cd ..
	fi

	# Ejecutando las funciones...
	msg1 "Iniciando build()..."
	build > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		cd ..
	fi

	# Check
	msg1 "Iniciando package()..."
	package > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		cd ..
	fi
	
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

# Iniciando instalacion aparitir de un .abc
if [ "$1" == "-i" ]; then
	echo "Iniciando instalacion..."
	install_abi $2
fi