// ANCHOR: here
use std::fmt::Result;
use std::io::Result as IoResult;

fn fonction1() -> Result {
    // -- partie masquée ici --
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn fonction2() -> IoResult<()> {
    // -- partie masquée ici --
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
