use std::io::{stdout, Write};
use std::fs;
use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    easy.url("https://github.com/Kedap/makechine").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
    nuevo();
}

fn nuevo() {
	println!("file contains {}", "testing.tap");

        let filedata = fs::read_to_string("testing.tap")
            .expect("Archivo no encontrado!!! ");

        println!("content of file sample data:\n{}", filedata);
}