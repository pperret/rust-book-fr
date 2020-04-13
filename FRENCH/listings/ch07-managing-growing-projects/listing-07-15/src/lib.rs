// ANCHOR: here
use std::fmt;
use std::io;

fn fonction1() -> fmt::Result {
    // -- partie masquée ici --
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn fonction2() -> io::Result<()> {
    // -- partie masquée ici --
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
