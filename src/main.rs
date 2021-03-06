/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// Un administrador de paquetes universal para linux como modelo: PKGBUILD //
//                                                                         //
// Autor contribuidores:                                                   //
//                                                                         //
// kedap <dxhqezk@hi2.in>                                                  //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////

//use y modulos
use {apmpkg::core_funcions,
	colored::*};
//use std::env;
//use std::process;

fn print_banner() {
	println!(" 
	\t _______ _______ __   __ _______ ___   _ _______ 
	\t|       |       |  |_|  |       |   | | |       |
	\t|   _   |    _  |       |    _  |   |_| |    ___|
	\t|  |_|  |   |_| |       |   |_| |      _|   | __ 
	\t|       |    ___|       |    ___|     |_|   ||  |
	\t|   _   |   |   | ||_|| |   |   |    _  |   |_| |
	\t|__| |__|___|   |_|   |_|___|   |___| |_|_______|
	");
}

//fn instalar(name: &str) {
//	println!("Instalando mi pai el archivo {}", name);
//}
//
//fn dinstalar(name: &str) {
//	println!("Desinatalando el archivo {}", name);
//}
//
//fn actualizar(name: &str) {
//	println!("Actualizando {}", name);
//}

fn main(){
	print_banner();
	let info_arg = core_funcions::leer_argumentos();
	println!("{:?}", info_arg);
	if info_arg.verbose == true {
		println!("{}", "Modo verbose: Activado".blue());
	}
}