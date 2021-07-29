#!/bin/bash

#Variables de cajon
NOMBRE='iiabc'
VERSION='1.4.1'
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
                #Seleccionado para opcion de desarrollo
	else
		apmpkg -d $1
		#/../apmpkg/target/debug/apmpkg -d $1
                #Seleccionado para opcion de desarrollo
	fi
}

check_depen(){
	if [ -z $cmd_depends ]; then
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
	else
		for depen in "${cmd_depends[@]}"; do
			$depen > /dev/null 2>&1
			if [ $? -eq 127 ]; then
				install_depen "$depen"
			fi
			msg2 "$depen esta disponible!"
		done
	fi
}

check_makedepen(){
	if [ -z $cmd_depends ]; then
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
	else
		for depen in "${cmd_depends[@]}"; do
			$depen > /dev/null 2>&1
			if [ $? -eq 172 ]; then
				install_depen "$depen"
			fi
			msg2 "$depen esta disponible!"
		done
	fi
}

descargar_fuentes_curl(){
	wget -O $src_dir/source.tar.gz -q --show-progress $1
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

generar_adi(){
	nombre=$1
	if [ -z $1 ]; then
		nombre="paquete"
	fi
	echo '[paquete]' > $nombre.adi
	echo >> $nombre.adi
	echo 'nombre = "foo"' >> $nombre.adi
	echo 'version = "1.0.0"'>> $nombre.adi
	echo 'rama = "estable"'>> $nombre.adi
	echo 'descrip = "Descripcion"'>> $nombre.adi
	echo 'pagina = "https://foo.com/bar/"'>> $nombre.adi
	echo 'licensia = "LICENSE"'>> $nombre.adi
	echo 'dependencias = ["python3"]'>> $nombre.adi
	echo 'conflicto = "/opt/tool-ac/"'>> $nombre.adi
	echo >> $nombre.adi
	echo '#[gem]'>> $nombre.adi
	echo >> $nombre.adi
	echo '#gemfile = true'>> $nombre.adi
	echo '#file = "path/de/Gemfile"'>> $nombre.adi
	echo >> $nombre.adi
	echo '##[pip]'>> $nombre.adi
	echo >> $nombre.adi
	echo '##version = 3'>> $nombre.adi
	echo '##requirements = false'>> $nombre.adi
	echo '##packages = ["requests"]'>> $nombre.adi
	echo >> $nombre.adi
        echo '#[npm]'>> $nombre.adi
        echo '#package_json = true / false'>> $nombre.adi
        echo '#ruta_package_json = "package.json"'>> $nombre.adi
        echo '#modulos = ["angular", "electron"]'>> $nombre.adi
	echo >> $nombre.adi
	echo '[descarga]'>> $nombre.adi
	echo >> $nombre.adi
	echo '#git = "https://git.foo.com/bar.git"'>> $nombre.adi
	echo '#local = "/path/de/las/fuentes/de/manera/local"'>> $nombre.adi
	echo 'url = "https://foo.com/bar/foo-bar.tar.gz"'>> $nombre.adi
	echo 'carpeta = "foo-bar"'>> $nombre.adi
	echo 'sha256sum = "j9f2898934fjfj893j89j893fj89sdjdksajijsiodje9we"'>> $nombre.adi
	echo '#sha256sum = "SALTAR"'>> $nombre.adi
	echo >> $nombre.adi
	echo '[instalacion]'>> $nombre.adi
	echo >> $nombre.adi
        echo '#opt_src = true'>> $nombre.adi
        echo 'pre_install = "pre_apmpkg.sh"'>> $nombre.adi
	echo 'files = ["path/del/binario"]'>> $nombre.adi
	echo 'ruta = ["/usr/bin/foobar"]'>> $nombre.adi
        echo 'post_install = "post_apmpkg.sh"'>> $nombre.adi
	echo '#mensaje = "Eso es una prototipo!"'>> $nombre.adi
}

generar_abc(){
	nombre=$1
	if [ -z $1 ]; then
		nombre="paquete"
	fi

	echo '# Maintainer: Your Name <youremail@domain.com>' > $nombre.abc
	echo 'pkgname=NAME'>> $nombre.abc
	echo 'pkgver=VERSION'>> $nombre.abc
	echo 'pkgrel=1'>> $nombre.abc
	echo 'epoch='>> $nombre.abc
	echo 'pkgdesc=""'>> $nombre.abc
	echo 'arch=()'>> $nombre.abc
	echo 'url=""'>> $nombre.abc
	echo 'license=()'>> $nombre.abc
	echo 'groups=()'>> $nombre.abc
	echo 'depends=()'>> $nombre.abc
	echo 'makedepends=()'>> $nombre.abc
	echo 'checkdepends=()'>> $nombre.abc
	echo 'optdepends=()'>> $nombre.abc
	echo 'provides=()'>> $nombre.abc
	echo 'conflicts=()'>> $nombre.abc
	echo 'replaces=()'>> $nombre.abc
	echo 'backup=()'>> $nombre.abc
	echo 'options=()'>> $nombre.abc
	echo 'install='>> $nombre.abc
	echo 'changelog='>> $nombre.abc
	echo 'source=("$pkgname-$pkgver.tar.gz"'>> $nombre.abc
	echo '       "$pkgname-$pkgver.patch")'>> $nombre.abc
	echo 'noextract=()'>> $nombre.abc
	echo 'md5sums=()'>> $nombre.abc
	echo 'validpgpkeys=()'>> $nombre.abc
	echo >> $nombre.abc
	echo 'prepare() {'>> $nombre.abc
	echo '        cd "$pkgname-$pkgver"'>> $nombre.abc
	echo '        patch -p1 -i "$srcdir/$pkgname-$pkgver.patch"'>> $nombre.abc
	echo '}'>> $nombre.abc
	echo >> $nombre.abc
	echo 'build() {'>> $nombre.abc
	echo '        cd "$pkgname-$pkgver"'>> $nombre.abc
	echo '        ./configure --prefix=/usr'>> $nombre.abc
	echo '        make'>> $nombre.abc
	echo '}'>> $nombre.abc
	echo >> $nombre.abc
	echo 'check() {'>> $nombre.abc
	echo '        cd "$pkgname-$pkgver"'>> $nombre.abc
	echo '        make -k check'>> $nombre.abc
	echo '}'>> $nombre.abc
}
