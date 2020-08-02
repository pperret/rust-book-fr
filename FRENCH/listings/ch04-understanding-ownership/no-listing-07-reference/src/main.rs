// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("hello");

    let long = calculer_taille(&s1);
    // ANCHOR_END: here

    println!("La taille de '{}' est {}.", s1, long);
}

fn calculer_taille(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
