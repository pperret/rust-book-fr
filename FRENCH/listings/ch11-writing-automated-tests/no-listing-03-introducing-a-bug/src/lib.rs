#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

// ANCHOR: here
// -- partie masquée ici --
impl Rectangle {
    fn peut_contenir(&self, other: &Rectangle) -> bool {
        self.largeur < other.largeur && self.hauteur > other.hauteur
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_grand_peut_contenir_un_petit() {
        let le_grand = Rectangle {
            largeur: 8,
            hauteur: 7,
        };
        let le_petit = Rectangle {
            largeur: 5,
            hauteur: 1,
        };

        assert!(le_grand.peut_contenir(&le_petit));
    }

    #[test]
    fn un_petit_ne_peut_pas_contenir_un_plus_grand() {
        let le_grand = Rectangle {
            largeur: 8,
            hauteur: 7,
        };
        let le_petit = Rectangle {
            largeur: 5,
            hauteur: 1,
        };

        assert!(!le_petit.peut_contenir(&le_grand));
    }
}
