fn main() {
    // ANCHOR: here
    let nombre = Some(4);

    match nombre {
        Some(x) if x % 2 == 0 => println!("Le nombre {} est pair", x),
        Some(x) => println!("Le nombre {} est impair", x),
        None => (),
    }
    // ANCHOR_END: here
}
