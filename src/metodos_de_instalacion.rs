// Todos los metodos por instalacion ya sea por un .adi
// un .abc o un .adi.tar 

// use
use {
	crate::{
		core_funcions, archivos
	},
	std::{process,time::Duration,thread, path::Path},
	pbr::ProgressBar,
	colored::*
};

// Instalacion apartir de un archivo .adi
pub fn instalar_adi(name: &str, no_user: bool, bin: bool) {
	println!("Iniciando instalacion/creacion del paquete desde el archivo: {}", name);
	let toml = archivos::read_fs(name);
	let meta = archivos::read_adi(&toml);
	core_funcions::clear();
	core_funcions::print_banner();
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
	copy_install.push_str(".d/"); copy_install.push_str("apkg.adi");

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

	println!("Verificando los conflictos");
	let existe_conflicto = Path::new(&meta.conflicto.clone()).exists();
	if existe_conflicto == true {
		println!("No se puede instalar, el archivo {} entra en conflicto", &meta.conflicto.clone());
		process::exit(0x0100);
	}
	else {
		println!("No existe el conflicto");
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

// Instalacion apartir de un archivo .abi.tar
pub fn instalar_abi(path: &str, no_user: bool) {
	println!("Iniciando instalacion desde el binario: {}", path);
	println!("Desempaquetando el binario....");
	let resultado_e_tar = archivos::e_tar(path, "install.d/");
	match resultado_e_tar {
		Ok(_resultado_e_tar) => println!("El tarball se descomprimio con exito"),
		Err(_e) => {println!("{}", "Ocurrio un error al descomprimir el tarball".red()); process::exit(0x0100);}
	}

	let abi_funcion = archivos::existe_adi();
	if abi_funcion == true {
		instalar_abi_adi(no_user);
	}
	else {
		instalar_abi_abc(path);
	}
}

fn instalar_abi_adi(no_user: bool) {
	//Creacion del progress bar
	let contador_bar = 7; let mut pb = ProgressBar::new(contador_bar);
	pb.format("(->.)");
	thread::sleep(Duration::from_secs(1));

	// Leyendo metadata y confrmirmacion
	let toml = archivos::read_fs("install.d/apkg.adi");
	let meta = archivos::read_adi(&toml);
	core_funcions::clear();
	core_funcions::print_banner();
	core_funcions::print_metapkg(meta.clone());
	// Preguntando a por la confirmacion
	if no_user == true {
		println!("{}", "Omitiendo la confirmacion...".yellow());
	}
	else {
		let confirm = core_funcions::quess("Deseas seguir con la instalacion?");

			if confirm == true {
				println!("Iniciando proceso de instalacion");
			}
			else {
				println!("Limpiando...");
				archivos::remove_dd("install.d/");
				println!("{}", "abortando!".red());
				process::exit(0x0100);
			}
	}
	pb.inc();

	println!("Verificando conflictos...");
	let conflicto = Path::new(&meta.conflicto).exists();
	if conflicto == true {
		println!("{}", "Ocurrio un problema, ha ocurrido un problema con los conflictos".red());
		process::exit(0x0100);
	}
	else {
		println!("Pasando al siguiente paso...");
	}

	//Checando dependencias
	println!("Leyendo dependencias");
	let ya_install = core_funcions::local_depen(&toml);
	if ya_install == true {
		println!("Yeah, ya tienes las dependencias instaladas!!!!");
	}
	else {
		core_funcions::install_depen(&toml);
	}
	pb.inc();

	let desempacar_binario = archivos::binario_completo(&toml);
	if desempacar_binario == true {
		//Analizando el codigo extraido
		println!("Instalacion de librerias extras...");
		let es_git = archivos::source_git_q(&toml);
		let descarga_meta = archivos::read_adi_down(&toml, es_git);
		let mut src_path = String::from("install.d/"); src_path.push_str(&descarga_meta.src);
		src_path.push_str("/");	archivos::extern_depen(&toml, &src_path);
		pb.inc();
	
		//Colocando los archivos en los lugares deseados
		println!("Procediendo con la instalacion");
		archivos::install_path(&toml, &src_path);
		archivos::opt_src(&toml, &src_path);
		pb.inc();
	}
	else {
		//Analizando el codigo extraido
		println!("Instalacion de librerias extras...");
		let es_git = archivos::source_git_q(&toml);
		let descarga_meta = archivos::read_adi_down(&toml, es_git);
		let mut src_path = String::from("install.d/"); src_path.push_str(&meta.nombre.clone());
		src_path.push_str(".d/"); src_path.push_str(&descarga_meta.src); src_path.push_str("/");	archivos::extern_depen(&toml, &src_path);
		pb.inc();
	
		//Colocando los archivos en los lugares deseados
		println!("Procediendo con la instalacion");
		archivos::install_path(&toml, &src_path);
		archivos::opt_src(&toml, &src_path);
		pb.inc();
	}

	//Colocando en /etc/apmpkg/paquetes
	println!("Ejecutando los ultimos disparadores para la instalacion...");
	let mut pack_db = String::from("/etc/apmpkg/paquetes/");
	pack_db.push_str(&meta.nombre); pack_db.push_str(".adi");
	archivos::copy_dd("install.d/apkg.adi",&pack_db);
	pb.inc();
	
	//Borrando el directorio install.d
	println!("Limpiando las fuentes de instalacion");
	archivos::remove_dd("install.d/");
	pb.inc();

	//Mensaje de desarrollador
	pb.inc();
	pb.finish_print("Se realizo con exito la instalacion!");
	core_funcions::msg_end(&toml);
}

fn instalar_abi_abc(path: &str) {
	archivos::remove_ddf("install.d");
	core_funcions::binario_abc(path);							
}

// Instalacion apartir de un archivo .abc
pub fn instalar_abc(path: &str, bin: bool) {
	println!("Iniciando desde un .abc");

	if bin == true {
		let mut child = process::Command::new("bash")
										.arg("/etc/apmpkg/iiabc/iiabc.sh")
										.arg("-b")
										.arg(path)
										.spawn()
										.expect("Al parecer no tienes iiabc, algo anda mal");
		let _result = child.wait().unwrap();
	}
	else {
		let mut child = process::Command::new("bash")
										.arg("/etc/apmpkg/iiabc/iiabc.sh")
										.arg("-i")
										.arg(path)
										.spawn()
										.expect("Al parecer no tienes iiabc, algo anda mal");
		let _result = child.wait().unwrap();
	}
}