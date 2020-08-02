fn main() {
    // ANCHOR: here
    fn plus_un(x: Option<i32>) -> Option<i32> {
        match x {
            // ANCHOR: first_arm
            None => None,
            // ANCHOR_END: first_arm
            // ANCHOR: second_arm
            Some(i) => Some(i + 1),
            // ANCHOR_END: second_arm
        }
    }

    let cinq = Some(5);
    let six = plus_un(cinq);
    let none = plus_un(None);
    // ANCHOR_END: here
}
