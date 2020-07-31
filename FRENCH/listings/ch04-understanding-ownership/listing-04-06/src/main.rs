fn main() {
    let s = String::from("hello");

    changer(&s);
}

fn changer(texte: &String) {
    texte.push_str(", world");
}
