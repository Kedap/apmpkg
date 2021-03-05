// Archivo con las funciones principales y necesarias

//uses
use clap::App;

pub fn leer_argumentos() {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	if let Some(matches) = matches.subcommand_matches("instalar") {
		if matches.is_present("url") {
			let link = matches.value_of("url").unwrap();
			println!("Descargando desde la URL: {}", link);
		}
		else {
			let pack = matches.value_of("paquete").unwrap();
			println!("Instalando {}...",pack);
		}
	}

	if let Some(matches) = matches.subcommand_matches("dinstal") {
		if matches.is_present("paquete") {
			let pack = matches.value_of("paquete").unwrap();
			println!("Desinstalando {}...", pack);
		}
	}
}