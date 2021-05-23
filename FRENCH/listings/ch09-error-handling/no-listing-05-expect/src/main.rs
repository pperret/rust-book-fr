use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Échec à l'ouverture de hello.txt");
}
