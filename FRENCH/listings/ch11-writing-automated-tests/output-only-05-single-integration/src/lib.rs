pub fn ajouter_deux(a: i32) -> i32 {
    addition_interne(a, 2)
}

fn addition_interne(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interne() {
        assert_eq!(4, addition_interne(2, 2));
    }
}
