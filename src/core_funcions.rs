// Archivo con las funciones principales y necesarias

//uses
use {
	crate::estructuras::{Argumentos, AdiPaquete},
	std::io::{stdout, Write},
	std::{fs, process},
	toml::Value,
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

pub fn read_adi(file: &str) -> AdiPaquete{
	let tomy: Value = toml::from_str(file).unwrap();
	let adi = tomy.as_table().unwrap();
	if !adi.contains_key("paquete") || !adi.contains_key("descarga") || !adi.contains_key("instalacion") {
		println!("Douh, eso no parece un archivo .adi");
		process::exit(0x0100);
	}
	let paking = put_adi_paque(tomy);
	paking
}

fn put_adi_paque(key_toml: Value) -> AdiPaquete {
	let toms = key_toml.as_table().unwrap();
	let pack = &toms["paquete"];
	AdiPaquete{
		nombre: pack["nombre"].to_string(),
		version: pack["version"].to_string(),
		rama: pack["rama"].to_string(),
		descrip: pack["descrip"].to_string(),
		pagina: pack["pagina"].to_string(),
		licensia: pack["licensia"].to_string(),
		dependencias: pack["dependencias"].to_string(),
		conflicto: pack["conflicto"].to_string(),
	}
}