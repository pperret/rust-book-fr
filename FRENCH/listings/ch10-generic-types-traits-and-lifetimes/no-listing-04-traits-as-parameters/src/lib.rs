pub trait Resumable {
    fn resumer(&self) -> String;
}

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {
    fn resumer(&self) -> String {
        format!("{}, par {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

impl Resumable for Tweet {
    fn resumer_auteur(&self) -> String {
        format!("@{}", self.nom_utilisateur)
    }
}

// ANCHOR: here
pub fn notifier(element: impl Summary) {
    println!("Flash-info ! {}", element.resumer());
}
// ANCHOR_END: here
