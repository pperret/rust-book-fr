fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut points = HashMap::new();
    
    points.insert(String::from("Bleu"), 10);
    points.insert(String::from("Bleu"), 25);
    
    println!("{:?}", points);
    // ANCHOR_END: here
}
