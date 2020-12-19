enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ANCHOR: here
fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => {
            println!("Un centime porte-bonheur !");
            1
        }
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
