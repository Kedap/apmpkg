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
use {apmpkg::{
	core_funcions},
	std::{process, thread},
	pbr::ProgressBar,
	colored::*};

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


fn instalar(name: &str) {
	println!("Iniciando instalacion/creacion del paquete desde el archivo: {}", name);
	let toml = core_funcions::read_f(name);
	let meta = core_funcions::read_adi(&toml);
	core_funcions::clear();
	print_banner();
	core_funcions::print_metapkg(meta.clone());
	let confirm = core_funcions::quess("Deseas seguir con la instalacion?");
	if confirm == true {
		println!("Iniciando proceso de instalacion");
	}
	else {
		println!("{}", "abortando!".red());
		process::exit(0x0100);
	}
	// Progres barr
	let contador_bar = 2; let mut pb = ProgressBar::new(contador_bar);
	pb.format("(->.)");
	pb.inc();

	core_funcions::install_depen(&toml);thread::sleep_ms(1);
	let des = core_funcions::read_adi_down(&toml);thread::sleep_ms(1);
	pb.inc();
	println!("{}", "iniciando la descarga del tarball...".green());thread::sleep_ms(1);
	let mut pack_ver = String::new();thread::sleep_ms(1);
	pack_ver.push_str(&meta.nombre); pack_ver.push_str("-");thread::sleep_ms(1);
	pack_ver.push_str(&meta.version); pack_ver.push_str(".acd.tar");thread::sleep_ms(1);
	let f = core_funcions::download(&des.url, &pack_ver);thread::sleep_ms(1);
	match f {
		Ok(_f) => println!("Correcto"),
		Err(_e) => {println!("{}", "Ocurrio un error al hacer la peticion, intenta de nuevo".red()); process::exit(0x0100);}
	}
	thread::sleep_ms(1);
	println!("Se termino la descarga");
}

fn instalar_url(name: &str) {
	println!("Iniciando instalacion apartir de la URL: {}", name);
	let f = core_funcions::web_requets(name, "print");
	match f {
		Ok(_f) => println!("Analizando el archivo"),
		Err(_e) => {println!("{}", "Ocurrio un error al hacer la peticion, intenta de nuevo".red()); process::exit(0x0100);}
	}
}

fn dinstalar(name: &str) {
	println!("Desinatalando el archivo {}", name);
}

fn actualizar(name: &str) {
	println!("Actualizando {}", name);
}

fn url_act(name: &str) {
	println!("Actualizando desde la URL: {}", name);
}

fn main(){
	print_banner();
	let info_arg = core_funcions::leer_argumentos();

	// verbose?
	if info_arg.verbose == true {
		println!("{}", "Modo verbose: Activado".blue());
	}

	// Separador:
	let argu = core_funcions::check_args(info_arg.clone());
	match &argu [..] {
		"instalar" => instalar(&info_arg.instalar),
		"instalar_url" => instalar_url(&info_arg.instalar_url),
		"dinstal" => dinstalar(&info_arg.dinstal),
		"actualizar" => actualizar(&info_arg.actualizar),
		"url_act" => url_act(&info_arg.url_act),
		_ => {println!("{}", "Intenta con: apmpkg -h o apmpkg --help".green()); process::exit(0x0100);},
	}
}