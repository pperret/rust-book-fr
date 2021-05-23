// ANCHOR: here
use std::fs;
use std::io;

fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
// ANCHOR_END: here

fn main() {
    let pseudo =
        lire_pseudo_depuis_fichier().expect("Échec de lecture du pseudo");
}
