use addition;

mod common;

#[test]
fn cela_ajoute_deux() {
    common::parametrage();
    assert_eq!(4, addition::ajouter_deux(2));
}
