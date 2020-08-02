fn main() {
    let reference_vers_rien = pendouille();
}

// ANCHOR: here
fn pendouille() -> &String { // pendouille retourne une référence vers une String

  let s = String::from("hello"); // s est une nouvelle String

  &s // nous retournons une référence vers la String, s
} // Ici, s sort de la portée, et est libéré. Sa mémoire disparaît.
  // Attention, danger !
// ANCHOR_END: here
