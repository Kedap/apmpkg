#!/bin/bash
# Codficado en utf-8
# Este archivo debe de estar en /etc/apmpkg/iiabc.sh

################################################################
#                                                              #
# Interpretador para la Instalacion con archivos .ABC (IIABC)  #
# v1.5.1                                                       #
#                                                              #
# Autor / Contribudores                                        #
#                                                              #
# kedap <kedap.dev@protonmail.com>                             #
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

# Crear un binario
create_bin(){
	pwd_dd=$(pwd)
	pkgdir="$pwd_dd/pkg/"
	mkdir $pkgdir > /dev/null 2>&1
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
	pwd_dir=$(pwd)
	src_dir="$pwd_dir/$pkgname.d/"
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
		msg1 "Extrayendo fuentes..."
		cd $src_dir
		tar -xf source.tar.gz
		cd ..
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
	if [ -z "$noextract" ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "iniciando prepare()..."
		cd "$pkgname.d"
		prepare 
		cd $pwd_dd
	fi

	# Ejecutando las funciones...
	declare -F build > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "Iniciando build()..."
		cd "$pkgname.d"
		build 
		cd $pwd_dd
	fi

	# Check
	declare -F check > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		msg1 "Saltando a la siguiente funcion"
	else
		msg1 "Iniciando check()..."
		cd "$pkgname.d"
		check
		if [ $? -eq 1 ]; then
			error "Al parcer el test fallo..."
			exit 1
		fi
		cd $pwd_dd
	fi

	# La instalacion se debe de instalar como root, aqui
	msg1 "Iniciando package()..."
	if [ "$(id -u)" != '0' ]; then
		warn "Entrando en el entrono fakeroot..."
		cd $pwd_dd
		fakeroot -- bash -$- "${BASH_SOURCE[0]}" -f "${ARGLIST[0]}" || exit $?
		cd "$pkgname.d"
		package
	else
		cd $pwd_dd;cd "$pkgname.d"
		package
	fi

	# Empaquetando el binario
	msg1 "Empaquetando el binario..."
	cd $pwd_dd
	cp $1 apkg.abc
	tar -czf $pkgname-$pkgver.abi.tar.gz pkg apkg.abc > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		error "Algo fallo al crear el archivo binario"
		exit 1
	fi
	rm apkg.abc

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
	mkdir abi_d > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		pregunta "El archivo de trabajo ya existe, desea borrarlo? [S/n]"
		if [ $bool_pregunta -eq 1 ]; then
			warn "Borrando directorio abi_d..."
			rm -r abi_d
			mkdir abi_d > /dev/null 2>&1
		else
			error "Es necesario borrarlo"
			exit 1
		fi
	fi
	msg1 "Verificando conflictos..."
	if [ -z $conflicts ]; then
		msg1 "No hay de que preocuparnos!"
	else
		for cmd in "$conflicts[@]"; do
			$cmd > /dev/null 2>&1
			if [ $? -eq 127 ]; then
				msg1 "No presenta conflicto alguno!"
			else
				error "El comando $cmd existe, por lo cual causa conflicto\nAbortando..."
				exit 1
			fi
		done
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
	
	msg1 "Verificando dependencias..."
	check_depen 
	msg1 "Procediendo con la instalacion de el paquete $pkgname..."
	cd pkg 
	cp -r * /

	# Limpiando
	msg1 "Realizando los ultimos movimientos..."
	find * > /etc/apmpkg/paquetes/$pkgname.files
	cd ..
	cd ..
	cp $1 /etc/apmpkg/paquetes/$pkgname.abc
	rm -r abi_d
	msg1 "$pkgname se a instalado de manera correcta!"
}

# Remover paquetes
remove_pkg(){
	clear
	banner
	echo
	echo

	# root?
	if [ "$(id -u)" != '0' ]; then
		error "Necesitas ser root para ejecutar, vuelve a intentar como root"
		exit 1
	fi

	source /etc/apmpkg/paquetes/$1.abc > /dev/null 2>&1
	if [ $? -eq 1 ]; then
		error "Al parecer no tienes instalado el paquetes $1..."
		exit 1
	fi

	msg1 "Removiendo el paquetes $pkgname..."
	for f in $(cat /etc/apmpkg/paquetes/$1.files); do
		cd /
		rm "$f" > /dev/null 2>&1
	done
	rm /etc/apmpkg/paquetes/$1.abc
	rm /etc/apmpkg/paquetes/$1.files

	msg1 "La desinstalacion se a completado por completo!"
}

# Leyendo argumentos

# Saber si es un archivo .abc
if [ "$1" == "-abc" ]; then
    usuario= cat $2 | grep "package()" > /dev/null
	if [ $? -eq 1 ]; then
		echo "false"
	else
		echo "true"
	fi
fi

# Iniciando instalacion aparitir de un .abc
if [ "$1" == "-i" ]; then
	echo "Iniciando instalacion..."
	create_bin $2
	install_bin $pkgname-$pkgver.abi.tar.gz
fi

if [ "$1" == "-ib" ]; then
	echo "Iniciando instalacion desde un binario..."
	install_bin $2 
fi

if [ "$1" == "-b" ]; then
	echo "Creando binario..."
	create_bin $2
fi

if [ "$1" == "-r" ]; then
	echo "Iniciando desinstalacion..."
	remove_pkg $2
fi

if [ $1 == "-a" ]; then
	generar_adi $2
fi

if [ $1 == "-bb" ]; then
	generar_abc $2
fi
