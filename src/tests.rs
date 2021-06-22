use crate::archivos;

#[test]
fn download_test() {
    let testa = archivos::download(
        "https://raw.githubusercontent.com/Kedap/apmpkg/main/ejemplos/nspawn.adi",
        "testdir/test.adi",
    );
    match testa {
        Err(e) => {
            panic!("fallo el test de descarga {}", e)
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
        Err(e) => {
            panic!("fallo extraer el archivo testdir/test-tar.tar.gz {}", e)
        }
        _ => println!("de pana"),
    }
}
