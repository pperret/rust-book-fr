// ANCHOR: here
#[derive(Debug)] // pour pouvoir afficher l'État
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
// ANCHOR_END: here

fn main() {}
