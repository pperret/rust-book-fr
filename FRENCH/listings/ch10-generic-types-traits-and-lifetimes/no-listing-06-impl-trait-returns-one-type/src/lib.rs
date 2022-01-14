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
        format!("{}, par {} ({})", self.titre, self.auteur, self.lieu)
    }
}

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

// ANCHOR: here
fn retourne_resumable(estArticle: bool) -> impl Resumable {
    if estArticle {
        ArticleDePresse {
            titre: String::from("Les Penguins ont remporté la Coupe Stanley !"),
            lieu: String::from("Pittsburgh, PA, USA"),
            auteur: String::from("Iceburgh"),
            contenu: String::from("Les Penguins de Pittsburgh sont une nouvelle fois la \
            meilleure équipe de hockey de la LNH."),
        }
    } else {
        Tweet {
            nom_utilisateur: String::from("jean"),
            contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
            reponse: false,
            retweet: false,
        }
    }
}
// ANCHOR_END: here
