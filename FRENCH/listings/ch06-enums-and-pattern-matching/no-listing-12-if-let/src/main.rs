fn main() {
    // ANCHOR: here
    let une_valeur_u8 = Some(0u8);
    if let Some(3) = une_valeur_u8 {
        println!("trois");
    }
    // ANCHOR_END: here
}
