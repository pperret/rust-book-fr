fn main() {
    enum Message {
        Quitter,
        Deplacer { x: i32, y: i32 },
        Ecrire(String),
        ChangerCouleur(i32, i32, i32),
    }

    // ANCHOR: here
    impl Message {
        fn appeler(&self) {
            // le corps de la méthode sera défini ici
        }
    }
    
    let m = Message::Ecrire(String::from("hello"));
    m.appeler();
    // ANCHOR_END: here
}
