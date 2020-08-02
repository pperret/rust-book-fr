// ANCHOR: here
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}
// ANCHOR_END: here

fn main() {}
