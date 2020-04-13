fn main() {
    let rect1 = (30, 50);

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        aire(rect1)
    );
}

fn aire(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
