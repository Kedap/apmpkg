//Modulo para la manipulacion de archivos

use {
    crate::estructuras::{Adi, AdiInstalacion, GestoresLenguajes, MsgError},
    flate2::{read::GzDecoder, write::GzEncoder, Compression},
    read_input::prelude::*,
    sha2::{Digest, Sha256},
    std::{fs, fs::File, io, path::Path, process::Command},
    tar::Archive,
};

pub fn es_abi(path: &str) -> bool {
    let comando_file = Command::new("file")
        .arg("-i")
        .arg(path)
        .output()
        .expect("Ocurrio un error al ejecutar el comando file");
    let comando_salida = String::from_utf8_lossy(&comando_file.stdout);

    // Tipo de salidas segun los soportados
    let mut adi_file = String::new();
    adi_file.push_str(path);
    adi_file.push_str(": text/plain; charset=us-ascii\n");
    let mut abi_file = String::new();
    abi_file.push_str(path);
    abi_file.push_str(": application/gzip; charset=binary\n");

    if comando_salida == abi_file {
        true
    } else if comando_salida == adi_file {
        false
    } else {
        let error = MsgError::new("El archivo no es soportado, prueba con otro");
        error.print_salir();
        false
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
    comando_salida == abc_salida
}

pub fn leer_archivo(ruta: &str) -> String {
    match fs::read_to_string(ruta) {
        Ok(v) => v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
            String::new()
        }
    }
}

pub fn git_clone(repositorio: &str, destino: &str) {
    let mut child = Command::new("git")
        .arg("clone")
        .arg(repositorio)
        .arg(destino)
        .spawn()
        .expect("Algo fallo con git_clone");
    let _result = child.wait().unwrap();
}

#[tokio::main]
pub async fn descarga(url: &str, ruta: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cuerpo = reqwest::get(url).await?.bytes().await?;
    let ar = escribir_archivo(ruta, &cuerpo);
    match ar {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }
    Ok(())
}

pub fn escribir_archivo(nombre: &str, contenido: &[u8]) -> io::Result<()> {
    let mut archivo = File::create(nombre).expect("Algo fallo al crear el archivo");
    let mut cont = contenido;
    io::copy(&mut cont, &mut archivo)?;
    Ok(())
}

pub fn copiar_archivo(desde: &str, destino: &str) {
    let mut child = Command::new("cp")
        .arg("-r")
        .arg(desde)
        .arg(destino)
        .spawn()
        .expect("Algo fallo con cp");
    let _result = child.wait().unwrap();
}

pub fn crear_directorio(directorio: &str) -> std::io::Result<()> {
    fs::create_dir_all(directorio)?;
    Ok(())
}

pub fn verificacion_hash(ruta: &str, suma_esperada: &str) -> bool {
    let mut archivo = File::open(ruta).expect("Ocurrio un error al abrir el archivo");
    let mut suma = Sha256::new();
    io::copy(&mut archivo, &mut suma).expect("Error al copiar");
    let fhash = format!("{:x}", &suma.finalize());

    fhash == suma_esperada
}

pub fn extraer_tar(ruta_tar: &str, ruta_destino: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(ruta_tar)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archivo = Archive::new(tar);
    archivo.unpack(ruta_destino)?;

    Ok(())
}

pub fn instalar_archivos(adi_instalacion: AdiInstalacion, carpeta_src: &str) {
    let fuente = adi_instalacion.fuente;
    let destino = adi_instalacion.destino;
    let carpeta_fuente = Path::new(carpeta_src);
    let mut usuario = String::new();

    for i in 0..fuente.len() {
        let archivo = carpeta_fuente.join(fuente[i].as_str().unwrap());
        if i == 0 {
            let mut child = Command::new("install")
                .arg("-Dm")
                .arg("755")
                .arg(archivo)
                .arg(destino[i].as_str().unwrap())
                .spawn()
                .expect("Algo fallo con install");
            let _result = child.wait().unwrap();
        } else if Path::new(destino[i].as_str().unwrap()).is_relative() {
            if usuario.is_empty() {
                let directorys = match fs::read_dir("/home") {
                    Ok(v) => v,
                    Err(e) => {
                        let error = MsgError::new(&e.to_string());
                        error.print_salir();
                        panic!();
                    }
                };
                println!("¿Para que usuario quieres instalar este paquete?");
                for usuario in directorys {
                    println!(
                        "{}",
                        usuario
                            .unwrap()
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                    );
                }

                print!("> ");
                usuario = input().get();
                if usuario.is_empty() {
                    let error = MsgError::new("Eso no parece el nombre de un usuario");
                    error.print_salir();
                }

                let destino_usuario = Path::new("/home")
                    .join(usuario.clone())
                    .join(destino[i].as_str().unwrap());
                let mut child = Command::new("rsync")
                    .arg("-a")
                    .arg(archivo)
                    .arg(destino_usuario.to_str().unwrap())
                    .spawn()
                    .expect("Ocurrio un error con rsync");
                let _result = child.wait().unwrap();
            } else {
                let destino_usuario = Path::new("/home")
                    .join(usuario.clone())
                    .join(destino[i].as_str().unwrap());
                let mut child = Command::new("rsync")
                    .arg("-a")
                    .arg(archivo)
                    .arg(destino_usuario.to_str().unwrap())
                    .spawn()
                    .expect("Ocurrio un error con rsync");
                let _result = child.wait().unwrap();
            }
        } else {
            let mut child = Command::new("rsync")
                .arg("-a")
                .arg(archivo)
                .arg(destino[i].as_str().unwrap())
                .spawn()
                .expect("Ocurrio un error con rsync");
            let _result = child.wait().unwrap();
        }
    }

    if adi_instalacion.fuente_opt {
        copiar_archivo(carpeta_src, "/opt/");
    }
}

pub fn borrar_archivo(ruta: &str) {
    let resultado = fs::remove_file(ruta);
    match resultado {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }
}

pub fn existe_adi() -> bool {
    let resultado_cat = Command::new("cat")
        .arg("install.d/apkg.adi")
        .output()
        .expect("Ocurrio un error al ejecutar cat");

    resultado_cat.status.to_string() != "exit status: 1"
}

pub fn binario_completo(adi: Adi) -> bool {
    let conservar: bool;
    if let GestoresLenguajes::Ninguno = adi.gestor {
        conservar = false;
    } else {
        conservar = true;
    }

    conservar
}

pub fn construir_binario(adi: Adi, ruta: &Path, ruta_adi: &str) {
    let binario_completo = binario_completo(adi.clone());
    let mut nombre_binario = adi.paquete.nombre;
    nombre_binario.push('-');
    nombre_binario.push_str(&adi.paquete.version);
    nombre_binario.push_str(".abi.tar.gz");
    let tar_gz = File::create(nombre_binario).expect("Algo fallo al crear el archivo tar_gz");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    if binario_completo {
        tar.append_dir_all(".", ruta)
            .expect("Algo fallo al crear el archivo comprimido");
    } else {
        let ruta_proyecto = ruta.join(adi.descarga.carpeta.clone());
        let archivos = adi.instalacion.fuente.clone();

        for archivo in archivos {
            if ruta_proyecto.join(archivo.as_str().unwrap()).is_dir() {
                tar.append_dir_all(
                    &ruta_proyecto.join(archivo.as_str().unwrap()),
                    &ruta_proyecto.join(archivo.as_str().unwrap()),
                )
                .expect("Algo fallo al comprimir los archivos");
            } else {
                tar.append_path(&ruta_proyecto.join(archivo.as_str().unwrap()))
                    .expect("Algo fallo al comprimir los archivos");
            }
        }
    }

    copiar_archivo(ruta_adi, "apkg.adi");
    tar.append_path("apkg.adi").unwrap();
    let borrar_archivo = fs::remove_file("apkg.adi");
    match borrar_archivo {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }

    if !adi.instalacion.pre_instalacion.is_empty() {
        tar.append_path(
            ruta.join(adi.descarga.carpeta.clone())
                .join(adi.instalacion.pre_instalacion.clone()),
        )
        .unwrap();
    }

    if !adi.instalacion.post_instalacion.is_empty() {
        tar.append_path(
            ruta.join(adi.descarga.carpeta.clone())
                .join(adi.instalacion.post_instalacion),
        )
        .unwrap();
    }
}

pub fn remover_archivos(adi: Adi) {
    let adi_descarga = adi.descarga.clone();
    let adi_instalacion = adi.instalacion;
    let mut usuario = String::new();

    for archivos in adi_instalacion.destino {
        if Path::new(archivos.as_str().unwrap()).is_relative() {
            if usuario.is_empty() {
                let directorys = match fs::read_dir("/home") {
                    Ok(v) => v,
                    Err(e) => {
                        let error = MsgError::new(&e.to_string());
                        error.print_salir();
                        panic!();
                    }
                };
                println!("¿Para que usuario quieres desinstalar este paquete?");
                for usuario in directorys {
                    println!(
                        "{}",
                        usuario
                            .unwrap()
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                    );
                }

                print!("> ");
                usuario = input().get();
                if usuario.is_empty() {
                    let error = MsgError::new("Eso no parece el nombre de un usuario");
                    error.print_salir();
                }

                let destino_usuario = Path::new("/home")
                    .join(usuario.clone())
                    .join(archivos.as_str().unwrap());
                remover_rm(destino_usuario.to_str().unwrap());
            } else {
                let destino_usuario = Path::new("/home")
                    .join(usuario.clone())
                    .join(archivos.as_str().unwrap());
                remover_rm(destino_usuario.to_str().unwrap());
            }
        } else {
            remover_rm(archivos.as_str().unwrap());
        }
    }

    if adi_instalacion.fuente_opt {
        let mut opt_ruta = String::from("/opt/");
        opt_ruta.push_str(&adi_descarga.carpeta);
        remover_rm(&opt_ruta);
    }
}

pub fn remover_rm(ruta: &str) {
    Command::new("rm")
        .arg("-rf")
        .arg(ruta)
        .output()
        .expect("Algo raro sucedio con rm -r");
}
