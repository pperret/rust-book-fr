fn premier_mot(s: &String) -> usize {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let mot = premier_mot(&s); // la variable mot aura 5 comme valeur.

    s.clear(); // ceci vide la String, elle vaut maintenant "".

    // mot a toujours la valeur 5 ici, mais il n'y a plus de chaîne qui donne
    // du sens à la valeur 5. mot est maintenant complètement invalide !
}
// ANCHOR_END: here
