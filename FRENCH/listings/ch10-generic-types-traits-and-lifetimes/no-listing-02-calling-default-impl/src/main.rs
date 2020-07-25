use chapter10::{self, ArticleDePresse, Resumable};

fn main() {
    // ANCHOR: here
    let article = ArticleDePresse {
        titre: String::from("Les Pinguins ont gagné la Stanley Cup Championship !"),
        lieu: String::from("Pittsburgh, PA, USA"),
        auteur: String::from("Iceburgh"),
        contenu: String::from(
            "Les Pinguins de Pittsburgh sont une nouvelle fois la meilleure\
            équipe de hockey de la NHL."
        ),
    };
    
    println!("Nouvel article disponible ! {}", article.resumer());
    // ANCHOR_END: here
}
