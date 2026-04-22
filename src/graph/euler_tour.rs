pub struct EulerTour {
    to: Vec<Vec<usize>>,
    parent: Vec<usize>,
    in_v: Vec<usize>,
    out_v: Vec<usize>,
    order: Vec<usize>,
}

impl EulerTour {
    pub fn new(to: Vec<Vec<usize>>, n: usize) -> Self {
        Self {
            to,
            parent: vec![usize::MAX; n],
            in_v: vec![usize::MAX; n],
            out_v: vec![usize::MAX; n],
            order: vec![],
        }
    }

    pub fn run(&mut self, v: usize, p: usize) {
        self.in_v[v] = self.order.len();
        self.order.push(v);

        for &v2 in self.to[v].to_owned().iter() {
            if v2 == p {
                continue;
            }
            self.parent[v2] = v;
            self.run(v2, v);
        }

        self.out_v[v] = self.order.len();
    }
}
