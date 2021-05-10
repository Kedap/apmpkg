// Manejador de archivos

//uses
use {crate::{
	lang_managers,
	estructuras::{AdiDescarga, AdiPaquete, AdiGem, AdiPip}},
	toml::Value,
	colored::*,
	flate2::{read::GzDecoder,Compression,write::GzEncoder},
	tar::Archive,
	sha2::{Sha256, Digest},
	std::{fs, process, fs::File, io, process::Command}};

#[tokio::main]
pub async fn download(url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
	let cuerpo = reqwest::get(url)
    .await?
    .bytes()
    .await?;
	let f = write_f(name, &cuerpo);
    match f {
		Ok(_f) => println!("Correcto"),
		Err(_e) => {println!("{}", "Ocurrio un error al hacer la peticion, intenta de nuevo".red()); process::exit(0x0100);}
	}
	Ok(())
}

pub fn read_fs(file: &str) -> String {
	println!("Leyendo el archivo {}...", file);
    let filedata = fs::read_to_string(file)
    	.expect("Archivo no encontrado!!! ");
    filedata
}

pub fn write_f(name: &str ,file: &[u8]) -> io::Result<()>{
	let mut salida = File::create(name).expect("Algo fallo al crear el archivo");
	let mut conte = file;
	io::copy(&mut conte, &mut salida)?;
	println!("El archivo {} fue creado correctamente", name);
	Ok(())
}

pub fn read_adi_down(file: &str, gito: bool) -> AdiDescarga {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	if !adi.contains_key("paquete") || !adi.contains_key("descarga") || !adi.contains_key("instalacion") {
		println!("Douh, eso no parece un archivo .adi");
		process::exit(0x0100);
	}
	if gito == true {
		let source = {
			AdiDescarga{
				url: String::new(),
				src: tomy["descarga"]["carpeta"].as_str().unwrap().to_string(),
				sha256sum: tomy["descarga"]["sha256sum"].as_str().unwrap().to_string()
			}
		};
		source
	}
	else {
		let source = {
			AdiDescarga{
				url: tomy["descarga"]["url"].as_str().unwrap().to_string(),
				src: tomy["descarga"]["carpeta"].as_str().unwrap().to_string(),
				sha256sum: tomy["descarga"]["sha256sum"].as_str().unwrap().to_string()
			}
		};
		source
	}
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

pub fn extern_depen(file: &str, path_src: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();
    let null_arr = adi["paquete"]["dependencias"].as_array().unwrap();

    // Probando con gem/bundle o pip
    if adi.contains_key("gem") {
    	println!("{}", "Se han dectectados gemas de ruby!".yellow());
    	let gemm = adi["gem"]["gemfile"].as_bool().expect("Eso no es un booleano");
    	if gemm == true {
    		let payload = {
    			AdiGem{
    				gemfile: gemm,
					file: adi["gem"]["file"].as_str().expect("wtf?").to_string(),
					gemas: null_arr.to_vec(),
    			}
    		};
    		println!("Instalando desde un Gemfile");
    		lang_managers::analized_gem(payload, path_src);
    	}
    	else {
    		let gemas = adi["gem"]["gemas"].as_array().expect("Debe de ser un array!");
    		let payload = {
    			AdiGem{
    				gemfile: false,
    				file: String::new(),
    				gemas: gemas.to_vec(),
    			}
    		};
    		println!("Instalando gemas...");
    		lang_managers::analized_gem(payload, path_src);
    	}
    }
    else if adi.contains_key("pip") {
    	println!("{}", "Se ha dectectado paquetes python por instalar!".yellow());
    	let version = adi["pip"]["version"].as_integer().expect("Eso no es un numero");
    	let null_arr = adi["paquete"]["dependencias"].as_array().unwrap();

    	match version {
    		2 => println!("Instalando con pip2"),
    		3 => println!("Instalando con pip3"),
    		_ => {println!("{}", "Douh, esa version no la conosco, beep boop".red());process::exit(0x0100);},
    	}
    	let archivo = adi["pip"]["requirements"].as_bool().expect("Cuantico?");

    	if archivo == true {
    		println!("Instalando desde un archivo requirements.txt");
    		let payload = {
    			AdiPip{
    				version: version,
					requirements: archivo,
					file: adi["pip"]["file"].as_str().expect("?").to_string(),
					packages: null_arr.to_vec(),
    			}
    		};
    		lang_managers::analized_pip(payload, path_src);
    	}
    	else {
    		println!("Instalando packages de python");
    		let pack = adi["pip"]["packages"].as_array().expect("Eso no es un array");
    		println!("Instalando paquetes de python");
    		let payload = {
    			AdiPip{
    				version: version,
					requirements: archivo,
					file: String::new(),
					packages: pack.to_vec(),
    			}
    		};
    		lang_managers::analized_pip(payload, path_src);
    	}
    }
    else {
    	println!("{}", "Al parecer no hay archivos para pip o bundle/gem. Yeah, si nada que hacer aqui".green());
    }
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

pub fn new_dir(name: &str) -> std::io::Result<()> {
	fs::create_dir(name)?;
    Ok(())
}

pub fn e_tar(path: &str, target: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target)?;

    Ok(())
}

pub fn move_dd(source: &str, target: &str) {
	let mut aa = Command::new("mv")
				.arg(source)
				.arg(target)
				.spawn()
				.expect("Cielos, ocurrio un error al mover los archivos");
	let _result = aa.wait().unwrap();
}

pub fn copy_dd(source: &str, target: &str) {
	let mut child = Command::new("cp")
								.arg(source)
								.arg(target)
								.spawn()
								.expect("WTF porque no tienes cp?");
	let _result = child.wait().unwrap();
}

pub fn hash_sum(path: &str, check: &str) -> bool {
	let mut file = File::open(path).expect("Ocurrio un error al abrir el archivo");
	let mut suma = Sha256::new();
	io::copy(&mut file, &mut suma).expect("Error al copiar");
	let fhash = format!("{:x}", suma.finalize());

	if fhash == check {
		true
	}
	else {
		false
	}
}

pub fn copy_df(source: &str, target: &str) {
	let mut child = Command::new("cp")
								.arg("-r")
								.arg(source)
								.arg(target)
								.spawn()
								.expect("Algo a fallado al copiar los directorios");
	let _result = child.wait().unwrap();
}

pub fn install_path(file: &str, root_src: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();

	let select = &adi["instalacion"]["files"].as_array().unwrap();
	let insta = &adi["instalacion"]["ruta"].as_array().unwrap();
	println!("Iniciando instalacion");

	for i in 0..select.len() {
		let mut aak = String::new(); aak.push_str(root_src);
		aak.push_str(&select[i].as_str().unwrap().to_string());
		if i == 0 {
			Command::new("install")
								.arg("-Dm")
								.arg("755")
								.arg(aak)
								.arg(&insta[i].as_str().unwrap())
								.spawn()
								.expect("Install?");
		}
		else {
			let mut child = Command::new("rsync")
										.arg("-a")
										.arg(&aak)
										.arg(&insta[i].as_str().unwrap().to_string())
										.spawn()
										.expect("Ocurrio un error al instalar paquetes");
			let _result = child.wait().unwrap();
		}
	}
}

pub fn remove_df(path: &str) {
	let mut child = Command::new("rm")
								.arg(path)
								.spawn()
								.expect("Algo raro sucedio ejecutando rm");
	let _result = child.wait().unwrap();
}

pub fn remove_ddf(path: &str) {
	Command::new("rm")
			.arg("-r")
			.arg(path)
			.output()
			.expect("Algo muy raro sucedio con RM -R");
}

pub fn dinstall_path(file: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();

	let remove = &adi["instalacion"]["ruta"].as_array().unwrap();

	for i in 0..remove.len() {
		remove_ddf(&remove[i].as_str().unwrap().to_string());
	}
}

pub fn opt_remove(file:&str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let insta = adi["instalacion"].as_table().unwrap();

	if insta.contains_key("opt_src") {
		let si = insta["opt_src"].as_bool().unwrap();
		if si == true {
			let carpeta = adi["descarga"]["carpeta"].as_str().unwrap().to_string();
			let mut opt_src = String::new(); opt_src.push_str("/opt/") ;opt_src.push_str(&carpeta);
			remove_ddf(&opt_src);
		}
		else {
			let _h = true;
		}
	}
}

pub fn source_git_q(file: &str) -> bool {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let fuente = &adi["descarga"].as_table().unwrap();
	if fuente.contains_key("git") {
		true
	}
	else {
		false
	}
}

pub fn read_git(file: &str) -> String {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	adi["descarga"]["git"].as_str().unwrap().to_string()
}

pub fn git_clone(url_git: &str, target: &str) {
	let mut child = Command::new("git")
								.arg("clone")
								.arg(url_git)
								.arg(target)
								.spawn()
								.expect("No tenis git?");
	let _result = child.wait().unwrap();
}

pub fn remove_dd(dir: &str) {
	fs::remove_dir_all(dir).expect("Ocurrio un error al borrar el archivo");
}

pub fn opt_src(file:&str, dir: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();
	let insta = adi["instalacion"].as_table().unwrap();
	if insta.contains_key("opt_src") {
		let si = insta["opt_src"].as_bool().unwrap();
		if si == true {
			move_dd(dir, "/opt/");
		}
		else {
			let _h = true;
		}
	}
}

// Funcion para crear un binario apartir de un .ADI
pub fn crate_bin(path: &str, nombre:&str, meta_file: &str) {
	println!("Iniciando la creacion de un Archivos Binario de Instalacion...");
	let tomy: Value = toml::from_str(meta_file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();

	let conservar_src_dir = binario_completo(meta_file);

    if conservar_src_dir == true {
    	let mut noombre = String::new(); noombre.push_str(nombre); noombre.push_str(".abi.tar.gz");
		let tar_gz = File::create(noombre).expect("Algo fallo al crear el tar_gz");
    	let enc = GzEncoder::new(tar_gz, Compression::default());
    	let mut tar = tar::Builder::new(enc);
    	tar.append_dir_all(".", path).expect("Fallo en dir_all");
    }
    else {
    	let mut noombre = String::new(); noombre.push_str(nombre); noombre.push_str(".abi.tar.gz");
		let tar_gz = File::create(noombre).expect("Algo fallo al crear el tar_gz");
    	let enc = GzEncoder::new(tar_gz, Compression::default());
    	let mut tar = tar::Builder::new(enc);

    	let des = read_adi_down(meta_file, false);
    	let mut dirc = String::new(); dirc.push_str(path);
    	dirc.push_str(&des.src); dirc.push_str("/");

    	let archivos = &adi["instalacion"]["files"].as_array().unwrap();
    	for i in 0..archivos.len() {
    		let mut archivo = String::new(); archivo.push_str(&dirc);
			archivo.push_str(&archivos[i].as_str().unwrap().to_string());
			tar.append_path(archivo).unwrap();
    	}

    	let out_adi = String::from("apkg.adi");
    	let f = write_f(&out_adi, meta_file.as_bytes());
    	match f {
			Ok(_f) => println!("Es archivo .adi se copio con exito"),
			Err(_e) => {println!("{}", "Ocurrio un error al copiar el archivo .adi al binario".red()); process::exit(0x0100);}
		}
		tar.append_path(&out_adi).unwrap();

		println!("{}", "Limpiando...".yellow());
		remove_df(&out_adi);
    }

    println!("Creacion del binario a sido de manera exitosa!!!");
}

pub fn binario_completo(toml_file: &str) -> bool {
	let tomy: Value = toml::from_str(toml_file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();
    let mut conservar_src_dir = false;
    let insta = adi["instalacion"].as_table().unwrap();
    let gito = source_git_q(&toml_file);

    if adi.contains_key("gem") || adi.contains_key("pip") {
    	conservar_src_dir = true;
    }
    else if insta.contains_key("opt_src") {
    	let boleano = insta["opt_src"].as_bool().unwrap();
    	if boleano == true {
    		conservar_src_dir = true;
    	}
    	else {
    		conservar_src_dir = false;
    	}
    }
    else if gito == true {
    	conservar_src_dir = true;
    }
    conservar_src_dir
}

pub fn es_abi(path: &str) -> bool {
	let comando_file = Command::new("file")
							.arg("-i")
							.arg(path)
							.output()
							.expect("Ocurrio un error al ejecutar el comando file");
	let comando_salida = String::from_utf8_lossy(&comando_file.stdout);

	// Tipo de salidas segun los soportados 
	let mut adi_file = String::new(); adi_file.push_str(path); adi_file.push_str(": text/plain; charset=us-ascii\n");
	let mut abi_file = String::new(); abi_file.push_str(path); 
	abi_file.push_str(": application/gzip; charset=binary\n");

	if comando_salida == abi_file {
		println!("El archivo es soportado!!!");
		true
	}
	else if comando_salida == adi_file {
		println!("El archivo es soportado!!!");
		false
	}
	else {
		println!("{}", "El archivo no es soportado, prueba con otro".red());
		process::exit(0x0100);
	}
}

pub fn es_abc(path: &str) -> bool {
	let comando_iiabc = Command::new("bash")
									.arg("/etc/apmpkg/iiabc/iiabc.sh")
									.arg("-abc")
									.arg(path)
									.output()
									.expect("Exite el archivo /etc/apmpkg/iiabc.sh");
	let comando_salida = String::from_utf8_lossy(&comando_iiabc.stdout);

	// Tipo de salidas segun si es abc
	let abc_salida = String::from("true\n");
	if comando_salida == abc_salida {
		true
	}
	else {
		false
	}
}

pub fn existe_abc(path: &str) -> bool {
	let mut db_path = String::from("/etc/apmpkg/paquetes/");
	db_path.push_str(path); db_path.push_str(".abc");
	let cat_file = Command::new("cat")
								.arg(db_path)
								.output()
								.expect("Ocurrio algo con cat");
	if cat_file.status.to_string() == "exit code: 1" {
		false
	}
	else {
		true
	}
}

pub fn existe_adi() -> bool {
	let resultado_cat = Command::new("cat")
									.arg("install.d/apkg.adi")
									.output()
									.expect("Ocurrio un error al ejecutar cat");

	if resultado_cat.status.to_string() == "exit code: 1" {
		false
	}
	else {
		true
	}
}