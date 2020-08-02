fn main() {
    let mut compteur = 0;

    let resultat = loop {
        compteur += 1;

        if compteur == 10 {
            break compteur * 2;
        }
    };

    println!("Le résultat est {}", resultat);
}
