pub trait Affichable {
    fn afficher(&self);
}

pub struct Ecran {
    pub composants: Vec<Box<dyn Affichable>>,
}

impl Ecran {
    pub fn executer(&self) {
        for composant in self.composants.iter() {
            composant.afficher();
        }
    }
}

// ANCHOR: here
pub struct Bouton {
    pub largeur: u32,
    pub hauteur: u32,
    pub libelle: String,
}

impl Affichable for Bouton {
    fn afficher(&self) {
        // code servant à afficher vraiment un bouton
    }
}
// ANCHOR_END: here
