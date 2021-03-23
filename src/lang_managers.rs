// Manejador como bundle o pip

//uses
use {
	std::process::Command};


pub fn install_gem(path: &str) {
	let mut gem = String::from("--gemfile="); gem.push_str(path);
    let mut child = Command::new("bundle")
                .arg("install")
                .arg(gem)
                .spawn()
                .expect("No tenis el bundle");
    let _result = child.wait().unwrap();
}