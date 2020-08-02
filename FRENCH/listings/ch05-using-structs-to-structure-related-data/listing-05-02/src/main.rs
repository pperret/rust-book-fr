struct Utilisateur {
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
    actif: bool,
}

fn main() {
    // ANCHOR: here
    let utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };
    // ANCHOR_END: here
}
