fn main() {
  let s = String::from("hello");  // s rentre dans la portée.

  prendre_possession(s);  // La valeur de s est déplacée dans la fonction…
                          // … et n'est plus en vigueur à partir d'ici

  let x = 5;              // x rentre dans la portée.

  creer_copie(x);         // x va être déplacée dans la fonction,
                          // mais i32 est Copy, donc on peut
                          // utiliser x ensuite.

} // Ici, x sort de la portée, puis ensuite s. Mais puisque la valeur de s a
// été déplacée, il ne se passe rien de spécial.

fn prendre_possession(texte: String) { // texte rentre dans la portée.
  println!("{}", texte);
} // Ici, texte sort de la portée et `drop` est appelé. La mémoire est libérée.

fn creer_copie(entier: i32) { // entier rentre dans la portée.
  println!("{}", entier);
} // Ici, entier sort de la portée. Il ne se passe rien de spécial.
