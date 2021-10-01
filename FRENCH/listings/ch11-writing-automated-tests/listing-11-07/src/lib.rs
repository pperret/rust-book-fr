pub fn ajouter_deux(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cela_ajoute_deux() {
        assert_eq!(4, ajouter_deux(2));
    }
}
