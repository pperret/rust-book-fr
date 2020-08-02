fn main() {
    let mut s = String::from("hello");

    changer(&mut s);
}

fn changer(texte: &mut String) {
    texte.push_str(", world");
}
