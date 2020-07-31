fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // s est en vigueur à partir de ce point
    
        // on fait des choses avec s ici
    }                                  // cette portée est désormais terminée, et s
                                       // n'est plus en vigueur maintenant
    // ANCHOR_END: here
}
