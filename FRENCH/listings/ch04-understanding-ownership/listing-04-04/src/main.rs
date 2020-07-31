fn main() {
  let s1 = donne_possession();     // donne_possession déplace sa valeur de
                                   // retour dans s1

  let s2 = String::from("hello");  // s2 rentre dans la portée

  let s3 = prend_et_rend(s2);      // s2 est déplacée dans
                                   // prend_et_rend, qui elle aussi
                                   // déplace sa valeur de retour dans s3.
} // Ici, s3 sort de la portée et est éliminée. s2 sort de la portée mais a été
  // déplacée donc il ne se passe rien. s1 sort aussi de la portée et est
  // éliminée.

fn donne_possession() -> String {      // donne_possession va déplacer sa
                                       // valeur de retour dans la
                                       // fonction qui l'appelle.

  let texte = String::from("hello");   // texte rentre dans la portée.

  texte                                // texte est retournée et
                                       // est déplacée vers le code qui
                                       // l'appelle.
}

// prend_et_rend va prendre une String et en retourne aussi une.
fn prend_et_rend(texte: String) -> String { // texte rentre dans la portée.

  texte  // texte est retournée et déplacée vers le code qui l'appelle.
}
