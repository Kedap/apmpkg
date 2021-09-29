use {
    crate::{archivos, estructuras::Adi},
    std::path::PathBuf,
    testdir::testdir,
};

#[test]
fn descarga_test() {
    let dir: PathBuf = testdir!();
    let path = dir.join("nspawn.adi");
    let _testeo = archivos::descarga(
        "https://raw.githubusercontent.com/Kedap/apmpkg/main/ejemplos/nspawn.adi",
        path.to_str().unwrap(),
    )
    .unwrap();
}

#[test]
fn leer_adi_test() {
    Adi::nuevo("testdir/nspawn.adi");
}

#[test]
fn extraer_tar_test() {
    let dir: PathBuf = testdir!();
    let path = dir.join("tar_extraido/");
    let _testeo = archivos::extraer_tar("testdir/test-tar.tar.gz", path.to_str().unwrap()).unwrap();
}
