// ANCHOR: here
pub trait Resumable {
    fn resumer_auteur(&self) -> String;

    fn resumer(&self) -> String {
        format!("(Lire plus d'éléments de {} ...)", self.resumer_auteur())
    }
}
// ANCHOR_END: here

pub struct Tweet {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

// ANCHOR: impl
impl Resumable for Tweet {
    fn resumer_auteur(&self) -> String {
        format!("@{}", self.nom_utilisateur)
    }
}
// ANCHOR_END: impl
