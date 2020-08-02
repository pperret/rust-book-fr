// ANCHOR: all
fn main() {
    let largeur1 = 30;
    let hauteur1 = 50;

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(largeur1, hauteur1)
    );
}

// ANCHOR: here
fn aire(largeur: u32, hauteur: u32) -> u32 {
    // ANCHOR_END: here
    largeur * hauteur
}
// ANCHOR_END: all
