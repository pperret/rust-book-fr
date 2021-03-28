fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let nom_champ = String::from("Couleur favorite");
    let valeur_champ = String::from("Bleu");

    let mut table = HashMap::new();
    table.insert(nom_champ, valeur_champ);
    // nom_champ et valeur_champ ne sont plus en vigueur à partir d'ici,
    // essayez de les utiliser et vous verrez l'erreur du compilateur que
    // vous obtiendrez !
    // ANCHOR_END: here
}
