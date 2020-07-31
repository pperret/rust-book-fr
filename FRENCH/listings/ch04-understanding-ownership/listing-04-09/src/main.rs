// ANCHOR: here
fn premier_mot(s: &str) -> &str {
    // ANCHOR_END: here
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let ma_string = String::from("hello world");

    // premier_mot fonctionne avec les slices de `String`
    let mot = premier_mot(&ma_string[..]);

    let mon_litteral_de_chaine = "hello world";

    // premier_mot fonctionne avec les slices de littéraux de chaîne
    let mot = premier_mot(&mon_litteral_de_chaine[..]);

    // Comme les littéraux de chaîne *sont* déjà des slices de chaînes,
    // cela fonctionne aussi, sans la syntaxe de slice !
    let mot = premier_mot(mon_litteral_de_chaine);
}
// ANCHOR_END: usage
