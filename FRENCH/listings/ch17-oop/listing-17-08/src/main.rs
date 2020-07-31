// ANCHOR: here
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
// ANCHOR_END: here

fn main() {}
