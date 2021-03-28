fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Bleu"), 10);
    scores.insert(String::from("Jaune"), 50);

    let nom_equipe = String::from("Bleu");
    let score = scores.get(&nom_equipe);
    // ANCHOR_END: here
}
