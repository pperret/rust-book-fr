struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let ne_fonctionnera_pas = Point { x: 5, y: 4.0 };
}
