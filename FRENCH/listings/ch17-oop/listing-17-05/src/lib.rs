pub trait Affichable {
    fn afficher(&self);
}

pub struct Ecran {
    pub composants: Vec<Box<dyn Affichable>>,
}

// ANCHOR: here
impl Ecran {
    pub fn executer(&self) {
        for composant in self.composants.iter() {
            composant.afficher();
        }
    }
}
// ANCHOR_END: here
