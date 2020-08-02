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
    if let USACoin::Quarter(etat) = piece {
        println!("Il s'agit d'un Quarter de l'état de {:?}!", etat);
    } else {
        compteur += 1;
    }
    // ANCHOR_END: here
}
