// Estructuras
use toml::Value;

//Enumeracion de subcomandos

#[derive(Debug, Clone)]
pub enum SubComandos {
    Instalar(String),
    InstalarUrl(String),
    Remover(String),
    InstalarDependencia(String),
    Crear([String; 2]),
    Ninguno,
}

#[derive(Debug, Clone)]
pub enum Banderas {
    ConfirmarInstalacion,
    InstalacionConBinarios,
    ConfirmarConBinarios,
    ConfirmacionRemove,
    Ninguno,
}

#[derive(Clone, Debug)]
pub struct Argumentos {
    pub subcomand: SubComandos,
    pub flags: Banderas,
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
    pub depen: String,
    pub conflicto: String,
}

#[derive(Clone, Debug)]
pub struct AdiGem {
    pub gemfile: bool,
    pub file: String,
    pub gemas: Vec<Value>,
}

#[derive(Clone, Debug)]
pub struct AdiPip {
    pub version: i64,
    pub requirements: bool,
    pub file: String,
    pub packages: Vec<Value>,
}

#[derive(Clone, Debug)]
pub struct AdiDescarga {
    pub url: String,
    pub src: String,
    pub sha256sum: String,
}

//PACKAGE MANAGERS LINUX

#[derive(Clone, Debug)]
pub struct PackageManager {
    pub comando: String,
    pub buscar: String,
    pub intalacion: String,
    pub dinstalacion: String,
    pub paquete: String,
    pub confirmacion: String,
    pub root: bool,
}
