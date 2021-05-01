#!/bin/bash

#Variables de cajon
NOMBRE='iiabc'
VERSION='1.0-beta'
TRUE=1
FALSE=0
ROJO='\033[91m'
VERDE='\033[92m'
AMARILLO='\033[93m'
AZUL='\033[94m'
CYAN='\033[96m'
BLANCO='\033[0m'

# Variables necesarias
src_dir=''
bool_pregunta=0
pkgdir=''


# Funciones primarias

error(){
	echo -e $ROJO"--> Error:" "$BLANCO""$@"
}

warn(){
	echo -e $AMARILLO"--> Advertencia:" "$BLANCO""$@"
}

ctrl_c(){
	error "Abortando..."
	exit 1
}
trap ctrl_c 2

banner(){
	echo -e "\t\t _______ _______ __   __ _______ ___   _ _______ "
	echo -e "\t\t|       |       |  |_|  |       |   | | |       |"
	echo -e "\t\t|   _   |    _  |       |    _  |   |_| |    ___|"
	echo -e "\t\t|  |_|  |   |_| |       |   |_| |      _|   | __ "
	echo -e "\t\t|       |    ___|       |    ___|     |_|   ||  |"
	echo -e "\t\t|   _   |   |   | ||_|| |   |   |    _  |   |_| |"
	echo -e "\t\t|__| |__|___|   |_|   |_|___|   |___| |_|_______|"
}

msg1(){
	echo -e $VERDE"-->" "$BLANCO""$@"
}

msg2(){
	echo -e $AZUL"\t-->" "$BLANCO""$@"
}

msg3(){
	echo -e $CYAN"\t\t-->" "$BLANCO""$@"
}

pregunta(){
	warn "$@"
	read opc
	if [[ "$opc" == "S" || "$opc" == "s" ]]; then
		bool_pregunta=1
	else
		bool_pregunta=0
	fi
}

install_depen(){
	msg1 "Instalando dependencias..."
	if [ "$(id -u)" != '0' ]; then
		error "Para instalar dependencias se necesita de ser root"
		sudo apmpkg -d $1
		#sudo /../apmpkg/target/debug/apmpkg -d $1
	else
		apmpkg -d $1
		#/../apmpkg/target/debug/apmpkg -d $1
	fi
}

check_depen(){
	for depen in "${depends[@]}"; do
		ls /bin/ | grep  $depen > /dev/null 2>&1
		if [ $? -eq 1 ]; then
			ls /usr/bin/ | grep $depen > /dev/null 2>&1
			if [ $? -eq 1 ]; then
				install_depen "$depen"
			else
				msg2 "$depen instalado!"
			fi
		fi
		msg2 "$depen instalado!"
	done
}

check_makedepen(){
	for depen in "${makedepends[@]}"; do
		ls /bin/ | grep  $depen > /dev/null 2>&1
		if [ $? -eq 1 ]; then
			ls /usr/bin/ | grep $depen > /dev/null 2>&1
			if [ $? -eq 1 ]; then
				install_depen "$depen"
			else
				msg2 "$depen instalado!"
			fi
		fi
		msg2 "$depen instalado!"
	done
}

descargar_fuentes_curl(){
	curl $1 -o $src_dir/source.tar.gz
}

git_clone(){
	msg2 "Clonando git..."
	IFS='+' read -ra array_src <<< "$source"
	git clone ${array_src[1]} 
	cd ..
}

check_arch(){
	if [ "$arch" == 'any' ]; then
		msg1 "Requisitos cumplidos"
	else

		for archi in "${arch[@]}"; do
			bool_arch=0

			if [ "$(uname -m)" == "$archi" ]; then
				bool_arch=1
				break
			else
				bool_arch=0
			fi
		done

		if [[ $bool_arch -eq 1 ]]; then
			msg1 "Requisitos cumplidos"
		else
			error "Requisitos no cumplidos"
			exit 1
		fi
	fi
}