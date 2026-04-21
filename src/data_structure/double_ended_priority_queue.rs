use std::collections::BTreeMap;
pub struct DoubleEndedPriorityQueue<T> {
    map: BTreeMap<T, usize>,
}

impl<T: num_integer::Integer + Copy> Default for DoubleEndedPriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: num_integer::Integer + Copy> DoubleEndedPriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, x: T) {
        self.map.entry(x).and_modify(|e| *e += 1).or_insert(1usize);
    }
    pub fn remove(&mut self, x: T) -> bool {
        let &cc = self.map.get(&x).unwrap_or(&0);
        if cc == 0 {
            return false;
        }
        if cc == 1 {
            self.map.remove(&x);
            return true;
        }
        self.map.insert(x, cc - 1);
        true
    }

    pub fn first(&self) -> Option<T> {
        let (&f, _) = self.map.first_key_value()?;
        Some(f)
    }
    pub fn last(&self) -> Option<T> {
        let (&f, _) = self.map.last_key_value()?;
        Some(f)
    }
    pub fn count(&self, x: T) -> usize {
        let &c = self.map.get(&x).unwrap_or(&0);
        c
    }
    pub fn pop_first(&mut self) -> Option<T> {
        let num = self.first()?;
        self.remove(num);
        Some(num)
    }
    pub fn pop_last(&mut self) -> Option<T> {
        let num = self.last()?;
        self.remove(num);
        Some(num)
    }

    pub fn map(&self) -> &BTreeMap<T, usize> {
        &self.map
    }
}

#[cfg(test)]
mod tests {
    use rand::{RngCore, SeedableRng};

    use crate::data_structure::double_ended_priority_queue::DoubleEndedPriorityQueue;

    fn ref_insert(v: &mut Vec<i64>, x: i64) {
        v.push(x);
    }
    fn ref_remove(v: &mut Vec<i64>, x: i64) -> bool {
        if let Some(i) = v.iter().position(|&y| y == x) {
            v.swap_remove(i);
            true
        } else {
            false
        }
    }
    fn ref_first(v: &[i64]) -> Option<i64> {
        v.iter().copied().min()
    }
    fn ref_last(v: &[i64]) -> Option<i64> {
        v.iter().copied().max()
    }
    fn ref_count(v: &[i64], x: i64) -> usize {
        v.iter().filter(|&&y| y == x).count()
    }
    fn ref_pop_first(v: &mut Vec<i64>) -> Option<i64> {
        let m = ref_first(v)?;
        assert!(ref_remove(v, m));
        Some(m)
    }
    fn ref_pop_last(v: &mut Vec<i64>) -> Option<i64> {
        let m = ref_last(v)?;
        assert!(ref_remove(v, m));
        Some(m)
    }

    fn rand_i64(rng: &mut impl RngCore) -> i64 {
        let x = (rng.next_u64() % 101) as i64;
        x - 50 // -50..=50
    }

    #[test]
    fn basic_scenarios() {
        let mut deq = DoubleEndedPriorityQueue::<i64>::new();

        assert_eq!(deq.first(), None);
        assert_eq!(deq.last(), None);
        assert_eq!(deq.pop_first(), None);
        assert_eq!(deq.pop_last(), None);
        assert!(!deq.remove(10));
        assert_eq!(deq.count(10), 0);

        deq.insert(5);
        deq.insert(5);
        deq.insert(-1);
        assert_eq!(deq.first(), Some(-1));
        assert_eq!(deq.last(), Some(5));
        assert_eq!(deq.count(5), 2);

        assert!(deq.remove(5));
        assert_eq!(deq.count(5), 1);
        assert!(deq.remove(5));
        assert_eq!(deq.count(5), 0);
        assert!(!deq.remove(5));

        assert_eq!(deq.pop_first(), Some(-1));
        assert_eq!(deq.first(), None);
    }

    #[test]
    fn randomized_against_vec_multiset() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xC0FFEE);

        for _case in 0..200 {
            let mut deq = DoubleEndedPriorityQueue::<i64>::new();
            let mut r: Vec<i64> = vec![];

            for _ in 0..5000 {
                let op = rng.next_u32() % 8;
                let x = rand_i64(&mut rng);

                match op {
                    0 => {
                        // insert
                        deq.insert(x);
                        ref_insert(&mut r, x);
                    }
                    1 => {
                        // remove(x)
                        let a = deq.remove(x);
                        let b = ref_remove(&mut r, x);
                        assert_eq!(a, b);
                    }
                    2 => {
                        // first
                        assert_eq!(deq.first(), ref_first(&r));
                    }
                    3 => {
                        // last
                        assert_eq!(deq.last(), ref_last(&r));
                    }
                    4 => {
                        // pop_first
                        assert_eq!(deq.pop_first(), ref_pop_first(&mut r));
                    }
                    5 => {
                        // pop_last
                        assert_eq!(deq.pop_last(), ref_pop_last(&mut r));
                    }
                    6 => {
                        // count
                        assert_eq!(deq.count(x), ref_count(&r, x));
                    }
                    _ => {
                        if r.is_empty() {
                            assert_eq!(deq.first(), None);
                            assert_eq!(deq.last(), None);
                        } else {
                            assert_eq!(deq.first(), ref_first(&r));
                            assert_eq!(deq.last(), ref_last(&r));
                        }

                        for _ in 0..5 {
                            let y = rand_i64(&mut rng);
                            assert_eq!(deq.count(y), ref_count(&r, y));
                        }

                        for (&_k, &c) in deq.map().iter() {
                            assert!(c >= 1);
                        }
                    }
                }
            }
        }
    }
}
