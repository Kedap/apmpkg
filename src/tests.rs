use crate::archivos;

#[test]
fn suma_test() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn download_test() {
    let testa = archivos::download(
        "https://raw.githubusercontent.com/Kedap/apmpkg/main/ejemplos/nspawn.adi",
        "testdir/test.adi",
    );
    match testa {
        Err(_e) => {
            panic!("fallo el test de descarga")
        }
        _ => println!("de pana"),
    }
}

#[test]
fn read_adi_test() {
    let file = archivos::read_fs("testdir/nspawn.adi");
    let _adi_file = archivos::read_adi(&file);
}

#[test]
fn e_tar_test() {
    let testar = archivos::e_tar("testdir/test-tar.tar.gz", "testdir/test-tar.d");
    match testar {
        Err(_e) => {
            panic!("fallo extraer el archivo testdir/test-tar.tar.gz")
        }
        _ => println!("de pana"),
    }
}
