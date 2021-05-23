// ANCHOR: here
use std::fs::File;
use std::io;
use std::io::Read;

fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// ANCHOR_END: here

fn main() {
    let pseudo =
        lire_pseudo_depuis_fichier().expect("Échec de lecture du pseudo");
}
