pub trait CollectVec: Iterator + Sized {
    fn collect_vec(self) -> Vec<Self::Item>;
}
impl<I: Iterator> CollectVec for I {
    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

pub trait Joiner {
    fn join_(self, sep: &str) -> String;
}

impl<T: ToString, I: IntoIterator<Item = T>> Joiner for I {
    fn join_(self, sep: &str) -> String {
        self.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}
