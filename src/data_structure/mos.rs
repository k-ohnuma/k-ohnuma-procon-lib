#[derive(Clone, Copy, Debug)]
struct Query {
    l: usize,
    r: usize,
    idx: usize,
}

pub trait MoState<Ans> {
    fn add_left(&mut self, i: usize);
    fn add_right(&mut self, i: usize);
    fn remove_left(&mut self, i: usize);
    fn remove_right(&mut self, i: usize);
    fn answer(&self) -> Ans;
}

pub struct Mo {
    n: usize,
    block: usize,
    queries: Vec<Query>,
}

impl Mo {
    pub fn new(n: usize) -> Self {
        let block = (n as f64).sqrt() as usize + 1;
        Self {
            n,
            block,
            queries: vec![],
        }
    }

    pub fn add_query(&mut self, l: usize, r: usize) {
        assert!(l <= r, "query must satisfy l <= r");
        assert!(r <= self.n, "query must satisfy r <= n");
        let idx = self.queries.len();
        self.queries.push(Query { l, r, idx });
    }

    pub fn run<S, Ans>(&mut self, state: &mut S) -> Vec<Ans>
    where
        S: MoState<Ans>,
        Ans: Clone,
    {
        let block = self.block;

        self.queries.sort_by(|a, b| {
            let ab = a.l / block;
            let bb = b.l / block;
            if ab != bb {
                return ab.cmp(&bb);
            }
            if ab % 2 == 0 {
                a.r.cmp(&b.r)
            } else {
                b.r.cmp(&a.r)
            }
        });

        let mut cur_l = 0usize;
        let mut cur_r = 0usize; // current range: [cur_l, cur_r)

        let mut ans: Vec<Option<Ans>> = vec![None; self.queries.len()];

        for q in self.queries.iter().copied() {
            let (l, r) = (q.l, q.r);

            while cur_l > l {
                cur_l -= 1;
                state.add_left(cur_l);
            }
            while cur_r < r {
                state.add_right(cur_r);
                cur_r += 1;
            }
            while cur_l < l {
                state.remove_left(cur_l);
                cur_l += 1;
            }
            while cur_r > r {
                cur_r -= 1;
                state.remove_right(cur_r);
            }

            ans[q.idx] = Some(state.answer());
        }

        ans.into_iter().map(|x| x.unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Mo, MoState};
    use rand::{rngs::StdRng, Rng, SeedableRng};

    struct SumState<'a> {
        a: &'a [i64],
        sum: i64,
    }

    impl MoState<i64> for SumState<'_> {
        fn add_left(&mut self, i: usize) {
            self.sum += self.a[i];
        }
        fn add_right(&mut self, i: usize) {
            self.sum += self.a[i];
        }
        fn remove_left(&mut self, i: usize) {
            self.sum -= self.a[i];
        }
        fn remove_right(&mut self, i: usize) {
            self.sum -= self.a[i];
        }
        fn answer(&self) -> i64 {
            self.sum
        }
    }

    #[test]
    fn random_range_sum_matches_naive() {
        let mut rng = StdRng::seed_from_u64(20260421);

        for _case in 0..1000 {
            let n = rng.random_range(0..=80usize);
            let mut a = vec![0i64; n];
            for x in a.iter_mut().take(n) {
                *x = rng.random_range(-20..=20);
            }

            let q = rng.random_range(0..=300usize);
            let mut mo = Mo::new(n);
            let mut queries = Vec::with_capacity(q);

            for _ in 0..q {
                let l = rng.random_range(0..=n);
                let r = rng.random_range(l..=n);
                mo.add_query(l, r);
                queries.push((l, r));
            }

            let mut state = SumState { a: &a, sum: 0 };
            let got = mo.run(&mut state);
            let exp: Vec<i64> = queries.iter().map(|&(l, r)| a[l..r].iter().sum()).collect();

            assert_eq!(got, exp, "n={n} q={q}");
        }
    }

    #[test]
    #[should_panic]
    fn add_query_panics_when_l_gt_r() {
        let mut mo = Mo::new(10);
        mo.add_query(7, 3);
    }

    #[test]
    #[should_panic]
    fn add_query_panics_when_r_exceeds_n() {
        let mut mo = Mo::new(10);
        mo.add_query(0, 11);
    }
}
