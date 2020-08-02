fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("La valeur de y est : {}", y);
}
