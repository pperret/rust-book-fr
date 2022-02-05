use std::ops::Add;

struct Millimetres(u32);
struct Metres(u32);

impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, other: Metres) -> Millimetres {
        Millimetres(self.0 + (other.0 * 1000))
    }
}
