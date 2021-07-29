use crate::{archivos, estructuras::Adi};

#[test]
fn descarga_test() {
    let testeo = archivos::descarga(
        "https://raw.githubusercontent.com/Kedap/apmpkg/main/ejemplos/nspawn.adi",
        "testdir/test_descarga.adi",
    );
    match testeo {
        Err(e) => panic!("fallo el test de descarga: {}", e),
        _ => {}
    }
}

#[test]
fn leer_adi_test() {
    Adi::nuevo("testdir/nspawn.adi");
}

#[test]
fn extraer_tar_test() {
    let testeo = archivos::extraer_tar("testdir/test-tar.tar.gz", "testdir/extraer_tar.d");
    match testeo {
        Err(e) => panic!("fallo al test de extraer tar: {}", e),
        _ => {}
    }
}
