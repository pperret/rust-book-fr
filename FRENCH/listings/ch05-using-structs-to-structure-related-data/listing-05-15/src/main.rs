#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

// ANCHOR: here
impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }

    fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur > autre.largeur && self.hauteur > autre.hauteur
    }
}
// ANCHOR_END: here

fn main() {
    let rect1 = Rectangle {
        largeur: 30,
        hauteur: 50
    };
    let rect2 = Rectangle {
        largeur: 10,
        hauteur: 40
    };
    let rect3 = Rectangle {
        largeur: 60,
        hauteur: 45
    };

    println!("rect1 peut-il contenir rect2 ? {}", rect1.peut_contenir(&rect2));
    println!("rect1 peut-il contenir rect3 ? {}", rect1.peut_contenir(&rect3));
}
