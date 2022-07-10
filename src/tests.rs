use {
    crate::{archivos, estructuras::Adi},
    temp_dir::TempDir,
};

#[test]
fn descarga_test() {
    let d = TempDir::new().unwrap();
    let path = d.child("nspawn.adi");
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
    let d = TempDir::new().unwrap();
    let path = d.child("tar_extraido/");
    let _testeo = archivos::extraer_tar("testdir/test-tar.tar.gz", path.to_str().unwrap()).unwrap();
    d.cleanup().unwrap();
}

#[test]
fn clono_test() {
    let d = TempDir::new().unwrap();
    let path = d.child("dotfiles/");
    let _testeo = archivos::git_clone(
        "https://github.com/Kedap/dotfiles.git",
        path.to_str().unwrap(),
    )
    .unwrap();
    d.cleanup().unwrap();
}
