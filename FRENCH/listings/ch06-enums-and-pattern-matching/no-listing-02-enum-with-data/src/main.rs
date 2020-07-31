fn main() {
    // ANCHOR: here
    enum AdresseIp {
        V4(String),
        V6(String),
    }
    
    let local = AdresseIp::V4(String::from("127.0.0.1"));
    
    let rebouclage = AdresseIp::V6(String::from("::1"));
    // ANCHOR_END: here
}
