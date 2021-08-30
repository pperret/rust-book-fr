use std::fmt::Display;

struct Paire<T> {
    x: T,
    y: T,
}

impl<T> Paire<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Paire<T> {
    fn afficher_comparaison(&self) {
        if self.x >= self.y {
            println!("Le plus grand élément est x = {}", self.x);
        } else {
            println!("Le plus grand élément est y = {}", self.y);
        }
    }
}
