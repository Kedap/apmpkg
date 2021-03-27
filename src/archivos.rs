// Manejador de archivos

//uses
use {crate::{
	lang_managers,
	estructuras::{AdiDescarga, AdiPaquete, AdiGem, AdiPip}},
	toml::Value,
	colored::*,
	flate2::read::GzDecoder,
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
					file: adi["pip"]["file"].as_str().expect("?").to_string(),
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
			copy_dd(&aak, &insta[i].as_str().unwrap().to_string());
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

pub fn dinstall_path(file: &str) {
	let tomy: Value = toml::from_str(file).expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
	let adi = tomy.as_table().unwrap();

	let remove = &adi["instalacion"]["ruta"].as_array().unwrap();

	for i in 0..remove.len() {
		//println!("Borrando el archivo {}", remove[i]);
		remove_df(&remove[i].as_str().unwrap().to_string());
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
			remove_dd(&carpeta);
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