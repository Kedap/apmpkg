// Archivo con las funciones principales y necesarias

//uses
use {
	crate::estructuras::{Argumentos, AdiPaquete, PackageManager},
	std::io::{stdout, Write},
	std::{fs, process, any::type_name, process::Command},
	toml::Value,
	read_input::prelude::*,
	colored::*,
	clap::{load_yaml,App},
	curl::easy::Easy};

pub fn leer_argumentos() -> Argumentos {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	// Structura de los argumentos
	Argumentos {
		verbose: if matches.is_present("verbose") {
			true
		}
		else {
			false
		},

		instalar: if let Some(matches) = matches.subcommand_matches("instalar") {
			if matches.is_present("paquete") {
				matches.value_of("paquete").unwrap().to_string()
			} else {
				String::new()
			}
		}
		else {
			String::new()
		},

		instalar_url: if let Some(matches) = matches.subcommand_matches("instalar") {
			if matches.is_present("url") {
				matches.value_of("url").unwrap().to_string()
			} else {
				String::new()
			}
		}
		else {
			String::new()
		},

		dinstal: if let Some(matches) = matches.subcommand_matches("dinstal") {
			if matches.is_present("paquete") {
				matches.value_of("paquete").unwrap().to_string()
			} else {
				String::new()
			}
		}
		else {
			String::new()
		},

		actualizar: if let Some(matches) = matches.subcommand_matches("actualizar") {
			if matches.is_present("paquete_act") {
				matches.value_of("paquete_act").unwrap().to_string()
			} else {
				String::new()
			}
		}
		else {
			String::new()
		},

		url_act: if let Some(matches) = matches.subcommand_matches("actualizar") {
			if matches.is_present("url") {
				matches.value_of("url").unwrap().to_string()
			} else {
				String::new()
			}
		}
		else {
			String::new()
		}
	}

}

pub fn check_args(input: Argumentos) -> String {
	if input.instalar != "" {
		"instalar".to_string()
	}
	else if input.instalar_url != "" {
		"instalar_url".to_string()
	}
	else if input.dinstal != "" {
		"dinstal".to_string()
	}
	else if input.actualizar != "" {
		"actualizar".to_string()
	}
	else if input.url_act != "" {
		"url_act".to_string()
	}
	else {
		"nope".to_string()
	}
}

pub fn web_req(url: &str) {
	let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}

pub fn read_f(file: &str) -> String {
	println!("Leyendo el archivo {}...", file);
    let filedata = fs::read_to_string(file)
    	.expect("Archivo no encontrado!!! ");
    filedata
}

pub fn print_metapkg(pkg: AdiPaquete) {
	println!("
	\t\t        Paquete: {}
	\t\t           Rama: {} 
	\t\t Version actual: {}
	\t\t    Descripcion: {}
	\t\t   Dependencias: {}
	\n\n", pkg.nombre, pkg.rama, pkg.version, pkg.descrip, pkg.depen);
}
pub fn read_adi(file: &str) -> AdiPaquete {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	if !adi.contains_key("paquete") || !adi.contains_key("descarga") || !adi.contains_key("instalacion") {
		println!("Douh, eso no parece un archivo .adi");
		process::exit(0x0100);
	}
	let mut adi_f = put_adi_pack(tomy);
	adi_f.depen = pkg_depen(file);
	adi_f
}

fn put_adi_pack(adi: Value) -> AdiPaquete {
	AdiPaquete{
		nombre: adi["paquete"]["nombre"].as_str().unwrap().to_string(),
		version: adi["paquete"]["version"].as_str().unwrap().to_string(),
		rama: adi["paquete"]["rama"].as_str().unwrap().to_string(),
		descrip: adi["paquete"]["descrip"].as_str().unwrap().to_string(),
		pagina: adi["paquete"]["pagina"].as_str().unwrap().to_string(),
		licensia: adi["paquete"]["licensia"].as_str().unwrap().to_string(),
		depen: String::new(),
		conflicto: adi["paquete"]["conflicto"].as_str().unwrap().to_string(),
	}
}

pub fn pkg_depen(file: &str) -> String {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let depen = &adi["paquete"]["dependencias"].as_array().unwrap();
	let mut depen_str = String::new();
	let ultimo = depen.len();
	for i in 0..depen.len() {
		depen_str.push_str(&depen[i].as_str().unwrap());
		if i == ultimo {
			let _ = String::new();
		}
		else {
			depen_str.push_str(" ");
		}
	}
	depen_str
}

pub fn clear() {
	print!("\x1B[2J");
}

pub fn quess(texto: &str) -> bool {
	let mut aviso = String::from("[?] ");
	aviso.push_str(texto);
	aviso.push_str(" [S/n]");
	println!("{}", aviso.yellow());
	let opc: String = input().get();
	match &opc[..] {
		"S"|"s" => true,
		_ => false,
	}
}

pub fn install_depen(file_toml: &str) {
	println!("Administrando dependencias...");
	let cata = ["apt", "pacman", "dnf", "snap", "flatpak", "zypper"];
	let mut manpack = Vec::new();

	for i in 0..cata.len() {
		let comando = Command::new("bash")
                     .arg("-c")
                     .arg(cata[i])
                     .output()
                     .expect("Algo fallo en install depen");
        if comando.status.to_string() == "exit code: 1" {
        	let hi = {let tmp = cata[i];tmp.to_string()};
        	manpack.push(hi);
        }
	}

	let tomy: Value = toml::from_str(file_toml).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let depen_arr = &adi["paquete"]["dependencias"].as_array().unwrap();
	
	for i in 0..manpack.len() {
		println!("Se a dectectado {}", manpack[i]);
	}
	println!("Procediendo con la descarga e instalacion de dependencias... ");
	let mut contador = 0;
	loop {
		for i in 0..depen_arr.len() {
			let pack_less = manager(manpack[contador].to_string());
			let _instalar_comando = Command::new(pack_less.comando)
                     .arg(pack_less.intalacion)
                     .arg(depen_arr[i].as_str().unwrap())
                     .arg(pack_less.confirmacion)
                     .output()
                     .expect("Algo fallo en install depen");
		}
        let mut ready = false;


		for i in 0..depen_arr.len() {
			let check_depn = Command::new("bash")
                     .arg("-c")
                     .arg(depen_arr[i].as_str().unwrap())
                     .output()
                     .expect("Algo fallo en install depen");
            println!("Comprobando que {} se haya instalado", depen_arr[i].as_str().unwrap().to_string());
			if check_depn.status.to_string() == "exit code: 0" || check_depn.status.to_string() == "exit code: 1" {
				ready = true;
			}
			else {
				ready = false;
				println!("Algo fallo, al parecer no se encuentra en los repositorios");
			}					
		}


		if ready == true {
			println!("Se han terminado de instalar las dependencias correctamente");
			break;
		}
		else {
			contador += 1;
		}
	}
}

fn manager(pack: String) -> PackageManager {
	match &pack[..] {
		"apt" => {PackageManager {
			comando: "apt".to_string(),
        	buscar: "search".to_string(),
        	intalacion: "install".to_string(),
        	dinstalacion: "uninstall".to_string(),
        	paquete: String::new(),
        	confirmacion: "-y".to_string(),
        	root: true,
		}},
		"pacman" => {PackageManager {
			comando: "pacman".to_string(),
        	buscar: "-Ss".to_string(),
        	intalacion: "-S".to_string(),
        	dinstalacion: "-R".to_string(),
        	paquete: String::new(),
        	confirmacion: "--noconfirm".to_string(),
        	root: true,
		}},
		"dnf" => {PackageManager{
			comando: "dnf".to_string(),
        	buscar: "search".to_string(),
        	intalacion: "install".to_string(),
        	dinstalacion: "uninstall".to_string(),
        	paquete: String::new(),
        	confirmacion: "-y".to_string(),
        	root: true,
		}},
		"snap" => {PackageManager{
			comando: "snap".to_string(),
        	buscar: "find".to_string(),
        	intalacion: "install".to_string(),
        	dinstalacion: "remove".to_string(),
        	paquete: String::new(),
        	confirmacion: String::new(),
        	root: false,
		}},
		"flatpak" => {PackageManager{
			comando: "flatpak".to_string(),
        	buscar: "search".to_string(),
        	intalacion: "install".to_string(),
        	dinstalacion: "uninstall".to_string(),
        	paquete: String::new(),
        	confirmacion: String::new(),
        	root: false,
		}},
		"zypper" => {PackageManager {
			comando: "zypper".to_string(),
        	buscar: "search".to_string(),
        	intalacion: "search".to_string(),
        	dinstalacion: "remove".to_string(),
        	paquete: String::new(),
        	confirmacion: "--non-interactive".to_string(),
        	root: true,
		}},
		_ => {PackageManager {
			comando: "apmpkg".to_string(),
        	buscar: String::new(),
        	intalacion: "instalar".to_string(),
        	dinstalacion: "dinstal".to_string(),
        	paquete: String::new(),
        	confirmacion: "-v".to_string(),
        	root: true,
		}},
	}
}

/* Puede ayudar en casos de un programador que apenas se adentra en rust
Un ejemplo: yo*/
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}