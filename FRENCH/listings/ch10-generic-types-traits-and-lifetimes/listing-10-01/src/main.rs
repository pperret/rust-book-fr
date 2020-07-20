// ANCHOR: here
fn main() {
    let liste_de_nombres = vec![34, 50, 25, 100, 65];

    let mut le_plus_gros = liste_de_nombres[0];

    for nombre in liste_de_nombres {
        if nombre > le_plus_gros {
            le_plus_gros = nombre;
        }
    }

    println!("Le nombre le plus gros est {}", le_plus_gros);
    // ANCHOR_END: here
    assert_eq!(le_plus_gros, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
