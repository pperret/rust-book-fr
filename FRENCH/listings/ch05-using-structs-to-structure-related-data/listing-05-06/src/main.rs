struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

// ANCHOR: here
fn main() {
    // -- partie masquée ici --
    // ANCHOR_END: here

    let utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };
    // ANCHOR: here

    let utilisateur2 = Utilisateur {
        actif: utilisateur1.actif,
        pseudo: utilisateur1.email,
        email: String::from("quelquundautre@example.com"),
        nombre_de_connexions: utilisateur1.nombre_de_connexions,
    };
}
// ANCHOR_END: here
