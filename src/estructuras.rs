// Estructuras

#[derive(Clone, Debug)]

pub struct Argumentos {
	pub verbose: bool,
	pub instalar: String,
	pub instalar_url: String,
	pub dinstal: String,
	pub actualizar: String,
	pub url_act: String,
}

// ADI TOML archivo 
#[derive(Clone, Debug)]
pub struct AdiPaquete {
	pub nombre: String,
	pub version: String,
	pub rama: String,
	pub descrip: String,
	pub pagina: String,
	pub licensia: String,
	pub conflicto: String,
}

#[derive(Clone, Debug)]
pub struct AdiBundle {
	pub archivo: bool,
	pub file: String,
}

#[derive(Clone, Debug)]
pub struct AdiDescarga {
	pub url: String,
	pub sha256sum: String,
}

#[derive(Clone, Debug)]
struct AdiInstalacion {
	path: toml::value::Array,
}