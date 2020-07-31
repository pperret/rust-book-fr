pub trait Affichable {
    fn afficher(&self);
}

// ANCHOR: here
pub struct Ecran {
    pub composants: Vec<Box<dyn Affichable>>,
}
// ANCHOR_END: here
