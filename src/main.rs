/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// Un administrador de paquetes universal para linux como modelo: PKGBUILD //
//                                                                         //
// Autor contribuidores:                                                   //
//                                                                         //
// kedap <dxhqezk@hi2.in>                                                  //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////

//use 

use std::env;
use colored::*;
use std::process;


fn main(){
	let argumentos: Vec<String> = env::args().collect();
	let comando = &argumentos[1];
	let confir = comando.is_empty();
	if confir == true {
		println!("Prueba con: apmpkg help");process::exit(0x0100);
	}
	match &comando[..] {
		"help" => help_menu(),
		"version" => version_print(),
		_ => {println!("Prueba con: apmpkg help");process::exit(0x0100);}
	}
	
}

fn help_menu(){
	println!("Uso: apmpkg [Comando] <argumentos...>
	\nComandos:
	version:	Imprimir la version
	help:		Imprimir este menu");
}

fn version_print(){
	println!("{}", "apmpkg 0.1.0-beta".blue());
}