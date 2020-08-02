fn main() {
    // ANCHOR: here
    enum SorteAdresseIp {
        V4,
        V6,
    }

    struct AdresseIp {
        sorte: SorteAdresseIp,
        adresse: String,
    }

    let local = AdresseIp {
        sorte: SorteAdresseIp::V4,
        adresse: String::from("127.0.0.1"),
    };
    
    let rebouclage = AdresseIp {
        sorte: SorteAdresseIp::V6,
        adresse: String::from("::1"),
    };
    // ANCHOR_END: here
}
