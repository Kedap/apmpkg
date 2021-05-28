// Manejador como bundle o pip

//uses
use {
    crate::estructuras::{AdiGem, AdiPip},
    colored::*,
    std::{process, process::Command},
    toml::Value,
};

fn install_bundle(path: &str) -> bool {
    let mut gem = String::from("--gemfile=");
    gem.push_str(path);
    println!("Iniciando instalacion del archivo Gemfile");
    let mut child = Command::new("bundle")
        .arg("install")
        .arg(gem)
        .spawn()
        .expect("No tenis el bundle");
    let _result = child.wait().unwrap();
    true
}

fn install_pip(version: i64, path: &str) -> bool {
    if version == 2 {
        let mut child = Command::new("pip2")
            .arg("install")
            .arg("-r")
            .arg(path)
            .spawn()
            .expect("No tenis el pip2?");
        let _result = child.wait().unwrap();
    } else {
        let mut child = Command::new("pip3")
            .arg("install")
            .arg("-r")
            .arg(path)
            .spawn()
            .expect("No tenis el pip3?");
        let _result = child.wait().unwrap();
    }
    true
}

fn install_gem(gemas: Vec<Value>) -> bool {
    for i in 0..gemas.len() {
        println!("Instalando la gema {}", gemas[i]);
        let mut child = Command::new("gem")
            .arg("install")
            .arg(gemas[i].as_str().unwrap())
            .spawn()
            .expect("No tenis ruby?");
        let _result = child.wait().unwrap();
    }
    true
}

fn pip_pack(version: i64, packages: Vec<Value>) -> bool {
    if version == 2 {
        let pipa = "pip2";
        for i in 0..packages.len() {
            println!("Instalando {}", packages[i]);
            let mut child = Command::new(pipa)
                .arg("install")
                .arg(packages[i].as_str().unwrap())
                .spawn()
                .expect("No tenis el pip?");
            let _result = child.wait().unwrap();
        }
    } else {
        let pipa = "pip3";
        for i in 0..packages.len() {
            println!("Instalando {}", packages[i]);
            let mut child = Command::new(pipa)
                .arg("install")
                .arg(packages[i].as_str().unwrap())
                .spawn()
                .expect("No tenis el pip?");
            let _result = child.wait().unwrap();
        }
    }
    true
}

pub fn analized_pip(input: AdiPip, path: &str) {
    if input.requirements == true {
        let mut pat = String::from(path);
        pat.push_str(&input.file);
        let confir = install_pip(input.version, &pat);
        if confir == true {
            println!("Instalacion de pip terminada con exito!");
        } else {
            println!("{}", "Algo salio mal instalando con pip".red());
            process::exit(0x0100);
        }
    } else {
        let conf = pip_pack(input.version, input.packages);
        if conf == true {
            println!("Instalacion con pip correcta!");
        } else {
            println!("{}", "Algo salio mal instalando con pip".red());
            process::exit(0x0100);
        }
    }
}

pub fn analized_gem(input: AdiGem, path: &str) {
    if input.gemfile == true {
        let mut paa = String::new();
        paa.push_str(path);
        paa.push_str(&input.file);
        let conf = install_bundle(&paa);

        if conf == true {
            println!("La instalacion se ha realizado con exito");
        } else {
            println!("{}", "Algo salio mal instalando con bundler".red());
            process::exit(0x0100);
        }
    } else {
        let conf = install_gem(input.gemas);
        if conf == true {
            println!("Se termino la instalacion de las gemas!!");
        } else {
            println!("{}", "Algo salio mal instalando las gemas".red());
            process::exit(0x0100);
        }
    }
}
