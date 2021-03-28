fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let equipes = vec![String::from("Bleu"), String::from("Jaune")];
    let scores_initiaux = vec![10, 50];

    let mut scores: HashMap<_, _> =
        equipes.into_iter().zip(scores_initiaux.into_iter()).collect();
    // ANCHOR_END: here
}
