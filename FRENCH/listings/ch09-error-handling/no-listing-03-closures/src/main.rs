use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|erreur| {
        if erreur.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|erreur| {
                panic!("Erreur de création du fichier : {:?}", erreur);
            })
        } else {
            panic!("Erreur d'ouverture du fichier : {:?}", erreur);
        }
    });
}
