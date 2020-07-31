#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }
}

fn main() {
    let rect1 = Rectangle { largeur: 30, hauteur: 50 };

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        rect1.aire()
    );
}
