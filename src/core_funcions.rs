// Archivo con las funciones principales y necesarias

//uses
use {
    crate::estructuras::{AdiPaquete, Argumentos, Banderas, PackageManager, SubComandos},
    clap::{load_yaml, App},
    colored::*,
    psutil,
    read_input::prelude::*,
    std::{path::Path, process, process::Command},
    toml::Value,
};

pub fn print_banner() {
    println!(
        " 
	\t _______ _______ __   __ _______ ___   _ _______ 
	\t|       |       |  |_|  |       |   | | |       |
	\t|   _   |    _  |       |    _  |   |_| |    ___|
	\t|  |_|  |   |_| |       |   |_| |      _|   | __ 
	\t|       |    ___|       |    ___|     |_|   ||  |
	\t|   _   |   |   | ||_|| |   |   |    _  |   |_| |
	\t|__| |__|___|   |_|   |_|___|   |___| |_|_______|
	"
    );
}

pub fn leer_argumentos() -> Argumentos {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    Argumentos {
        subcomand: if let Some(matches) = matches.subcommand_matches("instalar") {
            if matches.is_present("paquete") {
                SubComandos::Instalar(matches.value_of("paquete").unwrap().to_string())
            } else if matches.is_present("url") {
                SubComandos::InstalarUrl(matches.value_of("url").unwrap().to_string())
            } else {
                SubComandos::Ninguno
            }
        } else if let Some(matches) = matches.subcommand_matches("remover") {
            if matches.is_present("paquete") {
                SubComandos::Remover(matches.value_of("paquete").unwrap().to_string())
            } else {
                SubComandos::Ninguno
            }
        } else if matches.is_present("instalard") {
            SubComandos::InstalarDependencia(matches.value_of("instalard").unwrap().to_string())
        } else if let Some(matches) = matches.subcommand_matches("crear") {
            if matches.is_present("tipo") && matches.is_present("nombre") {
                SubComandos::Crear {
                    tipo: matches.value_of("tipo").unwrap().to_string(),
                    nombre: matches.value_of("nombre").unwrap().to_string(),
                }
            } else {
                SubComandos::Ninguno
            }
        } else if let Some(matches) = matches.subcommand_matches("construir") {
            if matches.is_present("paquete") {
                SubComandos::Construir(matches.value_of("paquete").unwrap().to_string())
            } else {
                SubComandos::Ninguno
            }
        } else {
            SubComandos::Ninguno
        },

        flags: if let Some(matches) = matches.subcommand_matches("instalar") {
            if matches.is_present("confirmar") {
                if matches.is_present("binario") {
                    Banderas::ConfirmarConBinarios
                } else {
                    Banderas::ConfirmarInstalacion
                }
            } else if matches.is_present("binario") {
                if matches.is_present("confirmar") {
                    Banderas::ConfirmarConBinarios
                } else {
                    Banderas::InstalacionConBinarios
                }
            } else {
                Banderas::Ninguno
            }
        } else if let Some(matches) = matches.subcommand_matches("remover") {
            if matches.is_present("confirmar") {
                Banderas::ConfirmacionRemove
            } else {
                Banderas::Ninguno
            }
        } else {
            Banderas::Ninguno
        },
    }
}

pub fn print_metapkg(pkg: AdiPaquete) {
    println!(
        "
	\t\t        Paquete: {}
	\t\t           Rama: {} 
	\t\t Version actual: {}
	\t\t    Descripcion: {}
	\t\t   Dependencias: {}
	\n\n",
        pkg.nombre, pkg.rama, pkg.version, pkg.descrip, pkg.depen
    );
}

pub fn clear() {
    print!("\x1B[2J");
}

pub fn quess(texto: &str) -> bool {
    let mut aviso = String::from("[?] ");
    aviso.push_str(texto);
    aviso.push_str(" [S/n]");
    println!("{}", aviso.yellow());
    let opc: String = input().get();
    matches!(&opc[..], "S" | "s")
}

pub fn local_depen(file_toml: &str) -> bool {
    let tomy: Value = toml::from_str(file_toml)
        .expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();
    let depen_table = adi["paquete"].as_table().expect("Douh, no se un .adi");
    let mut ready = false;

    if depen_table.contains_key("cmd_depen") {
        let depen_arr = &adi["paquete"]["cmd_depen"].as_array().unwrap();
        for i in 0..depen_arr.len() {
            let check_depn = Command::new("bash")
                .arg("-c")
                .arg(depen_arr[i].as_str().unwrap())
                .output()
                .expect("Algo fallo en install depen");
            println!(
                "Comprobando que {} este instalado",
                depen_arr[i].as_str().unwrap().to_string()
            );
            if check_depn.status.to_string() != "exit code: 127" {
                ready = true;
            } else {
                println!("Al parecer no, porque no lo instalamos");
                ready = false;
                break;
            }
        }
    } else {
        let depen_arr = &adi["paquete"]["dependencias"].as_array().unwrap();
        for i in 0..depen_arr.len() {
            let check_depn = Command::new("bash")
                .arg("-c")
                .arg(depen_arr[i].as_str().unwrap())
                .output()
                .expect("Algo fallo en install depen");
            println!(
                "Comprobando que {} este instalado",
                depen_arr[i].as_str().unwrap().to_string()
            );
            if check_depn.status.to_string() != "exit code: 127" {
                ready = true;
            } else {
                //Comprobando que la dependencia este instalado con .adi
                let mut dependencia = String::from(depen_arr[i].as_str().unwrap());
                dependencia.push_str(".adi");
                let existe_adi = Path::new("/etc/apmpkg/paquetes")
                    .join(&dependencia)
                    .is_file();

                let mut dependencia_abc = String::from(depen_arr[i].as_str().unwrap());
                dependencia_abc.push_str(".abc");
                let existe_abc = Path::new("/etc/apmpkg/paquetes")
                    .join(&dependencia_abc)
                    .is_file();

                if existe_adi || existe_abc {
                    ready = true;
                } else {
                    println!("Al parecer no, porque no lo instalamos");
                    ready = false;
                    break;
                }
            }
        }
    }
    ready
}

fn instalar_paquete(gestor: PackageManager, paquete: &str) -> bool {
    if gestor.confirmacion.is_empty() {
        let comando_instalacion = Command::new(gestor.comando)
            .arg(gestor.intalacion)
            .arg(paquete)
            .output()
            .expect("Ocurrio un error cuando se instalaba las dependencias");
        comando_instalacion.status.to_string() == "exit code: 0"
    } else {
        let comando_instalacion = Command::new(gestor.comando)
            .arg(gestor.intalacion)
            .arg(paquete)
            .arg(gestor.confirmacion)
            .output()
            .expect("Ocurrio un error cuando se instalaba las dependencias");
        comando_instalacion.status.to_string() == "exit code: 0"
    }
}

pub fn install_depen(file_toml: &str) -> bool {
    println!("Administrando dependencias...");
    let catalogo = [
        "pkg", "apt", "pacman", "dnf", "zypper", "yum", "apk", "slackpkg", "snap", "npm", "flatpak",
    ];
    let mut manpack = Vec::new();

    for gestor in &catalogo {
        let comando = Command::new("bash")
            .arg("-c")
            .arg(gestor)
            .output()
            .expect("Algo fallo en install depen");
        if comando.status.to_string() == "exit code: 1"
            || comando.status.to_string() == "exit code: 0"
        {
            let hi = {
                let tmp = gestor;
                tmp.to_string()
            };
            manpack.push(hi);
        }
    }

    let tomy: Value = toml::from_str(file_toml)
        .expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();
    let depen_arr = &adi["paquete"]["dependencias"].as_array().unwrap();

    for gestor in &manpack {
        println!("Se a dectectado {}", gestor);
    }
    println!("Procediendo con la descarga e instalacion de dependencias... ");
    let mut contador = 0;
    loop {
        let mut ready = false;
        for i in 0..depen_arr.len() {
            let gestor = manager(manpack[contador].to_string());
            let dependencia = depen_arr[i].as_str().unwrap();
            let instalacion_completada = instalar_paquete(gestor.clone(), dependencia);
            // De igual manera como se instala se verifica que la dependencia fue instalada si este da como codigo de salida 0
            if instalacion_completada {
                println!(
                    "Se termino de instalar el paquete {} de manera correcta!",
                    depen_arr[i].as_str().unwrap()
                );
                ready = true
            } else {
                let mut child = Command::new(gestor.comando.clone())
                    .arg(gestor.buscar.clone())
                    .arg(dependencia)
                    .spawn()
                    .expect("Ocurrio un error al buscar posibles dependencias");
                let _result = child.wait().unwrap();
                println!(
                    "\nQue paquete sastiface la dependencia {}?",
                    dependencia.green()
                );
                let posible_paquete: String = input().get();
                println!("Instalando el posible paquete {}", posible_paquete);
                let instalacion_completada = instalar_paquete(gestor, &posible_paquete);
                if instalacion_completada {
                    ready = true
                } else {
                    ready = false
                }
            }
        }

        if ready {
            println!("Se han resolvido las dependencias de manera correcta");
            return true;
        } else {
            contador += 1;
            if contador >= manpack.len() {
                return false;
            }
        }
    }
}

fn manager(pack: String) -> PackageManager {
    match &pack[..] {
        "pkg" => PackageManager {
            comando: "pkg".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "uninstall".to_string(),
            paquete: String::new(),
            confirmacion: "-y".to_string(),
            root: false,
        },
        "apt" => PackageManager {
            comando: "apt".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "uninstall".to_string(),
            paquete: String::new(),
            confirmacion: "-y".to_string(),
            root: true,
        },
        "pacman" => PackageManager {
            comando: "pacman".to_string(),
            buscar: "-Ss".to_string(),
            intalacion: "-S".to_string(),
            dinstalacion: "-R".to_string(),
            paquete: String::new(),
            confirmacion: "--noconfirm".to_string(),
            root: true,
        },
        "dnf" => PackageManager {
            comando: "dnf".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "remove".to_string(),
            paquete: String::new(),
            confirmacion: "-y".to_string(),
            root: true,
        },
        "snap" => PackageManager {
            comando: "snap".to_string(),
            buscar: "find".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "remove".to_string(),
            paquete: String::new(),
            confirmacion: String::new(),
            root: false,
        },
        "flatpak" => PackageManager {
            comando: "flatpak".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "uninstall".to_string(),
            paquete: String::new(),
            confirmacion: String::new(),
            root: false,
        },
        "zypper" => PackageManager {
            comando: "zypper".to_string(),
            buscar: "search".to_string(),
            intalacion: "in".to_string(),
            dinstalacion: "remove".to_string(),
            paquete: String::new(),
            confirmacion: "--non-interactive".to_string(),
            root: true,
        },
        "yum" => PackageManager {
            comando: "yum".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "remove".to_string(),
            paquete: String::new(),
            confirmacion: "-y".to_string(),
            root: true,
        },
        "apk" => PackageManager {
            comando: "apk".to_string(),
            buscar: "search".to_string(),
            intalacion: "add".to_string(),
            dinstalacion: "delete".to_string(),
            paquete: String::new(),
            confirmacion: String::new(),
            root: true,
        },
        "npm" => PackageManager {
            comando: "npm".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "uninstall".to_string(),
            paquete: String::new(),
            confirmacion: String::new(),
            root: true,
        },
        "slackpkg" => PackageManager {
            comando: "slackpkg".to_string(),
            buscar: "search".to_string(),
            intalacion: "install".to_string(),
            dinstalacion: "remove".to_string(),
            paquete: String::new(),
            confirmacion: String::new(),
            root: true,
        },
        _ => PackageManager {
            comando: "apmpkg".to_string(),
            buscar: String::new(),
            intalacion: "instalar".to_string(),
            dinstalacion: "dinstal".to_string(),
            paquete: String::new(),
            confirmacion: "-c".to_string(),
            root: true,
        },
    }
}

pub fn msg_end(file: &str) {
    let tomy: Value = toml::from_str(file)
        .expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();
    let des = adi["instalacion"].as_table().unwrap();
    if des.contains_key("mensaje") {
        println!("{}", des["mensaje"].as_str().unwrap());
    }
}

pub fn remove_abc(path: &str) {
    let mut child = Command::new("bash")
        .arg("/etc/apmpkg/iiabc/iiabc.sh")
        .arg("-r")
        .arg(path)
        .spawn()
        .expect("Ocurrio un error");
    let _result = child.wait().unwrap();
}

pub fn binario_abc(path: &str) {
    let mut child = Command::new("bash")
        .arg("/etc/apmpkg/iiabc/iiabc.sh")
        .arg("-ib")
        .arg(path)
        .spawn()
        .expect("Algo fallo al intentar ejecutar iiabc");
    let _result = child.wait().unwrap();
}

pub fn verificar_arch(file_toml: &str) -> bool {
    let tomy: Value =
        toml::from_str(file_toml).expect("Al parcer no escribiste bien el archivo .ADI");
    let paquete = tomy["paquete"].as_table().unwrap();
    if paquete.contains_key("arch") {
        let archi = psutil::host::info().architecture().as_str().to_string();
        *paquete["arch"].as_str().unwrap() == archi
    } else {
        true
    }
}

pub fn post_install(file_toml: &str, path: &Path) {
    let tomy: Value =
        toml::from_str(file_toml).expect("Al parcer no escribiste bien el archivo .ADI");
    let instalacion = tomy["instalacion"].as_table().unwrap();
    if instalacion.contains_key("post_install") {
        println!("{}", "Ejecutando scripts de postinstalacion...".green());
        let mut comando = Command::new("bash")
            .arg(path.join(instalacion["post_install"].as_str().unwrap()))
            .spawn()
            .expect("Algo fallo al ejecutar el script de postinstalacion");
        let result = comando.wait().unwrap();
        if result.to_string() != "exit status: 0" {
            println!(
                "{}",
                "Ocurrio un error al ejecutar el script postinstalacion".red()
            );
            process::exit(0x0100);
        }
    }
}

pub fn post_install_existe(file_toml: &str) -> bool {
    let tomy: Value =
        toml::from_str(file_toml).expect("Al parcer no escribiste bien el archivo .ADI");
    let instalacion = tomy["instalacion"].as_table().unwrap();
    instalacion.contains_key("post_install")
}

pub fn pre_install(file_toml: &str, path: &Path) {
    let tomy: Value =
        toml::from_str(file_toml).expect("Al parcer no escribiste bien el archivo .ADI");
    let instalacion = tomy["instalacion"].as_table().unwrap();
    if instalacion.contains_key("pre_install") {
        println!("{}", "Ejecutando scripts de preinstalacion...".green());
        let mut comando = Command::new("bash")
            .arg(path.join(instalacion["pre_install"].as_str().unwrap()))
            .spawn()
            .expect("Algo fallo al ejecutar el script de postinstalacion");
        let result = comando.wait().unwrap();
        if result.to_string() != "exit status: 0" {
            println!(
                "{}",
                "Ocurrio un error al ejecutar el script postinstalacion".red()
            );
            process::exit(0x0100);
        }
    }
}

pub fn pre_install_existe(file_toml: &str) -> bool {
    let tomy: Value =
        toml::from_str(file_toml).expect("Al parcer no escribiste bien el archivo .ADI");
    let instalacion = tomy["instalacion"].as_table().unwrap();
    instalacion.contains_key("pre_install")
}
