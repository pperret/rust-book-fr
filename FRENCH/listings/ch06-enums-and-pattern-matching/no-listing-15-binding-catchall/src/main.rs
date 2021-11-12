fn main() {
    // ANCHOR: here
    let jete_de_de = 9;
    match jete_de_de {
        3 => ajouter_chapeau_fantaisie(),
        7 => enleve_chapeau_fantaisie(),
        autre => deplace_joueur(autre),
    }

    fn ajouter_chapeau_fantaisie() {}
    fn enleve_chapeau_fantaisie() {}
    fn deplace_joueur(nombre_cases: u8) {}
    // ANCHOR_END: here
}
