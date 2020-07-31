pub trait Affichable {
    fn afficher(&self);
}

// ANCHOR: here
pub struct Ecran<T: Affichable> {
    pub composants: Vec<T>,
}

impl<T> Ecran<T>
where
    T: Affichable,
{
    pub fn executer(&self) {
        for composant in self.composants.iter() {
            composant.afficher();
        }
    }
}
// ANCHOR_END: here
