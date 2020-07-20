struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let deux_entiers = Point { x: 5, y: 10 };
    let deux_flottants = Point { x: 1.0, y: 4.0 };
    let un_entier_et_un_flottant = Point { x: 5, y: 4.0 };
}
