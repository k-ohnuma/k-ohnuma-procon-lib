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
    block: usize,
    queries: Vec<Query>,
}

impl Mo {
    pub fn new(n: usize) -> Self {
        let block = (n as f64).sqrt() as usize + 1;
        Self {
            block,
            queries: vec![],
        }
    }

    pub fn add_query(&mut self, l: usize, r: usize) {
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
