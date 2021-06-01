/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// Un administrador de paquetes universal para linux como modelo: PKGBUILD //
//                                                                         //
// Autor contribuidores:                                                   //
//                                                                         //
// kedap <dxhqezk@hi2.in>                                                  //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////

//use y modulos
use {
    apmpkg::{archivos, core_funcions, metodos_de_instalacion, estructuras::*},
    colored::*,
    nix::unistd::Uid,
    std::process,
};

fn instalar(name: &str, flags: Banderas) {
    let no_user = match flags {
        Banderas::ConfirmarInstalacion | Banderas::ConfirmarConBinarios => true,
        _ => false,
    };
    let bin = match flags {
        Banderas::InstalacionConBinarios | Banderas::ConfirmarConBinarios => true,
        _ => false,
    };
    println!("{}", "Iniciando instalacion!".green());
    let abi = archivos::es_abi(name);
    if abi {
        if !Uid::effective().is_root() {
            println!(
                "{}",
                "Para instalar un binario necesitas de permisos root!".red()
            );
            process::exit(0x0100);
        }
        metodos_de_instalacion::instalar_abi(name, no_user);
    } else {
        let abc = archivos::es_abc(name);
        if abc {
            metodos_de_instalacion::instalar_abc(name, bin);
        } else {
            if !Uid::effective().is_root() {
                println!(
                    "{}",
                    "Para instalar un archivo adi necesitas de permisos root!".red()
                );
                process::exit(0x0100);
            }
            metodos_de_instalacion::instalar_adi(name, no_user, bin);
        }
    }
}

pub fn instalar_url(name: &str, flags: Banderas) {
    println!("Descargando desde la direccion {}", name);
    let f = archivos::download(name, "file.pmpf");
    match f {
        Ok(_f) => println!("La descarga se realizo con exito!"),
        Err(_e) => {
            println!(
                "{}",
                "Ocurrio un error al hacer la peticion, intenta de nuevo".red()
            );
            process::exit(0x0100);
        }
    }
    instalar("file.pmpf", flags);
    archivos::remove_df("file.pmpf");
}

fn dinstalar(name: &str, flags: Banderas) {
    let no_user = match flags {
        Banderas::ConfirmacionRemove => true,
        _ => false,
    };
    println!("Desinstalando el paquete {}", name);
    if !Uid::effective().is_root() {
        println!(
            "{}",
            "Para deinstalar un paquete necesitas de permisos root!".red()
        );
        process::exit(0x0100);
    }
    let bash_file = archivos::existe_abc(name);

    if bash_file {
        core_funcions::remove_abc(name);
    } else {
        let mut adi_file = String::new();
        adi_file.push_str("/etc/apmpkg/paquetes/");
        adi_file.push_str(name);
        adi_file.push_str(".adi");
        let toml = archivos::read_fs(&adi_file);
        let meta = archivos::read_adi(&toml);
        core_funcions::clear();
        core_funcions::print_banner();
        core_funcions::print_metapkg(meta.clone());

        if no_user {
            println!("{}", "Omitiendo la confirmacion".yellow());
        } else {
            let confirm = core_funcions::quess("Deseas seguir con la desinstalacion?");
            if confirm {
                println!("Iniciando con el proceso de desinstalacion");
            } else {
                println!("{}", "abortando!".red());
                process::exit(0x0100);
            }
        }

        println!("Removiendo los archivos...");
        archivos::dinstall_path(&toml);
        archivos::opt_remove(&toml);
        let mut file_db = String::new();
        file_db.push_str("/etc/apmpkg/paquetes/");
        file_db.push_str(&meta.nombre);
        file_db.push_str(".adi");
        archivos::remove_df(&file_db);
        println!("La desinstalacion se realizo con exito!");
    }
}

pub fn instalar_depen(depen: &str) {
    if !Uid::effective().is_root() {
        println!(
            "{}",
            "Para instalar un binario necesitas de permisos root!".red()
        );
        process::exit(0x0100);
    }
    core_funcions::clear();
    println!("Instalando el paquete {}", depen);
    let mut toml_str = String::from(
        "
		[paquete]
		dependencias = ['",
    );
    toml_str.push_str(depen);
    toml_str.push_str("']");
    core_funcions::install_depen(&toml_str);
}

pub fn crear_protipo(tipo: &str, nombre: &str) {
    // El tipo es correcto?
    if tipo == "adi" || tipo == "abc" {
        println!("Creando un archivo {} con el nombre de {}...", tipo, nombre);
    } else {
        println!(
            "{} {}",
            tipo,
            "No es un formato soportado para crear:/".red()
        );
        process::exit(0x0100);
    }

    if tipo == "adi" {
        archivos::spawn_adi(nombre);
        println!("La creacion del archivo {}.adi a sido correcta", nombre);
    } else {
        archivos::spawn_abc(nombre);
        println!("La creacion del archivo {}.abc a sido correcta", nombre);
    }
}

fn main() {
    core_funcions::print_banner();
    let info_arg = core_funcions::leer_argumentos();
    let flags = info_arg.flags;

    //Modificado para no utilizar core_funciones::checkargs
    match info_arg.subcomand {
        SubComandos::Instalar(path) => instalar(&path, flags),
        SubComandos::InstalarUrl(url) => instalar_url(&url, flags),
        SubComandos::Remover(path) => dinstalar(&path, flags),
        SubComandos::InstalarDependencia(dependencia) => instalar_depen(&dependencia),
        SubComandos::Crear(crear_arr) => crear_protipo(&crear_arr[0], &crear_arr[1]),
        _ => {
            println!("{}", "Intenta con: apmpkg -h o apmpkg --help".green());
            process::exit(0x0100);
        }
    }
}