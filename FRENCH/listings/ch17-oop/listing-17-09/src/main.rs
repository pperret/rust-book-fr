use gui::Affichable;

struct ListeDeroulante {
    largeur: u32,
    hauteur: u32,
    options: Vec<String>,
}

impl Affichable for ListeDeroulante {
    fn afficher(&self) {
        // code servant vraiment à afficher une liste déroulante
    }
}

// ANCHOR: here
use gui::{Bouton, Ecran};

fn main() {
    let ecran = Ecran {
        composants: vec![
            Box::new(ListeDeroulante {
                largeur: 75,
                hauteur: 10,
                options: vec![
                    String::from("Oui"),
                    String::from("Peut-être"),
                    String::from("Non"),
                ],
            }),
            Box::new(Bouton {
                largeur: 50,
                hauteur: 10,
                libelle: String::from("OK"),
            }),
        ],
    };

    ecran.executer();
}
// ANCHOR_END: here
