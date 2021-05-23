// ANCHOR: here
use std::fs::File;
use std::io;
use std::io::Read;

fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(fichier) => fichier,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let pseudo =
        lire_pseudo_depuis_fichier().expect("Échec de lecture du pseudo");
}
