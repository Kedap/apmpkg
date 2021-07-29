//Estructuras

use {colored::*, exitcode, std::process, toml::Value};

//Enums
#[derive(Debug, Clone)]
pub enum SubComandos {
    Instalar(String),
    InstalarUrl(String),
    Remover(String),
    InstalarDependencia(String),
    Crear { tipo: String, nombre: String },
    Construir(String),
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

#[derive(Debug, Clone)]
pub enum GestoresLenguajes {
    Gem(Gem),
    Pip(Pip),
    Npm(Npm),
    Ninguno,
}

#[derive(Debug, Clone)]
pub enum Fuente {
    Git(String),
    Local(String),
    Url(String),
}

//Structs
#[derive(Clone, Debug)]
pub struct Argumentos {
    pub subcomand: SubComandos,
    pub flags: Banderas,
}

#[derive(Debug, Clone)]
pub struct MsgError {
    pub mensaje: String,
}

#[derive(Debug, Clone)]
pub struct GestorNativo {
    pub nombre: String,
    pub buscar: String,
    pub instalacion: String,
    pub confirmacion: String,
}

//Estructuras para el archivo adi

#[derive(Debug, Clone)]
pub struct Gem {
    pub gemfile: bool,
    pub file: String,
    pub gemas: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Pip {
    pub version: i64,
    pub requeriments: bool,
    pub paquetes: Vec<Value>,
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct Npm {
    pub package_json_bool: bool,
    pub package_json_ruta: String,
    pub package: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct AdiPaquete {
    pub nombre: String,
    pub version: String,
    pub rama: String,
    pub descripcion: String,
    pub pagina: String,
    pub licensia: String,
    pub dependencias: Vec<Value>,
    pub dependencias_string: String,
    pub cmd_depen: Vec<Value>,
    pub abi_dependencias: Vec<Value>,
    pub arquitectura: String,
    pub conflicto: String,
}

#[derive(Debug, Clone)]
pub struct AdiDescarga {
    pub fuente: Fuente,
    pub carpeta: String,
    pub sumasha: String,
}

#[derive(Debug, Clone)]
pub struct AdiInstalacion {
    pub fuente_opt: bool,
    pub pre_instalacion: String,
    pub fuente: Vec<Value>,
    pub destino: Vec<Value>,
    pub post_instalacion: String,
    pub mensaje: String,
}

#[derive(Debug, Clone)]
pub struct AbiDependencias {
    pub paquetes: Vec<Value>,
    pub url: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Adi {
    pub paquete: AdiPaquete,
    pub abi_dependencias: AbiDependencias,
    pub gestor: GestoresLenguajes,
    pub descarga: AdiDescarga,
    pub instalacion: AdiInstalacion,
}

//Impl para los mensajes de error
impl MsgError {
    pub fn new(mensaje: &str) -> Self {
        MsgError {
            mensaje: mensaje.to_string(),
        }
    }

    pub fn salir(&self) {
        process::exit(exitcode::DATAERR);
    }
    pub fn print(&self) {
        println!("{} {}", "Error:".red(), &self.mensaje);
    }
    pub fn print_salir(&self) {
        println!("{} {}", "Error:".red(), &self.mensaje);
        process::exit(exitcode::DATAERR);
    }
}
