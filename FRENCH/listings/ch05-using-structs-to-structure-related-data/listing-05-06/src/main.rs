struct Utilisateur {
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
    actif: bool,
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
        pseudo: String::from("autrepseudo567"),
        actif: utilisateur1.actif,
        nombre_de_connexions: utilisateur1.nombre_de_connexions,
    };
    // ANCHOR_END: here
}
