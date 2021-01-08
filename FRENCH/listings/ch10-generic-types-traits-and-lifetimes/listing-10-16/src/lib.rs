use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn affiche_comparaison(&self) {
        if self.x >= self.y {
            println!("Le plus grand élément est x = {}", self.x);
        } else {
            println!("Le plus grand élément est y = {}", self.y);
        }
    }
}
