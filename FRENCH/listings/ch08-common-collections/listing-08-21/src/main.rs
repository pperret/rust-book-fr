fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let equipes = vec![String::from("Bleu"), String::from("Jaune")];
    let points_initiaux = vec![10, 50];

    let mut points: HashMap<_, _> =
        equipes.into_iter().zip(points_initiaux.into_iter()).collect();
    // ANCHOR_END: here
}
