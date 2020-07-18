fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut points = HashMap::new();

    points.insert(String::from("Bleu"), 10);
    points.insert(String::from("Jaune"), 50);

    let nom_equipe = String::from("Bleu");
    let points_bleu = points.get(&nom_equipe);
    // ANCHOR_END: here
}
