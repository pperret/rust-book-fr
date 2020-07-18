fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let texte = "bonjour le monde, magnifique monde";

    let mut tableau = HashMap::new();

    for mot in texte.split_whitespace() {
        let compteur = map.entry(mot).or_insert(0);
        *compteur += 1;
    }

    println!("{:?}", tableau);
    // ANCHOR_END: here
}
