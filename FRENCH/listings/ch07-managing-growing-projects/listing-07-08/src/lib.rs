fn servir_commande() {}

mod cuisines {
    fn corriger_commande_erronee() {
        cuisiner_commande();
        super::servir_commande();
    }

    fn cuisiner_commande() {}
}
