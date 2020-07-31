pub struct CollectionMoyennee {
    liste: Vec<i32>,
    moyenne: f64,
}

// ANCHOR: here
impl CollectionMoyennee {
    pub fn ajouter(&mut self, valeur: i32) {
        self.liste.push(valeur);
        self.mettre_a_jour_moyenne();
    }

    pub fn retirer(&mut self) -> Option<i32> {
        let resultat = self.liste.pop();
        match resultat {
            Some(valeur) => {
                self.mettre_a_jour_moyenne();
                Some(valeur)
            }
            None => None,
        }
    }

    pub fn moyenne(&self) -> f64 {
        self.moyenne
    }

    fn mettre_a_jour_moyenne(&mut self) {
        let total: i32 = self.liste.iter().sum();
        self.moyenne = total as f64 / self.liste.len() as f64;
    }
}
// ANCHOR_END: here
