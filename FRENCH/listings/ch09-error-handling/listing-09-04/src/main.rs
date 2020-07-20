use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(fichier) => fichier,
        Err(erreur) => panic!("Erreur lors de l'ouverture du fichier : {:?}", erreur),
    };
}
