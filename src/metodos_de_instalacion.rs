//Todos los metodos de instalacion

// use
use {
    crate::{archivos, core_funcions, estructuras::*},
    colored::*,
    pbr::ProgressBar,
    std::path::Path,
    std::{fs, process, process::Command},
};

//Instalacion abc
pub fn instalar_abc(ruta: &str, binario: bool) {
    println!("Iniciando desde un .abc");

    if binario {
        let mut child = process::Command::new("bash")
            .arg("/etc/apmpkg/iiabc/iiabc.sh")
            .arg("-b")
            .arg(ruta)
            .spawn()
            .expect("Al parecer no tienes iiabc, algo anda mal");
        let _result = child.wait().unwrap();
    } else {
        let mut child = process::Command::new("bash")
            .arg("/etc/apmpkg/iiabc/iiabc.sh")
            .arg("-i")
            .arg(ruta)
            .spawn()
            .expect("Al parecer no tienes iiabc, algo anda mal");
        let _result = child.wait().unwrap();
    }
}

pub fn instalar_adi(ruta_archivo: &str, confirmacion: bool, binario: bool) {
    let paquete = Adi::nuevo(ruta_archivo);
    paquete.imprimir_metadatos();

    if confirmacion {
        println!("{}", "Omitiendo confirmacion...".yellow());
    } else {
        let pregunta = core_funcions::pregunta("¿Deseas seguir con la instalacion?");
        if !pregunta {
            let error = MsgError::new("¡Abortando instalacion!");
            error.print_salir();
        }
    }

    //Comprobando si existe una instalacion
    let mut instalacion_adi = paquete.paquete.nombre.clone();
    instalacion_adi.push_str(".adi");
    let actualizacion = Path::new("/etc/apmpkg/paquetes")
        .join(instalacion_adi)
        .is_file();
    if actualizacion {
        println!(
            "{}{}",
            "Actualizando el paquete ".yellow(),
            paquete.paquete.nombre
        );
    }

    //Barra de progreso
    let contador = 13;
    let mut pb = ProgressBar::new(contador);
    pb.format("(->.)");

    pb.message("Creando directorios ");
    pb.inc();
    let mut dir_name = paquete.paquete.nombre.clone();
    dir_name.push_str(".d");
    let directorio = Path::new(&dir_name);
    if directorio.exists() {
        let borrar = core_funcions::pregunta(
            "Al parecer el directorio de trabajo ya esta creado, ¿Quieres borrarlo?",
        );
        if !borrar {
            let error = MsgError::new("No se puede continuar a menos que elimine dicho directorio");
            error.print_salir();
        }
        let borrar_dir = fs::remove_dir_all(directorio);
        match borrar_dir {
            Ok(_v) => _v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
    }

    pb.message("Buscando conflictos ");
    pb.inc();
    let existe_conflictos = Path::new(&paquete.paquete.conflicto).exists();
    if existe_conflictos {
        let mensaje = format!(
            "No se puede instalar, el archivo {} entra en conflicto",
            &paquete.paquete.conflicto
        );
        let error = MsgError::new(&mensaje);
        error.print_salir();
    }

    pb.message("Verificando arquitectura ");
    pb.inc();
    if !core_funcions::verificar_arch(paquete.paquete.clone()) {
        let error = MsgError::new("Al parecer no cuentas con la arquitectura requerida");
        error.print_salir();
    }

    pb.message("Resolviendo dependencias ");
    pb.inc();
    let dependencias_instaladas = core_funcions::dependencias_instaladas(paquete.paquete.clone());
    if !dependencias_instaladas {
        let dependencias_instalar = core_funcions::instalar_dependencias(paquete.paquete.clone());
        if !dependencias_instalar {
            //Agregar linea para la dependencias abi
            let error = MsgError::new("No se pudieron descargar las dependencias");
            error.print_salir();
        }
    }

    pb.message("Obteniendo fuentes ");
    pb.inc();
    let mut acd = String::from(&paquete.paquete.nombre);
    acd.push('-');
    acd.push_str(&paquete.paquete.version);
    acd.push_str(".acd.tar");
    let fuentes = paquete.descarga.fuente.clone();
    match fuentes {
        Fuente::Git(repositorio) => {
            if let Err(e) = archivos::git_clone(
                &repositorio,
                directorio
                    .join(paquete.descarga.carpeta.clone())
                    .to_str()
                    .unwrap(),
            ) {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
        Fuente::Url(url) => {
            let creando = archivos::crear_directorio(directorio.to_str().unwrap());
            match creando {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
            let descarga = archivos::descarga(&url, directorio.join(acd.clone()).to_str().unwrap());
            match descarga {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
        }
        Fuente::Local(ruta) => {
            let creando = archivos::crear_directorio(directorio.to_str().unwrap());
            match creando {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
            archivos::copiar_archivo(&ruta, directorio.join(&acd).to_str().unwrap());
        }
    }

    pb.message("Verificando la integridad del archivo ");
    pb.inc();
    if paquete.descarga.sumasha == "SALTAR" {
        println!("{}", "¡Se a saltado la verificacion!".yellow());
    } else if !archivos::verificacion_hash(
        directorio.join(acd.clone()).to_str().unwrap(),
        &paquete.descarga.sumasha,
    ) {
        let error = MsgError::new("Las sumas no son coinciden");
        error.print_salir();
    }

    pb.message("Extrayendo fuentes ");
    pb.inc();
    if let Fuente::Git(_repositorio) = paquete.descarga.fuente.clone() {
        let _rr = _repositorio;
    } else {
        let tar = archivos::extraer_tar(
            directorio.join(&acd).to_str().unwrap(),
            directorio.to_str().unwrap(),
        );
        match tar {
            Ok(_v) => _v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
    }

    pb.message("Iniciando la instalacion de dependencias del proyecto ");
    pb.inc();
    let root_proyecto = &directorio.join(&paquete.descarga.carpeta);
    core_funcions::instalar_dependencias_externas(root_proyecto.to_str().unwrap(), paquete.clone());

    pb.message("Ejecutando scripts pre-instalacion ");
    pb.inc();
    if !paquete.instalacion.pre_instalacion.is_empty() {
        let pre_instalacion_hecha = core_funcions::pre_instalacion(
            paquete.instalacion.clone(),
            directorio.join(paquete.descarga.carpeta.clone()).as_path(),
        );
        if !pre_instalacion_hecha {
            let error = MsgError::new("Ocurrio un error al ejecutar el script post instalacion");
            error.print_salir()
        }
    }

    pb.message("Iniciando instalacion ");
    pb.inc();
    let tmp = directorio.join(paquete.descarga.carpeta.clone());
    let dir = tmp.as_path();
    archivos::instalar_archivos(paquete.instalacion.clone(), dir.to_str().unwrap());

    pb.message("Ejecutando scripts post-instalacion ");
    pb.inc();
    if !paquete.instalacion.post_instalacion.is_empty() {
        let post_instalacion_hecha = core_funcions::post_instalacion(
            paquete.instalacion.clone(),
            directorio.join(paquete.descarga.carpeta.clone()).as_path(),
        );
        if !post_instalacion_hecha {
            let error = MsgError::new("Ocurrio un error al ejecutar el script post instalacion");
            error.print_salir()
        }
    }

    pb.message("Ejecutando los ultimos pasos ");
    pb.inc();
    let mut nombre_adi = paquete.paquete.nombre.clone();
    nombre_adi.push_str(".adi");
    let tmp_adi = Path::new("/etc/apmpkg/paquetes").join(nombre_adi);
    let archivo_adi = tmp_adi.as_path();
    archivos::copiar_archivo(ruta_archivo, archivo_adi.to_str().unwrap());

    //Creando el binario
    if binario {
        pb.message("Creando binario ");
        pb.inc();
        archivos::construir_binario(paquete.clone(), directorio, ruta_archivo);
    } else {
        pb.message("Limpiando ");
        pb.inc();
    }

    //Limpiando
    let borrar_dir = fs::remove_dir_all(directorio);
    match borrar_dir {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }

    pb.finish_print("¡Instalacion completada!");
    if !paquete.instalacion.mensaje.is_empty() {
        println!("{}", paquete.instalacion.mensaje);
    }
}

pub fn instalar_abi(ruta: &str, confirmacion: bool) {
    //Barra de progreso
    let contador = 2;
    let mut pba = ProgressBar::new(contador);
    pba.format("(->.)");

    pba.message("Desempacando binario ");
    pba.inc();
    if Path::new("install.d").exists() {
        let borrar = core_funcions::pregunta("El directorio install.d ya existe, ¿Desea borrarlo?");
        if !borrar {
            let borrar_error = MsgError::new(
                "No borar el directorio puede ocacionar que la instalacion no se realize correctamente",
            );
            borrar_error.print();
        } else {
            let borrar_dir = fs::remove_dir_all("install.d");
            match borrar_dir {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
        }
    }
    let resultado_tar = archivos::extraer_tar(ruta, "install.d/");
    match resultado_tar {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }

    pba.message("Dectectando archivo ");
    pba.inc();
    if archivos::existe_adi() {
        pba.finish_print(" ");

        //leyendo datos
        let adi = Adi::nuevo("install.d/apkg.adi");
        let directorio = Path::new("install.d");
        adi.imprimir_metadatos();
        if confirmacion {
            println!("{}", "Omitiendo confirmacion".yellow());
        } else {
            let pregunta = core_funcions::pregunta("¿Deseas seguir con la instalacion?");
            if !pregunta {
                let error = MsgError::new("Abortano instalacion");
                let borrar_dir = fs::remove_dir_all("install.d");
                match borrar_dir {
                    Ok(_v) => _v,
                    Err(e) => {
                        let error = MsgError::new(&e.to_string());
                        error.print_salir();
                    }
                }
                error.print_salir();
            }
        }
        //Barra de progreso
        let contador = 8;
        let mut pb = ProgressBar::new(contador);
        pb.format("(->.)");

        pb.message("Buscando conflictos ");
        pb.inc();
        let existe_conflictos = Path::new(&adi.paquete.conflicto).exists();
        if existe_conflictos {
            let mensaje = format!(
                "No se puede instalar, el archivo {} entra en conflicto",
                &adi.paquete.conflicto
            );
            let error = MsgError::new(&mensaje);
            error.print_salir();
        }

        pb.message("Verificando arquitectura ");
        pb.inc();
        if !core_funcions::verificar_arch(adi.paquete.clone()) {
            let error = MsgError::new("Al parecer no cuentas con la arquitectura requerida");
            error.print_salir();
        }

        pb.message("Resolviendo dependencias ");
        pb.inc();
        let dependencias_instaladas = core_funcions::dependencias_instaladas(adi.paquete.clone());
        if !dependencias_instaladas {
            let dependencias_instalar = core_funcions::instalar_dependencias(adi.paquete.clone());
            if !dependencias_instalar {
                //Agregar linea para la dependencias abi
                let error = MsgError::new("No se pudieron descargar las dependencias");
                error.print_salir();
            }
        }

        pb.message("Iniciando instalacion ");
        pb.inc();
        let binario_completo = archivos::binario_completo(adi.clone());
        if binario_completo {
            let ruta_archivos = directorio.join(adi.descarga.carpeta.clone());

            core_funcions::instalar_dependencias_externas(
                ruta_archivos.to_str().unwrap(),
                adi.clone(),
            );

            pb.message("Instalando archivos ");
            pb.inc();
            let temporal = ruta_archivos.as_path();
            archivos::instalar_archivos(adi.instalacion.clone(), temporal.to_str().unwrap());

            pb.message("Ejecutando scripts post-instalacion");
            pb.inc();
            if !adi.instalacion.post_instalacion.is_empty() {
                let post_instalacion_hecha =
                    core_funcions::post_instalacion(adi.instalacion.clone(), temporal);
                if !post_instalacion_hecha {
                    let error =
                        MsgError::new("Algo fallo al ejecutar los scripts post-instalacion");
                    error.print_salir();
                }
            }
        } else {
            let mut carpeta = adi.paquete.nombre.clone();
            carpeta.push_str(".d");
            let ruta_proyecto = directorio.join(carpeta).join(adi.descarga.carpeta.clone());

            core_funcions::instalar_dependencias_externas(
                ruta_proyecto.to_str().unwrap(),
                adi.clone(),
            );

            pb.message("Instalando archivos");
            pb.inc();
            let temporal = ruta_proyecto.as_path();
            archivos::instalar_archivos(adi.instalacion.clone(), temporal.to_str().unwrap());

            pb.message("Ejecutando scripts post-instalacion");
            pb.inc();
            if !adi.instalacion.post_instalacion.is_empty() {
                let post_instalacion_hecha =
                    core_funcions::post_instalacion(adi.instalacion.clone(), temporal);
                if !post_instalacion_hecha {
                    let error =
                        MsgError::new("Algo fallo al ejecutar los scripts post-instalacion");
                    error.print_salir();
                }
            }
        }

        pb.message("Ejecutando los ultimos pasos");
        pb.inc();
        let ruta_adi = Path::new("/etc/apmpkg/paquetes");
        let mut nombre_adi = adi.paquete.nombre;
        nombre_adi.push_str(".adi");
        archivos::copiar_archivo(
            "install.d/apkg.adi",
            ruta_adi.join(nombre_adi).to_str().unwrap(),
        );
        let borrar_dir = fs::remove_dir_all(directorio);
        match borrar_dir {
            Ok(_v) => _v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }

        pb.finish_print("¡La instalacion se realizo correctamente!");
        if !adi.instalacion.mensaje.is_empty() {
            println!("{}", adi.instalacion.mensaje);
        }
    } else {
        let salida = fs::remove_dir_all("install.d");
        match salida {
            Ok(_v) => _v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
        let mut child = Command::new("bash")
            .arg("/etc/apmpkg/iiabc/iiabc.sh")
            .arg("-ib")
            .arg(ruta)
            .spawn()
            .expect("Ocurrio un error al instalar el binario");
        let _result = child.wait().unwrap();
    }
}

pub fn remover_adi(nombre: &str, confirmacion: bool) {
    let mut adi_nombre = String::from(nombre);
    adi_nombre.push_str(".adi");
    let ruta_adi = Path::new("/etc/apmpkg/paquetes").join(adi_nombre);
    let adi = Adi::nuevo(ruta_adi.to_str().unwrap());
    adi.imprimir_metadatos();

    let mut pb = ProgressBar::new(2);
    pb.format("(->.)");

    if confirmacion {
        println!("{}", "Omitiendo confirmacion...".yellow());
    } else {
        let pregunta = core_funcions::pregunta("¿Deseas seguir con las desinstalacion?");
        if !pregunta {
            let error = MsgError::new("Abortando desinstalacion");
            error.print_salir();
        }
    }

    pb.message("Removiendo archivos ");
    pb.inc();
    archivos::remover_archivos(adi);

    pb.message("Realizando los ultimos movimientos ");
    pb.inc();
    let remover = std::fs::remove_file(ruta_adi);
    match remover {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }

    pb.finish_print("¡La deinstalacion se llevo con exito!");
}

pub fn construir_binario_adi(ruta: &str) {
    let mut pb = ProgressBar::new(7);
    pb.format("(->.)");
    pb.message("Leyendo archivo ");
    pb.inc();
    let adi = Adi::nuevo(ruta);

    //Directorios
    pb.message("Creando directorios ");
    pb.inc();
    let mut nombre_ruta = adi.paquete.nombre.clone();
    nombre_ruta.push_str(".d");
    let directorio = Path::new(&nombre_ruta);
    if directorio.exists() {
        let borrar = core_funcions::pregunta(
            "Al parecer el directorio de trabajo ya esta creado, ¿Desea eliminarlo?",
        );
        if !borrar {
            let error = MsgError::new("No se puede continuar con la creacion del binario a menos que se borre dicho directorio");
            error.print_salir();
        }
        let salida_borrar = fs::remove_dir_all(directorio);
        match salida_borrar {
            Ok(_v) => _v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
    }

    //Obteniendo las fuentes
    pb.message("Obteniendo fuentes ");
    pb.inc();
    let mut acd = String::from(&adi.paquete.nombre);
    acd.push('-');
    acd.push_str(&adi.paquete.version);
    acd.push_str(".acd.tar");
    let fuentes = adi.descarga.fuente.clone();
    match fuentes {
        Fuente::Git(repositorio) => {
            if let Err(e) = archivos::git_clone(
                &repositorio,
                directorio
                    .join(adi.descarga.carpeta.clone())
                    .to_str()
                    .unwrap(),
            ) {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
        Fuente::Url(url) => {
            let creando = archivos::crear_directorio(directorio.to_str().unwrap());
            match creando {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
            let descarga = archivos::descarga(&url, directorio.join(acd.clone()).to_str().unwrap());
            match descarga {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
        }
        Fuente::Local(ruta) => {
            let creando = archivos::crear_directorio(directorio.to_str().unwrap());
            match creando {
                Ok(_v) => _v,
                Err(e) => {
                    let error = MsgError::new(&e.to_string());
                    error.print_salir();
                }
            }
            archivos::copiar_archivo(&ruta, directorio.join(&acd).to_str().unwrap());
        }
    }

    pb.message("Verificando la integridad del archivo ");
    pb.inc();
    if adi.descarga.sumasha == "SALTAR" {
        println!("{}", "¡Se a saltado la verificacion!".yellow());
    } else if !archivos::verificacion_hash(
        directorio.join(acd.clone()).to_str().unwrap(),
        &adi.descarga.sumasha,
    ) {
        let error = MsgError::new("Las sumas no son coinciden");
        error.print_salir();
    }

    pb.message("Extrayendo fuentes ");
    pb.inc();
    if let Fuente::Git(_repositorio) = adi.descarga.fuente.clone() {
        let _rr = _repositorio;
    } else {
        let tar = archivos::extraer_tar(
            directorio.join(&acd).to_str().unwrap(),
            directorio.to_str().unwrap(),
        );
        match tar {
            Ok(_v) => _v,
            Err(e) => {
                let error = MsgError::new(&e.to_string());
                error.print_salir();
            }
        }
    }

    pb.message("Ejecutando scripts pre-instalacion");
    pb.inc();
    if !adi.instalacion.pre_instalacion.is_empty() {
        let pre_instalacion_hecha = core_funcions::pre_instalacion(
            adi.instalacion.clone(),
            directorio.join(adi.descarga.carpeta.clone()).as_path(),
        );
        if !pre_instalacion_hecha {
            let error = MsgError::new("Ocurrio un error al ejecutar el script post instalacion");
            error.print_salir()
        }
    }

    pb.message("Construyendo binario ");
    pb.inc();
    archivos::construir_binario(adi, directorio, ruta);
    let salida_borrar = fs::remove_dir_all(directorio);
    match salida_borrar {
        Ok(_v) => _v,
        Err(e) => {
            let error = MsgError::new(&e.to_string());
            error.print_salir();
        }
    }
    pb.finish_print("¡La creacion del binario a resultado ser correcta!");
}
