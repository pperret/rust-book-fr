fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Bleu"), 10);
    scores.insert(String::from("Bleu"), 25);

    println!("{:?}", scores);
    // ANCHOR_END: here
}
