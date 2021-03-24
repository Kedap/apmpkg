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
	core_funcions, archivos},
	std::{process,time::Duration,thread, path::Path},
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
	let toml = archivos::read_fs(name);
	let meta = archivos::read_adi(&toml);
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
	let contador_bar = 9; let mut pb = ProgressBar::new(contador_bar);
	pb.format("(->.)");
	pb.inc();
	thread::sleep(Duration::from_secs(1));

	pb.inc();
	let mut dir = String::new();
	dir.push_str(&meta.nombre);
	dir.push_str(".d");

	let mut dir0 = String::new();
	dir0.push_str(&meta.nombre);
	dir0.push_str(".d");

	let mut dirg = String::new();
	dirg.push_str(&meta.nombre);
	dirg.push_str(".d");

	let exist: bool = Path::new(&dir).exists();
	if exist == true {
		let borrar = core_funcions::quess("Al parecer el directorio de trabajo ya esta creado, quiere borrarlo?");
		if borrar == true {
			println!("Borrando el directorio...");
			archivos::remove_dd(&dir);
		}
		else {
			println!("No se puede continiar a menos que se elimine dicho directorio");			
			process::exit(0x0100);
		}
	}

	let a = archivos::new_dir(&dir);
	match a {
		Ok(_a) => println!("Creacion del directorio es correcto"),
		Err(_e) => {println!("{}", "Ocurrio un error al crear el directorio".red()); process::exit(0x0100);}
	}

	let ya_install = core_funcions::local_depen(&toml);
	if ya_install == true {
		println!("Yeah, ya tienes las dependencias instaladas!!!!");
	}
	else {
		core_funcions::install_depen(&toml);
	}
	pb.inc();

	println!("{}", "iniciando la descarga de las fuentes...".green());
	let mut pack_ver = String::new();
	pack_ver.push_str(&dir);pack_ver.push_str("/");
	pack_ver.push_str(&meta.nombre); pack_ver.push_str("-");
	pack_ver.push_str(&meta.version); pack_ver.push_str(".acd.tar");
	let gito = archivos::source_git_q(&toml);
	if gito == true {
		let des = archivos::read_adi_down(&toml, gito);
		dirg.push_str("/"); dirg.push_str(&des.src);
		let source_git = archivos::read_git(&toml);
		archivos::git_clone(&source_git, &dirg);
	}
	else {
		let des = archivos::read_adi_down(&toml, gito);
		let f = archivos::download(&des.url, &pack_ver);
		match f {
			Ok(_f) => println!("Correcto"),
			Err(_e) => {println!("{}", "Ocurrio un error al hacer la peticion, intenta de nuevo".red()); process::exit(0x0100);}
		}
		println!("Se termino la descarga");
	}
	pb.inc();

	let des = archivos::read_adi_down(&toml, gito);
	println!("Verificando la integridad del archivo...");
	if des.sha256sum == "SALTAR" {
		println!("{}", "Se ha saltado la verificacion!!!".red());
	}
	else {
		let suma = archivos::hash_sum(&pack_ver, &des.sha256sum);
		if suma == true {
			println!("{}", "Verificacion correcta".green());
		}
		else {
			println!("{}", "La verificacion no coinside, vuelve intentar".red());
			process::exit(0x0100);
		}
	}
	pb.inc();

	if gito != true {
		println!("Extrayendo el tarball");
		let taa = archivos::e_tar(&pack_ver, &dir);
		match taa {
			Ok(_taa) => println!("El tarball se descomprimio con exito"),
			Err(_e) => {println!("{}", "Ocurrio un error al descomprimir el tarball".red()); process::exit(0x0100);}
		}
	}
	pb.inc();

	println!("Iniciando la instalacion de archivos de depenencias del proyecto");
	let mut src_path = dir; src_path.push_str("/"); src_path.push_str(&des.src);
	let mut src_path0 = dir0; src_path0.push_str("/"); src_path0.push_str(&des.src);
	src_path.push_str("/");
	src_path0.push_str("/");
	archivos::extern_depen(&toml, &src_path);
	pb.inc();

	println!("Iniciando instalacion");
	archivos::install_path(&toml, &src_path0);
	pb.inc();

	println!("Borrando y limpiando los archivos de compilacion y fuentes");
	let mut dirc = String::new();
	dirc.push_str(&meta.nombre);
	dirc.push_str(".d");
	archivos::remove_dd(&dirc);
	pb.inc();
	pb.finish_print("Se realizo con exito la instalacion!");
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