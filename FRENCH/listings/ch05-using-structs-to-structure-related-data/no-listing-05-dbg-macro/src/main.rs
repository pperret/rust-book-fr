#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let echelle = 2;
    let rect1 = Rectangle {
        largeur: dbg!(30 * echelle),
        hauteur: 50,
    };

    dbg!(&rect1);
}
