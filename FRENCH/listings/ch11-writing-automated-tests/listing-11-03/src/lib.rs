// ANCHOR: here
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn un_autre() {
        panic!("Fait échouer ce test");
    }
}
// ANCHOR_END: here
