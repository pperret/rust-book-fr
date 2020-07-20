struct ExtraitImportant<'a> {
    partie: &'a str,
}

fn main() {
    let roman = String::from("Appelez-moi Ismaël. Il y a quelques années ...");
    let premiere_phrase = roman.split('.')
        .next()
        .expect("Impossible de trouver un '.'");
    let i = ExtraitImportant { partie: premiere_phrase };
}
