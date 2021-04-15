// Archivo con las funciones principales y necesarias

//uses
use {
	crate::estructuras::{Argumentos, AdiPaquete, PackageManager},
	std::{any::type_name, process::Command},
	toml::Value,
	read_input::prelude::*,
	colored::*,
	clap::{load_yaml,App}};


pub fn print_banner() {
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

		confirmar: if let Some(matches) = matches.subcommand_matches("instalar") {
			if matches.is_present("confirmar") {
				true
			} else {
				false
			}
		}
		else {
			false
		},

		instalar_bin: if let Some(matches) = matches.subcommand_matches("instalar") {
			if matches.is_present("binario") {
				true
			}
			else {
				false
			}
		}
		else {
			false
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

		dinstal: if let Some(matches) = matches.subcommand_matches("remover") {
			if matches.is_present("paquete") {
				matches.value_of("paquete").unwrap().to_string()
			} else {
				String::new()
			}
		}
		else {
			String::new()
		},

		dinstal_confi: if let Some(matches) = matches.subcommand_matches("remover") {
			if matches.is_present("confirmar") {
				true
			}
			else {
				false
			}
		}
		else {
			false
		},
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
		"remover".to_string()
	}
	else {
		"nope".to_string()
	}
}

#[tokio::main]
pub async fn web_requets(url: &str, flag: &str) -> Result<(), Box<dyn std::error::Error>> {
	let cuerpo = reqwest::get(url)
    .await?
    .text()
    .await?;
    match &flag[..] {
    	"check" => println!("ok! "),
    	"print" => println!("{} ", cuerpo.to_string()),
    	_ => println!("nope"),
    }
    Ok(())
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

pub fn local_depen(file_toml: &str) -> bool {
	let tomy: Value = toml::from_str(file_toml).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let depen_table = adi["paquete"].as_table().expect("Douh, no se un .adi");
	let mut ready = false;

	if depen_table.contains_key("cmd_depen") {
		let depen_arr = &adi["paquete"]["cmd_depen"].as_array().unwrap();
		for i in 0..depen_arr.len() {
			let check_depn = Command::new("bash")
    	                .arg("-c")
    	                .arg(depen_arr[i].as_str().unwrap())
    	                .output()
    	                .expect("Algo fallo en install depen");
    	    println!("Comprobando que {} este instalado", depen_arr[i].as_str().unwrap().to_string());
			if check_depn.status.to_string() == "exit code: 0" || check_depn.status.to_string() == "exit code: 1" {
				ready = true;
			}
			else {
				println!("Al parecer no, porque no lo instalamos");
				ready = false;
			}					
		}
		ready
	}

	else {
		let depen_arr = &adi["paquete"]["dependencias"].as_array().unwrap();
		for i in 0..depen_arr.len() {
			let check_depn = Command::new("bash")
    	                .arg("-c")
    	                .arg(depen_arr[i].as_str().unwrap())
    	                .output()
    	                .expect("Algo fallo en install depen");
    	    println!("Comprobando que {} este instalado", depen_arr[i].as_str().unwrap().to_string());
			if check_depn.status.to_string() == "exit code: 0" || check_depn.status.to_string() == "exit code: 1" {
				ready = true;
			}
			else {
				println!("Al parecer no, porque no lo instalamos");
				ready = false;
			}					
		}
		ready
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
        if comando.status.to_string() == "exit code: 1" || comando.status.to_string() == "exit code: 0" {
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


		let depen = &adi["paquete"].as_table().unwrap();
		if depen.contains_key("cmd_depen") {
			let cmd_arr = &adi["paquete"]["cmd_depen"].as_array().unwrap();
			for i in 0..cmd_arr.len() {
			let check_depn = Command::new("bash")
        	            .arg("-c")
        	            .arg(cmd_arr[i].as_str().unwrap())
        	            .output()
        	            .expect("Algo fallo en install depen");
        	println!("Comprobando que {} se haya instalado", cmd_arr[i].as_str().unwrap().to_string());
				if check_depn.status.to_string() == "exit code: 0" || check_depn.status.to_string() == "exit code: 1" {
					ready = true;
				}
				else {
					ready = false;
					println!("Algo fallo, al parecer no se encuentra en los repositorios");
				}					
			}
		}
		else {
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
        	dinstalacion: "remove".to_string(),
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

pub fn msg_end(file: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let des = adi["instalacion"].as_table().unwrap();
	if des.contains_key("mensaje") {
		println!("{}", des["mensaje"].as_str().unwrap());
	}
}

/* Puede ayudar en casos de un programador que apenas se adentra en rust
Un ejemplo: yo*/
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}