fn main() {
    // ANCHOR: here
    fn plus_un(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // ANCHOR_END: here

    let cinq = Some(5);
    let six = plus_un(cinq);
    let none = plus_un(None);
}
