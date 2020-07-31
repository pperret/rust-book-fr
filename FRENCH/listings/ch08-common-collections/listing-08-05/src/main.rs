fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let troisieme: &i32 = &v[2];
    println!("Le troisième élément est {}", troisieme);

    match v.get(2) {
        Some(troisieme) => println!("Le troisième élément est {}", troisieme),
        None => println!("Il n'y a pas de troisième élément."),
    }
    // ANCHOR_END: here
}
