// ANCHOR: here
fn dernier_caractere_de_la_premiere_ligne(texte: &str) -> Option<char> {
    texte.lines().next()?.chars().last()
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        dernier_caractere_de_la_premiere_ligne("Et bonjour\nComment ca va, aujourd'hui ?"),
        Some('r')
    );

    assert_eq!(dernier_caractere_de_la_premiere_ligne(""), None);
    assert_eq!(dernier_caractere_de_la_premiere_ligne("\nsalut"), None);
}
