enum USACoin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ANCHOR: here
fn valeur_en_centimes(piece: USACoin) -> u8 {
    match piece {
        USACoin::Penny => {
            println!("Un centime porte-bonheur !");
            1
        },
        USACoin::Nickel => 5,
        USACoin::Dime => 10,
        USACoin::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
