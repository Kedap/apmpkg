// Archivo con las funciones principales y necesarias

//uses
use {
	crate::estructuras::Argumentos,
	std::io::{stdout, Write},
	std::fs,
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

pub fn read_f(file: &str) {
	println!("file contains {}", file);
    let filedata = fs::read_to_string(file)
    	.expect("Archivo no encontrado!!! ");
    println!("content of file sample data:\n{}", filedata);
}