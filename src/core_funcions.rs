// Archivo con las funciones principales y necesarias

//uses
use {
	crate::estructuras::{Argumentos, AdiPaquete},
	std::io::{stdout, Write},
	std::{fs, process, any::type_name},
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
	\n\n", pkg.nombre, pkg.rama, pkg.version, pkg.descrip);
}
pub fn read_adi(file: &str) -> AdiPaquete {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	if !adi.contains_key("paquete") || !adi.contains_key("descarga") || !adi.contains_key("instalacion") {
		println!("Douh, eso no parece un archivo .adi");
		process::exit(0x0100);
	}
	let adi_f = put_adi_pack(tomy);
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
		conflicto: adi["paquete"]["conflicto"].as_str().unwrap().to_string(),
	}
}

pub fn print_pkg_depen(file: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let depen = &adi["paquete"]["dependencias"].as_array().unwrap();
	let mut depen_str = String::new();
	for i in 0..depen.len() {
		depen_str.push_str(&depen[i].as_str().unwrap());
		depen_str.push_str(" ");
	}
	println!("Cargando dependencias...
	\t\t   Dependencias: {}", depen_str);
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

/* Puede ayudar en casos de un programador que apenas se adentra en rust
Un ejemplo: yo*/
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}