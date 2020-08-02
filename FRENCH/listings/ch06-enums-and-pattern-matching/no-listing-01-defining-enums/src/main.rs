// ANCHOR: def
enum SorteAdresseIp {
    V4,
    V6,
}
// ANCHOR_END: def

fn main() {
    // ANCHOR: instance
    let quatre = SorteAdresseIp::V4;
    let six = SorteAdresseIp::V6;
    // ANCHOR_END: instance

    // ANCHOR: fn_call
    router(SorteAdresseIp::V4);
    router(SorteAdresseIp::V6);
    // ANCHOR_END: fn_call
}

// ANCHOR: fn
fn router(sorte_ip: SorteAdresseIp) { }
// ANCHOR_END: fn
