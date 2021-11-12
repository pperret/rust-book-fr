#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

// ANCHOR: here
impl Rectangle {
    fn largeur(&self) -> bool {
        self.largeur > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        largeur: 30,
        hauteur: 50,
    };

    if rect1.largeur() {
        println!("Le rectangle a une largeur non nulle ; elle vaut {}", rect1.largeur);
    }
}
// ANCHOR_END: here
