struct Utilisateur {
    pseudo: &str,
    email: &str,
    nombre_de_connexions: u64,
    actif: bool,
}

fn main() {
    let user1 = User {
        email: "quelquun@example.com",
        pseudo: "pseudoquelconque123",
        actif: true,
        nombre_de_connexions: 1,
    };
}
