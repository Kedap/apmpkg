//use std::io::{stdout, Write};
//use std::fs;
//use curl::easy::Easy;
//use core::any::type_name;
//use std::process::Command;


#[tokio::main]
async fn peti(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = reqwest::get(url)
    .await?
    .text()
    .await?;
    Ok(())
}

fn main() {
    println!("Iniciando....");
    let http = "https://raw.githubusercontent.com/y4ot3t1/Tool-AC/master/.gitignore";
    let lol = peti(http);
    println!("{:?}", lol);

}
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