//use std::io::{stdout, Write};
//use std::fs;
//use curl::easy::Easy;
//use core::any::type_name;
//use std::process::Command;
//use std::fs::File;
//use std::io::prelude::*;
use pbr::ProgressBar;
use std::thread;

fn main() {
    let count = 10;
    let mut pb = ProgressBar::new(count);
    pb.format("(->.)");
    for _ in 0..count {
        pb.inc();
        thread::sleep_ms(1);
    }
    //pb.finish_print("done");
}

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