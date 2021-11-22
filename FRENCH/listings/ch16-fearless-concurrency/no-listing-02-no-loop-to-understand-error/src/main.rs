use std::sync::Mutex;
use std::thread;

fn main() {
    let compteurs = Mutex::new(0);
    let mut manipulateurs = vec![];

    let manipulateur = thread::spawn(move || {
        let mut nombre = compteurs.lock().unwrap();

        *nombre += 1;
    });
    manipulateurs.push(manipulateur);

    let manipulateur2 = thread::spawn(move || {
        let mut nombre2 = compteurs.lock().unwrap();

        *nombre2 += 1;
    });
    manipulateurs.push(manipulateur2);

    for manipulateur in manipulateurs {
        manipulateur.join().unwrap();
    }

    println!("Resultat : {}", *compteurs.lock().unwrap());
}
