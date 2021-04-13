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


fn instalar(name: &str, no_user: bool, bin: bool) {
	println!("Iniciando instalacion/creacion del paquete desde el archivo: {}", name);
	let toml = archivos::read_fs(name);
	let meta = archivos::read_adi(&toml);
	core_funcions::clear();
	print_banner();
	core_funcions::print_metapkg(meta.clone());
	if no_user == true {
		println!("{}", "Omitiendo la confirmacion...".yellow());
	}
	else {
		let confirm = core_funcions::quess("Deseas seguir con la instalacion?");

			if confirm == true {
				println!("Iniciando proceso de instalacion");
			}
			else {
				println!("{}", "abortando!".red());
				process::exit(0x0100);
			}
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

	let mut copy_install = String::new();
	copy_install.push_str(&meta.nombre);
	copy_install.push_str(".d/"); copy_install.push_str(&meta.nombre);
	copy_install.push_str(".adi");

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
	archivos::opt_src(&toml, &src_path);
	pb.inc();

	println!("Borrando y limpiando los archivos de compilacion y fuentes");
	let mut dirc = String::new();
	dirc.push_str(&meta.nombre);
	dirc.push_str(".d"); dirc.push_str("/");
	// Antes de limpiar...
	if bin == true {
		archivos::copy_dd(name, &copy_install);
		let mut nombre_bin = String::new(); nombre_bin.push_str(&meta.nombre);
		nombre_bin.push_str("-"); nombre_bin.push_str(&meta.version);
		archivos::crate_bin(&dirc, &nombre_bin, &toml);
		println!("Limpiando...");
		archivos::remove_dd(&dirc);
	}
	else {
		println!("Limpiando...");
		archivos::remove_dd(&dirc);
	}
	
	println!("Ejecutando los ultimos disparadores para la instalacion...");
	let mut pack_db = String::new(); pack_db.push_str("/etc/apmpkg/paquetes/");
	pack_db.push_str(&meta.nombre); pack_db.push_str(".adi");
	archivos::copy_dd(name,&pack_db);

	pb.inc();
	pb.finish_print("Se realizo con exito la instalacion!");
	core_funcions::msg_end(&toml);
}

fn instalar_url(name: &str, user: bool, bin_bool:bool) {
	println!("Descargando desde la direccion {}", name);
	let f = archivos::download(name, "file.adi");
	match f {
		Ok(_f) => println!("La descarga se realizo con exito!"),
		Err(_e) => {println!("{}", "Ocurrio un error al hacer la peticion, intenta de nuevo".red()); process::exit(0x0100);}
	}
	instalar("file.adi", user, bin_bool);
	archivos::remove_df("file.adi");
}

fn dinstalar(name: &str, no_user: bool) {
	println!("Desinstalando el paquete {}", name);
	let mut adi_file = String::new(); adi_file.push_str("/etc/apmpkg/paquetes/"); 
	adi_file.push_str(name); adi_file.push_str(".adi");
	let toml = archivos::read_fs(&adi_file);
	let meta = archivos::read_adi(&toml);
	core_funcions::clear();
	print_banner();
	core_funcions::print_metapkg(meta.clone());

	if no_user == true {
		println!("{}", "Omitiendo la confirmacion".yellow());
	}
	else {
		let confirm = core_funcions::quess("Deseas seguir con la desinstalacion?");
			if confirm == true {
				println!("Iniciando con el proceso de desinstalacion");
			}
			else {
				println!("{}", "abortando!".red());
				process::exit(0x0100);
			}
	}

	println!("Removiendo los archivos...");
	archivos::dinstall_path(&toml);
	archivos::opt_remove(&toml);
	let mut file_db = String::new(); file_db.push_str("/etc/apmpkg/paquetes/");
	file_db.push_str(&meta.nombre); file_db.push_str(".adi");
	archivos::remove_df(&file_db);
	println!("La desinstalacion se realizo con exito!");
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
		"instalar" => instalar(&info_arg.instalar, info_arg.confirmar, info_arg.instalar_bin),
		"instalar_url" => instalar_url(&info_arg.instalar_url, info_arg.confirmar, info_arg.instalar_bin),
		"remover" => dinstalar(&info_arg.dinstal, info_arg.dinstal_confi),
		_ => {println!("{}", "Intenta con: apmpkg -h o apmpkg --help".green()); process::exit(0x0100);},
	}
}