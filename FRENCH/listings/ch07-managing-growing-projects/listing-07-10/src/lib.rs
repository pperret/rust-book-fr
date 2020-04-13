mod cuisines {
    pub enum AmuseBouche {
        Soupe,
        Salade,
    }
}

pub fn manger_au_restaurant() {
    let commande1 = cuisines::AmuseBouche::Soupe;
    let commande2 = cuisines::AmuseBouche::Salade;
}
