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

pub struct Bouton {
    pub largeur: u32,
    pub hauteur: u32,
    pub libelle: String,
}

impl Affichable for Bouton {
    fn afficher(&self) {
        // code servant vraiment à afficher un bouton
    }
}
