// ANCHOR: here
fn premier_mot(s: &String) -> usize {
    // ANCHOR: as_bytes
    let octets = s.as_bytes();
    // ANCHOR_END: as_bytes

    // ANCHOR: iter
    for (i, &element) in octets.iter().enumerate() {
        // ANCHOR_END: iter
        // ANCHOR: inside_for
        if element == b' ' {
            return i;
        }
    }

    s.len()
    // ANCHOR_END: inside_for
}
// ANCHOR_END: here

fn main() {}
