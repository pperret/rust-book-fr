fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 sort de la portée ici, donc nous pouvons créer une nouvelle référence
      // sans problèmes.

    let r2 = &mut s;
    // ANCHOR_END: here
}
