fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut points = HashMap::new();

    points.insert(String::from("Bleu"), 10);
    points.insert(String::from("Jaune"), 50);

    for (clee, valeur) in &points {
        println!("{} : {}", clee, valeur);
    }
    // ANCHOR_END: here
}
