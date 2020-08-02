fn main() {
    // ANCHOR: here
    let une_valeur_u8 = 0u8;
    match une_valeur_u8 {
        1 => println!("un"),
        3 => println!("trois"),
        5 => println!("cinq"),
        7 => println!("sept"),
        _ => (),
    }
    // ANCHOR_END: here
}
