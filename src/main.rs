/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// Un administrador de paquetes universal para linux como modelo: PKGBUILD //
//                                                                         //
// Autor contribuidores:                                                   //
//                                                                         //
// kedap <kedap.dev@protonmail.com>                                        //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////

//use y modulos
use {
    apmpkg::{archivos, core_funcions, estructuras::*, metodos_de_instalacion},
    colored::*,
    nix::unistd::Uid,
    pbr::ProgressBar,
    std::process,
};

fn instalar(ruta_archivo: &str, banderas: Banderas) {
    //Colocando valores de confirmacion o si se crea un binario
    let confirmacion = matches!(
        banderas,
        Banderas::ConfirmarInstalacion | Banderas::ConfirmarConBinarios
    );
    let binario = matches!(
        banderas,
        Banderas::InstalacionConBinarios | Banderas::ConfirmarConBinarios
    );

    let abi = archivos::es_abi(ruta_archivo);

    //Si es abi
    if abi {
        if !Uid::effective().is_root() {
            let error = MsgError::new("Para instalar un archivo abi necesitas permisos root!");
            error.print_salir();
        }
        metodos_de_instalacion::instalar_abi(&ruta_archivo, confirmacion);
    } else {
        //Si es que existe de otra manera que el abc o adi
        let abc = archivos::es_abc(ruta_archivo);
        if abc {
            metodos_de_instalacion::instalar_abc(ruta_archivo, binario);
        } else {
            if !Uid::effective().is_root() {
                let error = MsgError::new("Para instalar un archivo adi necesitar permisos root");
                error.print_salir();
            }
            metodos_de_instalacion::instalar_adi(ruta_archivo, confirmacion, binario);
        }
    }
}

fn instalar_url(url: &str, banderas: Banderas) {
    let mut barrita = ProgressBar::new(1);
    barrita.format("(->.)");
    barrita.message("Descargando... ");
    barrita.inc();
    let descarga = archivos::descarga(&url, "file.ada");
    match descarga {
        Ok(_v) => barrita.finish_print(" "),
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }
    instalar("file.ada", banderas);
    archivos::borrar_archivo("file.ada");
}

pub fn remover(nombre: &str, banderas: Banderas) {
    let confirmacion = matches!(banderas, Banderas::ConfirmacionRemove);
    if !Uid::effective().is_root() {
        let error = MsgError::new("Para remover un paquete necesitas permisos root");
        error.print_salir();
    }

    let mut abc = String::from(nombre);
    abc.push_str(".abc");
    if std::path::Path::new("/etc/apmpkg/paquetes")
        .join(abc.clone())
        .exists()
    {
        core_funcions::remover_abc(
            std::path::Path::new("/etc/apmpkg/paquetes")
                .join(abc)
                .to_str()
                .unwrap(),
        );
    } else {
        metodos_de_instalacion::remover_adi(nombre, confirmacion);
    }
}

fn instalar_depen(dependencia: &str) {
    if !Uid::effective().is_root() {
        let error = MsgError::new("Para instalar dependencias necesitas de permisos root!");
        error.print_salir();
    }
    let mut dependencia_vector: Vec<String> = Vec::new();
    dependencia_vector[0] = dependencia.to_string();
    if !core_funcions::instalar_dependencia_vector(dependencia_vector) {
        let error =
            MsgError::new("Al parecer no se puedo resolver la dependencia de manera correcta");
        error.print_salir();
    }
}

fn crear(tipo: &str, nombre: &str) {
    if tipo == "adi" || tipo == "abc" {
        println!("Creando un archivo {} con el nombre de {}...", tipo, nombre);
    } else {
        let error = MsgError::new("Tu archivo no es un formato soportado para crear");
        error.print_salir()
    }

    if tipo == "adi" {
        std::process::Command::new("bash")
            .arg("/etc/apmpkg/iiabc/iiabc.sh")
            .arg("-a")
            .arg(nombre)
            .spawn()
            .expect("Ocurrio un error al crear el archivo adi");
        println!("La creacion del archivo {}.adi a sido correcta", nombre);
    } else {
        std::process::Command::new("bash")
            .arg("/etc/apmpkg/iiabc/iiabc.sh")
            .arg("-bb")
            .arg(nombre)
            .spawn()
            .expect("Algo fallo al crear el archivo .abc");
        println!("La creacion del archivo {}.abc a sido correcta", nombre);
    }
}

fn constuir(ruta: &str) {
    if archivos::es_abc(ruta) {
        let mut child = std::process::Command::new("bash")
            .arg("/etc/apmpkg/iiabc/iiabc.sh")
            .arg("-b")
            .arg(ruta)
            .spawn()
            .expect("Algo fallo al intentar ejecutar iiabc");
        let _result = child.wait().unwrap();
    } else {
        metodos_de_instalacion::construir_binario_adi(ruta);
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
        SubComandos::Remover(path) => remover(&path, flags),
        SubComandos::InstalarDependencia(dependencia) => instalar_depen(&dependencia),
        SubComandos::Crear { tipo, nombre } => crear(&tipo, &nombre),
        SubComandos::Construir(path) => constuir(&path),
        _ => {
            println!("{}", "Intenta con: apmpkg -h o apmpkg --help".green());
            process::exit(0x0100);
        }
    }
}
