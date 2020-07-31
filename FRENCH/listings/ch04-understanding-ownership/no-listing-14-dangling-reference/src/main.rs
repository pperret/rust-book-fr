fn main() {
    let reference_vers_rien = pendouille();
}

fn pendouille() -> &String {
    let s = String::from("hello");

    &s
}
