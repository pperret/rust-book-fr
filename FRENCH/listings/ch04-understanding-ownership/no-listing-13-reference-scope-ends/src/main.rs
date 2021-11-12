fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // sans problème
    let r2 = &s; // sans problème
    println!("{} et {}", r1, r2);
    //les variables r1 et r2 ne seront plus utilisés à partir d'ici
    
    let r3 = &mut s; // sans problème
    println!("{}", r3);
    // ANCHOR_END: here
}
