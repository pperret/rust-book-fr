fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultat = la_plus_longue(string1.as_str(), string2);
    println!("La plus grande chaîne est {}", resultat);
}
