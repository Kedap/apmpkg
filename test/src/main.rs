//use std::io::{stdout, Write};
//use std::fs;
//use curl::easy::Easy;
//use core::any::type_name;
use std::process::Command;
//use std::fs::File;
//use std::io::prelude::*;
//use pbr::ProgressBar;
//use std::thread;
//use std::fs::File;
//use flate2::read::GzDecoder;
////use tar::Archive;
//use sha2::Digest;
//use sha2::Sha256;
//use std::fs::File;
//use std::io::copy;

fn main() {
    let path = "aaa/Tool-AC-Beta/ConsolaV/Gemfile";
    let mut gem = String::from("--gemfile="); gem.push_str(path);
    Command::new("bundle")
                .arg("install")
                .arg(gem)
                .spawn()
                .expect("No tenis el bundle");
}

//fn main() {
//    let mut file = File::open("Beta.tar.gz").expect("Un error al leer");
//    let mut sha256 = Sha256::new();
//    copy(&mut file, &mut sha256).expect("Error en el io");
//    let hash0 = sha256.finalize();
//    let fhash = format!("{:x}", hash0);
//    println!("{:?}", fhash);
//}

//fn main() {
//    let mut file = File::open("Beta.tar.gz").expect("Error al leer el file");
//    let mut sh = Sha256::new();
//    copy(&mut file, &mut sh).expect("Eror 0");
//    let cast = sh.result();
//    println!("{:x}", sh);
//}



//fn main() -> Result<(), std::io::Error> {
//    let path = "Beta.tar.gz";
//
//    let tar_gz = File::open(path)?;
//    let tar = GzDecoder::new(tar_gz);
//    let mut archive = Archive::new(tar);
//
//    for (_i, file) in archive.entries().unwrap().enumerate() {
//        let mut file = file.unwrap();
//        file.unpack_in("aaa/").unwrap();
//    }
//
//    Ok(())
//}

//
//fn main() {
//    let count = 10;
//    let mut pb = ProgressBar::new(count);
//    pb.format("(->.)");
//    for _ in 0..count {
//        pb.inc();
//        thread::sleep_ms(1);
//    }
//    //pb.finish_print("done");
//}

//#[tokio::main]
//async fn peti(url: &str) -> Result<(), Box<dyn std::error::Error>> {
//    let body = reqwest::get(url)
//    .await?
//    .text()
//    .await?;
//    let mut salida = File::create("toac.tap").expect("Eror al crear el archivo");
//    salida.write_all(body.as_bytes())?;
//    Ok(())
//}
//fn main() {
//    println!("Iniciando....");
//    let http = "https://raw.githubusercontent.com/y4ot3t1/Tool-AC/master/.gitignore";
//    let _ = peti(http);
//}
//fn type_of<T>(_: T) -> &'static str {
//    type_name::<T>()
//}
//
//fn main() {
//    println!("Holaa");
//    peti();
//    //let output = Command::new("pacman")
    //                 .arg("-S")
    //                 .arg("ruby findomain")
    //                 .output()
    //                 .expect("failed to execute process");
    //if output.status.to_string() != "exit code: 1" {
    //    println!("Algo fallo");
    //}
//
//    //println!("status: {}", output.status);
//    //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    //let mut easy = Easy::new();
    //easy.url("https://github.com/Kedap/makechine").unwrap();
    //easy.write_function(|data| {
    //    stdout().write_all(data).unwrap();
    //    Ok(data.len())
    //}).unwrap();
    //easy.perform().unwrap();
    //println!("{}", easy.response_code().unwrap());
    //nuevo();
//}

//fn nuevo() {
//	println!("file contains {}", "testing.tap");
//
//        let filedata = fs::read_to_string("testing.tap")
//            .expect("Archivo no encontrado!!! ");
//
//        println!("content of file sample data:\n{}", filedata);
//}