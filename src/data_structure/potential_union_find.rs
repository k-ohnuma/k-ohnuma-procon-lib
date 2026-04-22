pub struct PotentialUnionFind {
    rank: Vec<usize>,
    parent: Vec<usize>,
    size: Vec<usize>,
    diff_weight: Vec<isize>,
}
impl PotentialUnionFind {
    pub fn new(n: usize) -> Self {
        let rank = vec![0; n];
        let parent = (0..n).collect::<Vec<_>>();
        let size = vec![1; n];
        let diff_weight = vec![0; n];
        PotentialUnionFind {
            rank,
            parent,
            size,
            diff_weight,
        }
    }

    pub fn leader(&mut self, n: usize) -> usize {
        if self.parent[n] == n {
            n
        } else {
            let r = self.leader(self.parent[n]);
            self.diff_weight[n] += self.diff_weight[self.parent[n]];
            self.parent[n] = r;
            r
        }
    }

    pub fn weight(&mut self, n: usize) -> isize {
        self.leader(n);
        self.diff_weight[n]
    }

    pub fn merge(&mut self, a: usize, b: usize, w: isize) -> bool {
        let w = w + self.weight(a) - self.weight(b);
        let a = self.leader(a);
        let b = self.leader(b);
        if a == b {
            return w == 0;
        }
        if self.rank[a] < self.rank[b] {
            self.parent[a] = b;
            self.size[b] += self.size[a];
            self.diff_weight[a] = -w;
        } else {
            self.parent[b] = a;
            self.size[a] += self.size[b];
            if self.rank[a] == self.rank[b] {
                self.rank[a] += 1;
            }
            self.diff_weight[b] = w;
        }
        true
    }

    pub fn diff(&mut self, a: usize, b: usize) -> isize {
        self.weight(b) - self.weight(a)
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    pub fn size(&mut self, n: usize) -> usize {
        let leader = self.leader(n);
        self.size[leader]
    }
}

#[cfg(test)]
mod tests {
    use super::PotentialUnionFind;
    use rand::Rng;
    use std::collections::VecDeque;

    fn brute_diff(n: usize, g: &[Vec<(usize, isize)>], a: usize, b: usize) -> Option<isize> {
        let mut pot: Vec<Option<isize>> = vec![None; n];
        let mut q = VecDeque::new();
        pot[a] = Some(0);
        q.push_back(a);

        while let Some(v) = q.pop_front() {
            let pv = pot[v].unwrap();
            for &(nx, w) in &g[v] {
                if pot[nx].is_none() {
                    pot[nx] = Some(pv + w);
                    q.push_back(nx);
                }
            }
        }
        match (pot[a], pot[b]) {
            (Some(pa), Some(pb)) => Some(pb - pa),
            _ => None,
        }
    }

    fn brute_comp_size(n: usize, g: &[Vec<(usize, isize)>], s: usize) -> usize {
        let mut seen = vec![false; n];
        let mut q = VecDeque::new();
        seen[s] = true;
        q.push_back(s);
        let mut cnt = 0usize;

        while let Some(v) = q.pop_front() {
            cnt += 1;
            for &(nx, _) in &g[v] {
                if !seen[nx] {
                    seen[nx] = true;
                    q.push_back(nx);
                }
            }
        }
        cnt
    }

    #[test]
    fn random_consistent_matches_bruteforce() {
        let mut rng = rand::rng();

        for _case in 0..2000 {
            let n = rng.random_range(1..=60);
            let mut uf = PotentialUnionFind::new(n);

            let mut g: Vec<Vec<(usize, isize)>> = vec![vec![]; n];

            let q = rng.random_range(1..=500);
            for _ in 0..q {
                let t = rng.random_range(0..3);
                let a = rng.random_range(0..n);
                let b = rng.random_range(0..n);

                match t {
                    0 => {
                        if uf.same(a, b) {
                            let w = uf.diff(a, b);
                            assert!(uf.merge(a, b, w));
                        } else {
                            let w: isize = rng.random_range(-50i64..=50) as isize;
                            assert!(uf.merge(a, b, w));

                            g[a].push((b, w));
                            g[b].push((a, -w));
                        }
                    }
                    1 => {
                        let brute_same = brute_diff(n, &g, a, b).is_some();
                        assert_eq!(uf.same(a, b), brute_same);
                    }
                    _ => {
                        if uf.same(a, b) {
                            let got = uf.diff(a, b);
                            let exp = brute_diff(n, &g, a, b).unwrap();
                            assert_eq!(got, exp);
                        } else {
                            assert!(brute_diff(n, &g, a, b).is_none());
                        }
                    }
                }

                let v = rng.random_range(0..n);
                let got = uf.size(v);
                let exp = brute_comp_size(n, &g, v);
                assert_eq!(got, exp);
            }
        }
    }

    #[test]
    fn random_inconsistency_is_detected() {
        let mut rng = rand::rng();

        for _case in 0..2000 {
            let n = rng.random_range(2..=80);
            let mut uf = PotentialUnionFind::new(n);

            for _ in 0..(n * 2) {
                let a = rng.random_range(0..n);
                let b = rng.random_range(0..n);
                if a == b {
                    continue;
                }
                let w: isize = rng.random_range(-30i64..=30) as isize;
                uf.merge(a, b, w);
            }

            for _ in 0..200 {
                let a = rng.random_range(0..n);
                let b = rng.random_range(0..n);
                if !uf.same(a, b) {
                    continue;
                }

                let correct = uf.diff(a, b);

                let mut wrong = correct;
                while wrong == correct {
                    wrong = rng.random_range(-50i64..=50) as isize;
                }

                assert!(!uf.merge(a, b, wrong));

                assert!(uf.merge(a, b, correct));
                break;
            }
        }
    }
}
