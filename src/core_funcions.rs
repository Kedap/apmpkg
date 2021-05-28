// Archivo con las funciones principales y necesarias

//uses
use {
    crate::estructuras::{AdiPaquete, Argumentos, PackageManager},
    clap::{load_yaml, App},
    colored::*,
    read_input::prelude::*,
    std::{any::type_name, process::Command},
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

    // Structura de los argumentos
    Argumentos {
        verbose: if matches.is_present("verbose") {
            true
        } else {
            false
        },

        instalar: if let Some(matches) = matches.subcommand_matches("instalar") {
            if matches.is_present("paquete") {
                matches.value_of("paquete").unwrap().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        },

        confirmar: if let Some(matches) = matches.subcommand_matches("instalar") {
            if matches.is_present("confirmar") {
                true
            } else {
                false
            }
        } else {
            false
        },

        instalar_bin: if let Some(matches) = matches.subcommand_matches("instalar") {
            if matches.is_present("binario") {
                true
            } else {
                false
            }
        } else {
            false
        },

        instalar_url: if let Some(matches) = matches.subcommand_matches("instalar") {
            if matches.is_present("url") {
                matches.value_of("url").unwrap().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        },

        dinstal: if let Some(matches) = matches.subcommand_matches("remover") {
            if matches.is_present("paquete") {
                matches.value_of("paquete").unwrap().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        },

        dinstal_confi: if let Some(matches) = matches.subcommand_matches("remover") {
            if matches.is_present("confirmar") {
                true
            } else {
                false
            }
        } else {
            false
        },

        instalar_depen: if matches.is_present("instalard") {
            matches.value_of("instalard").unwrap().to_string()
        } else {
            String::new()
        },

        crear_tipo: if let Some(matches) = matches.subcommand_matches("crear") {
            if matches.is_present("tipo") {
                matches.value_of("tipo").unwrap().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        },

        crear_nombre: if let Some(matches) = matches.subcommand_matches("crear") {
            if matches.is_present("nombre") {
                matches.value_of("nombre").unwrap().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        },
    }
}

pub fn check_args(input: Argumentos) -> String {
    if input.instalar != "" {
        "instalar".to_string()
    } else if input.instalar_url != "" {
        "instalar_url".to_string()
    } else if input.dinstal != "" {
        "remover".to_string()
    } else if input.instalar_depen != "" {
        "instalar_depen".to_string()
    } else if input.crear_tipo != "" && input.crear_nombre != "" {
        "crear".to_string()
    } else {
        "nope".to_string()
    }
}

#[tokio::main]
pub async fn web_requets(url: &str, flag: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cuerpo = reqwest::get(url).await?.text().await?;
    match &flag[..] {
        "check" => println!("ok! "),
        "print" => println!("{} ", cuerpo.to_string()),
        _ => println!("nope"),
    }
    Ok(())
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
    match &opc[..] {
        "S" | "s" => true,
        _ => false,
    }
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
        ready
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
                println!("Al parecer no, porque no lo instalamos");
                ready = false;
                break;
            }
        }
        ready
    }
}

fn instalar_paquete(gestor: PackageManager, paquete: &str) -> bool {
    let comando_instalacion = Command::new(gestor.comando)
        .arg(gestor.intalacion)
        .arg(paquete)
        .arg(gestor.confirmacion)
        .output()
        .expect("Ocurrio un error cuando se instalaba las dependencias");
    if comando_instalacion.status.to_string() == "exit code: 0" {
        true
    } else {
        false
    }
}

pub fn install_depen(file_toml: &str) {
    println!("Administrando dependencias...");
    let cata = [
        "apt", "pacman", "dnf", "snap", "flatpak", "zypper", "yum", "apk",
    ];
    let mut manpack = Vec::new();

    for i in 0..cata.len() {
        let comando = Command::new("bash")
            .arg("-c")
            .arg(cata[i])
            .output()
            .expect("Algo fallo en install depen");
        if comando.status.to_string() == "exit code: 1"
            || comando.status.to_string() == "exit code: 0"
        {
            let hi = {
                let tmp = cata[i];
                tmp.to_string()
            };
            manpack.push(hi);
        }
    }

    let tomy: Value = toml::from_str(file_toml)
        .expect("Al parecer no has escrito bien el archivo ADI o no es un archivo ADI");
    let adi = tomy.as_table().unwrap();
    let depen_arr = &adi["paquete"]["dependencias"].as_array().unwrap();

    for i in 0..manpack.len() {
        println!("Se a dectectado {}", manpack[i]);
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
            if instalacion_completada == true {
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
                if instalacion_completada == true {
                    ready = true
                } else {
                    ready = false
                }
            }
        }

        if ready == true {
            println!("Se han resolvido las dependencias de manera correcta");
            break;
        } else {
            contador += 1;
        }
    }
}

fn manager(pack: String) -> PackageManager {
    match &pack[..] {
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

/* Puede ayudar en casos de un programador que apenas se adentra en rust
Un ejemplo: yo*/
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
