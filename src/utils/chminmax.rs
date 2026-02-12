pub trait ChMinMax: PartialOrd + Sized {
    fn chmin(&mut self, rhs: Self) -> bool {
        if rhs < *self {
            *self = rhs;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, rhs: Self) -> bool {
        if rhs > *self {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
impl<T: PartialOrd> ChMinMax for T {}

