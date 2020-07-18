fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut points = HashMap::new();
    points.insert(String::from("Bleu"), 10);

    points.entry(String::from("Jaune")).or_insert(50);
    points.entry(String::from("Bleu")).or_insert(50);

    println!("{:?}", points);
    // ANCHOR_END: here
}
