fn main() {
    // ANCHOR: here
    let une_valeur_u8 = Some(0u8);
    match une_valeur_u8 {
        Some(3) => println!("trois"),
        _ => (),
    }
    // ANCHOR_END: here
}
