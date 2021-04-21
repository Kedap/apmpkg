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
	mkdir $src_dir > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		pregunta "El archivo de trabajo ya existe, desea borrarlo? [S/n]"
		if [ $bool_pregunta -eq 1 ]; then
			warn "Borrando directorio $pkgname.d..."
			rm -r $pkgname.d
			mkdir $src_dir > /dev/null 2>&1
		else
			error "Es necesario borrarlo"
			exit 1
		fi
	fi
	# Descargando
	msg1 "Obteniendo fuentes..."
	echo $source | grep git+ > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		descargar_fuentes_curl "$source"
	else
		cd $src_dir
		git_clone
	fi

	# Sumas
	if [[ "$sha256sums" == "SKIP" || -z "$sha256sums" ]]; then
		warn "Omitiendo la suma sha256"
	else
		msg1 "Verificando la integridad de los archivos..."
		echo $sha256sums $pkgname.d/source.tar.gz | sha256sum -c > /dev/null 2>&1
		if [ $? -eq 1 ]; then
			error "Las sumas no coinciden..."
			exit 1
		else
			msg1 "Las sumas coinciden!"
			if [ -z "$noextract" ]; then
				msg1 "Extrayendo fuentes..."
				cd $src_dir
				tar -xf source.tar.gz
				cd ..
			else
				warn "No se extraen las fuentes..."
			fi
		fi
	fi

	# Prepare
	LC_ALL=C type prepare > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "iniciando prepare()..."
		cd "$pkgname.d"
		prepare 
		cd ..
	fi

	# Ejecutando las funciones...
	LC_ALL=C type build > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "Iniciando build()..."
		cd "$pkgname.d"
		build 
		cd ..
	fi

	# Check
	LC_ALL=C type check > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "Iniciando check()..."
		cd "$pkgname.d"
		check
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

	# Copiando a la base de datos...
	msg1 "Ejecutando los ultimos disparadores..."
	cd ..
	cd ..
	cp $1 /etc/apmpkg/paquetes

	## Binario
	#if [ "$2" == "binario" ]; then
	#	msg1 "Creando binario..."
	#	cp $1 $pkgname.d/apkg.abi
	#	tar -czf $pkgname-$pkgver.abi.tar $pkgname.d
	#fi

	#Limpiando...
	warn "Borrando archivos fuente..."
	rm -r $src_dir
}

# Crear un binario
create_bin(){
	pwd_dd=$(pwd)
	pkgdir="$pwd_dd/pkg"
	mkdir $pkgdir
	if [ $? -eq 1 ]; then
		pregunta "El archivo de trabajo ya existe, desea borrarlo? [S/n]"
		if [ $bool_pregunta -eq 1 ]; then
			warn "Borrando directorio $pkgdir.d..."
			rm -r $pkgdir
			mkdir $pkgdir > /dev/null 2>&1
		else
			error "Es necesario borrarlo"
			exit 1
		fi
	fi
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
	mkdir $src_dir > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		pregunta "El archivo de trabajo ya existe, desea borrarlo? [S/n]"
		if [ $bool_pregunta -eq 1 ]; then
			warn "Borrando directorio $pkgname.d..."
			rm -r $pkgname.d
			mkdir $src_dir > /dev/null 2>&1
		else
			error "Es necesario borrarlo"
			exit 1
		fi
	fi

	# Descargando
	msg1 "Obteniendo fuentes..."
	echo $source | grep git+ > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		descargar_fuentes_curl "$source"
	else
		cd $src_dir
		git_clone
	fi

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
			if [ -z "$noextract" ]; then
				msg1 "Extrayendo fuentes..."
				cd $src_dir
				tar -xf source.tar.gz
				cd ..
			else
				warn "No se extraen las fuentes..."
			fi
		fi
	fi

	# Prepare
	LC_ALL=C type prepare > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "iniciando prepare()..."
		cd "$pkgname.d"
		prepare 
		cd ..
	fi

	# Ejecutando las funciones...
	LC_ALL=C type build > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "Iniciando build()..."
		cd "$pkgname.d"
		build 
		cd ..
	fi

	# Check
	LC_ALL=C type check > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "Iniciando check()..."
		cd "$pkgname.d"
		check
		cd ..
	fi

	# La instalacion se debe de instalar como root, aqui
	msg1 "Iniciando package()..."
	if [ "$(id -u)" != '0' ]; then
		warn "Entrando en el entrono fakeroot..."
		cd ..
		fakeroot -- bash -$- "${BASH_SOURCE[0]}" -f "${ARGLIST[0]}" || exit $?
		cd "$pkgname.d"
		package
	else
		package
	fi

	# Empaquetando el binario
	msg1 "Empaquetando el binario..."
	cd ..
	cd ..
	tar -czf $pkgname-$pkgver.abi.tar.gz pkg $1

	# Mensaje final
	msg1 "La creacion del binario a terminado!"
}

# Instalar desde un binario
install_bin(){
	clear
	banner
	echo
	echo

	# Root?
	if [ "$(id -u)" != '0' ]; then
		error "Necesitas ser root para ejecutar, vuelve a intentar como root"
		exit 1
	fi

	# Desempaquetando...
	msg1 "Desempaquetando el archivo $1..."
	mkdir abi_d
	if [ $? -eq 1 ]; then
		pregunta "El archivo de trabajo ya existe, desea borrarlo? [S/n]"
		if [ $bool_pregunta -eq 1 ]; then
			warn "Borrando directorio $pkgdir.d..."
			rm -r $pkgdir
			mkdir $pkgdir > /dev/null 2>&1
		else
			error "Es necesario borrarlo"
			exit 1
		fi
	fi
	cp $1 abi_d
	cd abi_d
	tar -xf $1

	#Leyendo...
	msg1 "Leyendo el archivo de instalacion..."
	source *.abc > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		source PKGBUILD > /dev/null 2>&1
		if [ $? -eq 1 ]; then
			error "Al parecer el archivo .abi esta corrupto"
			exit 1
		fi
	fi
	msg1 "Procediendo con la instalacion de el paquete $pkgname..."
	cd pkg 
	cp -r * /

	# Limpiando
	msg1 "Realizando los ultimos movimientos..."
	cd ..
	cd ..
	rm -r abi_d
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

if [ "$1" == "-ib" ]; then
	echo "Iniciando instalacion desde un binario..."
	install_bin $2 
fi

if [ "$1" == "-b" ]; then
	echo "Creando binario..."
	create_bin $2
fi