struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(&rect1)
    );
}

fn aire(rectangle: &Rectangle) -> u32 {
    rectangle.largeur * rectangle.hauteur
}
