//Archvo con las funciones principales y necesarias

//uses
use {
    crate::{archivos, estructuras::*},
    clap::{load_yaml, App},
    colored::*,
    read_input::prelude::*,
    std::{env, path::Path, process::Command},
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

pub fn pregunta(texto: &str) -> bool {
    let mut aviso = String::from("[?] ");
    aviso.push_str(texto);
    aviso.push_str(" [S/n]");
    println!("{}", aviso.yellow());
    let opc: String = input().get();
    matches!(&opc[..], "S" | "s")
}

pub fn verificar_arch(datos: AdiPaquete) -> bool {
    if datos.arquitectura == "any" {
        true
    } else {
        let arquitectura = psutil::host::info().architecture().as_str().to_string();
        datos.arquitectura == arquitectura
    }
}

pub fn dependencias_instaladas(adi_paquete: AdiPaquete) -> bool {
    let mut instalado = false;

    if !adi_paquete.cmd_depen.is_empty() {
        let dependencia_array = adi_paquete.cmd_depen;
        for dependencia in dependencia_array {
            let verificar_dependencia = Command::new("bash")
                .arg("-c")
                .arg(dependencia.as_str().unwrap())
                .output()
                .expect("Algo fallo al ejecutar bash");
            //println!("Comprobando que {} este disponible", dependencia);
            if verificar_dependencia.status.to_string() != "exit status: 127" {
                instalado = true
            } else {
                //println!("Al parecer no, porque no lo instalamos");
                instalado = false;
                break;
            }
        }
    } else {
        let dependencia_array = adi_paquete.dependencias;
        for dependencias in dependencia_array {
            let verificar_dependencia = Command::new("bash")
                .arg("-c")
                .arg(dependencias.as_str().unwrap())
                .output()
                .expect("Al parecer algo fallo al ejecutar bash");
            //println!("Comprobando que {} este instalado", dependencias);
            if verificar_dependencia.status.to_string() != "exit status: 127" {
                instalado = true;
            } else {
                //Comprobando que la dependencia este instalado por apmpkg
                let mut dependencia_adi = String::from(dependencias.as_str().unwrap());
                dependencia_adi.push_str(".adi");
                let existe_adi = Path::new("/etc/apmpkg/paquetes")
                    .join(&dependencia_adi)
                    .exists();
                let mut dependencia_abc = String::from(dependencias.as_str().unwrap());
                dependencia_abc.push_str(".abc");
                let existe_abc = Path::new("/etc/apmpkg/paquetes")
                    .join(&dependencia_abc)
                    .exists();
                if existe_abc || existe_adi {
                    instalado = true;
                } else {
                    //println!("Al parecer no, porque no instalamos");
                    instalado = false;
                    break;
                }
            }
        }
    }

    instalado
}

fn gestor(gestor: String) -> GestorNativo {
    match &gestor[..] {
        "pkg" => GestorNativo {
            nombre: "pkg".to_string(),
            buscar: "search".to_string(),
            instalacion: "install".to_string(),
            confirmacion: "-y".to_string(),
        },
        "apt" => GestorNativo {
            nombre: "apt".to_string(),
            buscar: "search".to_string(),
            instalacion: "install".to_string(),
            confirmacion: "-y".to_string(),
        },
        "pacman" => GestorNativo {
            nombre: "pacman".to_string(),
            buscar: "-Ss".to_string(),
            instalacion: "-S".to_string(),
            confirmacion: "--noconfirm".to_string(),
        },
        "dnf" => GestorNativo {
            nombre: "dnf".to_string(),
            buscar: "search".to_string(),
            instalacion: "install".to_string(),
            confirmacion: "-y".to_string(),
        },
        "snap" => GestorNativo {
            nombre: "snap".to_string(),
            buscar: "find".to_string(),
            instalacion: "install".to_string(),
            confirmacion: String::new(),
        },
        "flatpak" => GestorNativo {
            nombre: "flatpak".to_string(),
            buscar: "search".to_string(),
            instalacion: "install".to_string(),
            confirmacion: String::new(),
        },
        "zypper" => GestorNativo {
            nombre: "zypper".to_string(),
            buscar: "search".to_string(),
            instalacion: "in".to_string(),
            confirmacion: "--non-interactive".to_string(),
        },
        "yum" => GestorNativo {
            nombre: "yum".to_string(),
            buscar: "search".to_string(),
            instalacion: "install".to_string(),
            confirmacion: "-y".to_string(),
        },
        "apk" => GestorNativo {
            nombre: "apk".to_string(),
            buscar: "search".to_string(),
            instalacion: "add".to_string(),
            confirmacion: String::new(),
        },
        "slackpkg" => GestorNativo {
            nombre: "slackpkg".to_string(),
            buscar: "search".to_string(),
            instalacion: "install".to_string(),
            confirmacion: String::new(),
        },
        _ => GestorNativo {
            nombre: "apmpkg".to_string(),
            buscar: String::new(),
            instalacion: "instalar".to_string(),
            confirmacion: "-c".to_string(),
        },
    }
}

fn instalar_paquete(gestor: GestorNativo, paquete: &str) -> bool {
    if gestor.confirmacion.is_empty() {
        let comando_instalacion = Command::new(gestor.nombre)
            .arg(gestor.instalacion)
            .arg(paquete)
            .output()
            .expect("Algo fallo al instalar");
        comando_instalacion.status.to_string() == "exit status: 0"
    } else {
        let comando_instalacion = Command::new(gestor.nombre)
            .arg(gestor.instalacion)
            .arg(paquete)
            .arg(gestor.confirmacion)
            .output()
            .expect("Algo fallo al instalar");
        comando_instalacion.status.to_string() == "exit status: 0"
    }
}

pub fn instalar_dependencias(adi_paquete: AdiPaquete) -> bool {
    let catalogo = [
        "pkg", "apt", "pacman", "dnf", "zypper", "yum", "apk", "slackpkg", "snap",
        /*"npm"*/ "flatpak",
    ];
    let mut gestores = Vec::new();
    let depen_arr = &adi_paquete.dependencias;

    //Selecionando los gestores instalados
    for gestor in catalogo {
        let comando = Command::new("bash")
            .arg("-c")
            .arg(gestor)
            .output()
            .expect("Algo fallo al ejecutar bash");
        if comando.status.to_string() == "exit status: 1"
            || comando.status.to_string() == "exit status: 0"
        {
            gestores.push(gestor);
        }
    }

    //Instalacion las dependencias
    let mut contador = 0;
    let mut listo = false;
    loop {
        for dependencia in depen_arr {
            let gestor = gestor(gestores[contador].to_string());
            let dependencia = dependencia.as_str().unwrap();
            let instalacion_completada = instalar_paquete(gestor.clone(), dependencia);
            // De igual manera como se instala se verifica que la dependencia fue instalada si este da como codigo de salida 0
            if instalacion_completada {
                //println!(
                //"Se termino de instalar el paquete {} de manera correcta!",
                //depen_arr[i].as_str().unwrap()
                //);
                listo = true
            } else {
                let mut child = Command::new(gestor.nombre.clone())
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
                    listo = true
                } else {
                    listo = false
                }
            }
        }

        if listo {
            return listo;
        } else {
            contador += 1;
            if contador == gestores.len() {
                return false;
            }
        }
    }
}

pub fn instalar_dependencia_vector(depen_arr: Vec<String>) -> bool {
    let catalogo = [
        "pkg", "apt", "pacman", "dnf", "zypper", "yum", "apk", "slackpkg", "snap",
        /*"npm"*/ "flatpak",
    ];
    let mut gestores = Vec::new();

    //Selecionando los gestores instalados
    for gestor in catalogo {
        let comando = Command::new("bash")
            .arg("-c")
            .arg(gestor)
            .output()
            .expect("Algo fallo al ejecutar bash");
        if comando.status.to_string() == "exit status: 1"
            || comando.status.to_string() == "exit status: 0"
        {
            gestores.push(gestor);
        }
    }

    //Instalacion las dependencias
    let mut contador = 0;
    let mut listo = false;
    loop {
        for dependencia in depen_arr.clone() {
            let gestor = gestor(gestores[contador].to_string());
            let instalacion_completada = instalar_paquete(gestor.clone(), &dependencia);
            // De igual manera como se instala se verifica que la dependencia fue instalada si este da como codigo de salida 0
            if instalacion_completada {
                //println!(
                //"Se termino de instalar el paquete {} de manera correcta!",
                //depen_arr[i].as_str().unwrap()
                //);
                listo = true
            } else {
                let mut child = Command::new(gestor.nombre.clone())
                    .arg(gestor.buscar.clone())
                    .arg(dependencia.clone())
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
                    listo = true
                } else {
                    listo = false
                }
            }
        }

        if listo {
            return listo;
        } else {
            contador += 1;
            if contador == gestores.len() {
                return false;
            }
        }
    }
}

pub fn instalar_dependencias_externas(ruta_proyecto: &str, adi_paquete: Adi) {
    match adi_paquete.gestor {
        GestoresLenguajes::Gem(gem) => {
            if gem.gemfile {
                let ruta_gemfile = Path::new(ruta_proyecto).join(gem.file);
                let mut ruta_gemf = String::from("--gemfile=");
                ruta_gemf.push_str(ruta_gemfile.to_str().unwrap());
                let mut child = Command::new("bundle")
                    .arg("install")
                    .arg(ruta_gemf)
                    .spawn()
                    .expect("No tienes bundler instalado");
                let _result = child.wait().unwrap();
            } else {
                for gema in &gem.gemas {
                    let mut child = Command::new("gem")
                        .arg("install")
                        .arg(gema.as_str().unwrap())
                        .spawn()
                        .expect("Ocurrio un error al ejecutar gem install");
                    let _result = child.wait().unwrap();
                }
            }
        }
        GestoresLenguajes::Pip(pip) => {
            let version = pip.version;
            match version {
                2 | 3 => {}
                _ => {
                    let error = MsgError::new("Version de pip no soportada");
                    error.print_salir();
                }
            }

            if pip.requeriments {
                let ruta_requeriments = Path::new(ruta_proyecto).join(pip.file);
                if version == 2 {
                    let mut child = Command::new("pip2")
                        .arg("install")
                        .arg("-r")
                        .arg(ruta_requeriments.to_str().unwrap())
                        .spawn()
                        .expect("Algo fallo al ejecutar pip2");
                    let _result = child.wait().unwrap();
                } else {
                    let mut child = Command::new("pip3")
                        .arg("install")
                        .arg("-r")
                        .arg(ruta_requeriments.to_str().unwrap())
                        .spawn()
                        .expect("Algo fallo al ejecutar pip3");
                    let _result = child.wait().unwrap();
                }
            } else if version == 2 {
                for paquete in pip.paquetes {
                    let mut child = Command::new("pip2")
                        .arg("install")
                        .arg(paquete.as_str().unwrap())
                        .spawn()
                        .expect("Ocurrio un error al ejecutar pip2");
                    let _result = child.wait().unwrap();
                }
            } else {
                for paquete in pip.paquetes {
                    let mut child = Command::new("pip3")
                        .arg("install")
                        .arg(paquete.as_str().unwrap())
                        .spawn()
                        .expect("Ocurrio un error al ejecutar pip3");
                    let _result = child.wait().unwrap();
                }
            }
        }
        GestoresLenguajes::Ninguno => {}
    }
}

pub fn pre_instalacion(adi_instalacion: AdiInstalacion, ruta_proyecto: &Path) -> bool {
    let ruta_actual = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
            Path::new(".").to_path_buf()
        }
    };
    let cambio = env::set_current_dir(ruta_proyecto);
    match cambio {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }
    let mut child = Command::new("bash")
        .arg(adi_instalacion.pre_instalacion)
        .spawn()
        .expect("Algo fallo al ejecutar el script pre-instalacion");
    let result = child.wait().unwrap();
    let actual = env::set_current_dir(ruta_actual);
    match actual {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir()
        }
    }
    result.to_string() == "exit status: 0"
}

pub fn post_instalacion(adi_instalacion: AdiInstalacion, ruta_proyecto: &Path) -> bool {
    let ruta_actual = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
            Path::new(".").to_path_buf()
        }
    };
    let cambio = env::set_current_dir(ruta_proyecto);
    match cambio {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }
    let mut child = Command::new("bash")
        .arg(adi_instalacion.pre_instalacion)
        .spawn()
        .expect("Algo fallo al ejecutar el script pre-instalacion");
    let result = child.wait().unwrap();
    let actual = env::set_current_dir(ruta_actual);
    match actual {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir()
        }
    }
    result.to_string() == "exit status: 0"
}

pub fn remover_abc(ruta: &str) {
    let mut child = Command::new("bash")
        .arg("/etc/apmpkg/iiabc/iiabc.sh")
        .arg("-r")
        .arg(ruta)
        .spawn()
        .expect("Ocurrio un erro al remover el paquete");
    let _result = child.wait().unwrap();
}

/*
 * Inicio de implementacion de funciones para la estructura Adi
*/

impl Adi {
    pub fn nuevo(archivo: &str) -> Adi {
        //Leyendo el archivo
        let archivo_str: String = archivos::leer_archivo(archivo);
        let tomy: Value = match toml::from_str(&archivo_str) {
            Ok(v) => v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
                Value::String("nada".to_string())
            }
        };
        let adi_tomy = tomy.as_table().unwrap();
        if !adi_tomy.contains_key("paquete")
            || !adi_tomy.contains_key("descarga")
            || !adi_tomy.contains_key("instalacion")
        {
            let error = MsgError::new("Douh, eso no parece un archivo adi");
            error.print_salir();
        }
        let paquete_tabla = adi_tomy["paquete"].as_table().unwrap();

        //Colocando en esctruturas
        let adi_paquete = AdiPaquete {
            nombre: paquete_tabla["nombre"].as_str().unwrap().to_string(),
            version: paquete_tabla["version"].as_str().unwrap().to_string(),
            rama: paquete_tabla["rama"].as_str().unwrap().to_string(),
            descripcion: paquete_tabla["descrip"].as_str().unwrap().to_string(),
            pagina: paquete_tabla["pagina"].as_str().unwrap().to_string(),
            licensia: paquete_tabla["licensia"].as_str().unwrap().to_string(),
            dependencias: paquete_tabla["dependencias"].as_array().unwrap().to_vec(),
            dependencias_string: {
                let mut dependencias = String::new();
                for depend in paquete_tabla["dependencias"].as_array().unwrap().to_vec() {
                    dependencias.push_str(depend.as_str().unwrap());
                    dependencias.push(' ');
                }
                dependencias
            },
            cmd_depen: if paquete_tabla.contains_key("cmd_depen") {
                paquete_tabla["cmd_depen"].as_array().unwrap().to_vec()
            } else {
                Vec::new()
            },
            abi_dependencias: if paquete_tabla.contains_key("abi_dependencias") {
                paquete_tabla["abi_dependencias"]
                    .as_array()
                    .unwrap()
                    .to_vec()
            } else {
                Vec::new()
            },
            arquitectura: if paquete_tabla.contains_key("arch") {
                paquete_tabla["arch"].as_str().unwrap().to_string()
            } else {
                String::from("any")
            },
            conflicto: paquete_tabla["conflicto"].as_str().unwrap().to_string(),
        };
        let paquete = adi_paquete.clone();
        let dependencias_adi = AbiDependencias {
            paquetes: if adi_tomy.contains_key("dependencias_adi") {
                let dependencias_tabla = adi_tomy["dependencias_adi"].as_table().unwrap();
                let mut dependencia_array = Vec::new();
                for dependencia in paquete.dependencias.clone() {
                    if dependencias_tabla.contains_key(&dependencia.to_string()) {
                        dependencia_array.push(dependencia);
                    }
                }
                dependencia_array
            } else {
                Vec::new()
            },
            url: if adi_tomy.contains_key("dependencias_adi") {
                let dependencias_tabla = adi_tomy["dependencias_adi"].as_table().unwrap();
                let mut url_array = Vec::new();
                for dependencia in &paquete.dependencias {
                    if dependencias_tabla.contains_key(&dependencia.to_string()) {
                        url_array.push(
                            dependencias_tabla[&dependencia.to_string()]
                                .as_str()
                                .unwrap()
                                .to_string(),
                        );
                    }
                }
                url_array
            } else {
                Vec::new()
            },
        };
        let descarga_tabla = adi_tomy["descarga"].as_table().unwrap();
        let adi_descarga = AdiDescarga {
            fuente: if descarga_tabla.contains_key("url") {
                Fuente::Url(descarga_tabla["url"].as_str().unwrap().to_string())
            } else if descarga_tabla.contains_key("local") {
                Fuente::Local(descarga_tabla["local"].as_str().unwrap().to_string())
            } else {
                Fuente::Git(
                    descarga_tabla["git"]
                        .as_str()
                        .expect("No exxiste ningun repositorio git")
                        .to_string(),
                )
            },
            carpeta: descarga_tabla["carpeta"].as_str().unwrap().to_string(),
            sumasha: descarga_tabla["sha256sum"].as_str().unwrap().to_string(),
        };
        let instalacion_tabla = adi_tomy["instalacion"].as_table().unwrap();
        let adi_instalacion = AdiInstalacion {
            fuente_opt: if instalacion_tabla.contains_key("opt_src") {
                instalacion_tabla["opt_src"].as_bool().unwrap()
            } else {
                false
            },
            pre_instalacion: if instalacion_tabla.contains_key("pre_install") {
                instalacion_tabla["pre_install"]
                    .as_str()
                    .unwrap()
                    .to_string()
            } else {
                String::new()
            },
            fuente: instalacion_tabla["files"].as_array().unwrap().to_vec(),
            destino: instalacion_tabla["ruta"].as_array().unwrap().to_vec(),
            post_instalacion: if instalacion_tabla.contains_key("post_install") {
                instalacion_tabla["post_install"]
                    .as_str()
                    .unwrap()
                    .to_string()
            } else {
                String::new()
            },
            mensaje: if instalacion_tabla.contains_key("mensaje") {
                instalacion_tabla["mensaje"].as_str().unwrap().to_string()
            } else {
                String::new()
            },
        };
        let adi_gestor = if adi_tomy.contains_key("gem") {
            let gem_tabla = adi_tomy["gem"].as_table().unwrap();
            let ruby_gem = Gem {
                gemfile: gem_tabla["gemfile"].as_bool().unwrap(),
                file: if gem_tabla.contains_key("file") {
                    gem_tabla["file"].as_str().unwrap().to_string()
                } else {
                    String::new()
                },
                gemas: if gem_tabla.contains_key("gemas") {
                    gem_tabla["gemas"].as_array().unwrap().to_vec()
                } else {
                    Vec::new()
                },
            };
            GestoresLenguajes::Gem(ruby_gem)
        } else if adi_tomy.contains_key("pip") {
            let pip_tabla = adi_tomy["pip"].as_table().unwrap();
            let pip_struct = Pip {
                version: pip_tabla["version"].as_integer().unwrap(),
                requeriments: pip_tabla["requirements"].as_bool().unwrap(),
                paquetes: if pip_tabla.contains_key("packages") {
                    pip_tabla["packages"].as_array().unwrap().to_vec()
                } else {
                    Vec::new()
                },
                file: if pip_tabla.contains_key("file") {
                    pip_tabla["file"].as_str().unwrap().to_string()
                } else {
                    String::new()
                },
            };
            GestoresLenguajes::Pip(pip_struct)
        } else {
            GestoresLenguajes::Ninguno
        };
        Adi {
            paquete: adi_paquete,
            abi_dependencias: dependencias_adi,
            gestor: adi_gestor,
            descarga: adi_descarga,
            instalacion: adi_instalacion,
        }
    }

    pub fn imprimir_metadatos(&self) {
        println!(
            "
	\t\t        Paquete: {}
	\t\t           Rama: {} 
	\t\t Version actual: {}
	\t\t    Descripcion: {}
        \t\t   Dependencias: {}
	\n\n",
            &self.paquete.nombre,
            &self.paquete.rama,
            &self.paquete.version,
            &self.paquete.descripcion,
            &self.paquete.dependencias_string
        );
    }
}
