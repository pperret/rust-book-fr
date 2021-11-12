struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

fn main() {
    let utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };

    // ANCHOR: here
    let utilisateur2 = Utilisateur {
        email: String::from("quelquundautre@example.com"),
        ..utilisateur1
    };
    // ANCHOR_END: here
}
