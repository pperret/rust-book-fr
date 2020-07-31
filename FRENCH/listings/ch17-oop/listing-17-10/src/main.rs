use gui::Ecran;

fn main() {
    let ecran = Ecran {
        composants: vec![Box::new(String::from("Salut"))],
    };

    ecran.executer();
}
