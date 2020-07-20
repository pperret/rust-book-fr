pub trait Resumable {
    fn resumer(&self) -> String {
        String::from("(En savoir plus ...)")
    }
}

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {}

pub struct Tweet {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

impl Resumable for Tweet {
    fn resumer(&self) -> String {
        format!("{} : {}", self.nom_utilisateur, self.contenu)
    }
}
