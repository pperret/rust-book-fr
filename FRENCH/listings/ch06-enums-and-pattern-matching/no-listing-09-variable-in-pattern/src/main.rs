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

// ANCHOR: here
fn valeur_en_centimes(piece: USACoin) -> u8 {
    match piece {
        USACoin::Penny => 1,
        USACoin::Nickel => 5,
        USACoin::Dime => 10,
        USACoin::Quarter(etat) => {
            println!("Il s'agit d'un Quarter de l'état de {:?} !", etat);
            25
        },
    }
}
// ANCHOR_END: here

fn main() {
    valeur_en_centimes(USACoin::Quarter(USAState::Alaska));
}
