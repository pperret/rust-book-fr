pub fn ajouter_deux(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajouter_deux_a_deux() {
        assert_eq!(4, ajouter_deux(2));
    }

    #[test]
    fn ajouter_deux_a_trois() {
        assert_eq!(5, ajouter_deux(3));
    }

    #[test]
    fn cent() {
        assert_eq!(102, ajouter_deux(100));
    }
}
