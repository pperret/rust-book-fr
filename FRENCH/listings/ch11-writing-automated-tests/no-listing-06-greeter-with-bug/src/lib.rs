// ANCHOR: here
pub fn accueil(name: &str) -> String {
    String::from("Salut !")
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accueil_contient_le_nom() {
        let resultat = accueil("Carole");
        assert!(resultat.contains("Carole"));
    }
}
