#[derive(Debug)]
enum USAState {
    Alabama,
    Alaska,
    // -- partie masquée ici --
}

enum USACoin {
    Penny,
    Nickel,
    Dime,
    Quarter(USAState),
}

fn main() {
    let piece = USACoin::Penny;
    // ANCHOR: here
    let mut compteur = 0;
    match piece {
        USACoin::Quarter(etat) => println!("Il s'agit d'un Quarter de l'état de {:?} !", etat),
        _ => compteur += 1,
    }
    // ANCHOR_END: here
}
