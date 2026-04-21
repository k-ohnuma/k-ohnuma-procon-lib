use std::cmp::Ordering;

pub fn argcmp(
    (x0, y0, idx0): (isize, isize, usize),
    (x1, y1, idx1): (isize, isize, usize),
    cw: bool,
) -> Ordering {
    let first_half0 = if cw {
        (y0 < 0) || (y0 == 0 && x0 >= 0)
    } else {
        (y0 > 0) || (y0 == 0 && x0 >= 0)
    };
    let first_half1 = if cw {
        (y1 < 0) || (y1 == 0 && x1 >= 0)
    } else {
        (y1 > 0) || (y1 == 0 && x1 >= 0)
    };

    let sec0 = if first_half0 { 0u8 } else { 1u8 };
    let sec1 = if first_half1 { 0u8 } else { 1u8 };

    sec0.cmp(&sec1).then_with(|| {
        let cross: i128 = (x0 as i128) * (y1 as i128) - (y0 as i128) * (x1 as i128);

        let ang = if cw {
            cross.cmp(&0)
        } else {
            cross.cmp(&0).reverse()
        };

        ang.then_with(|| {
            let r0: i128 = (x0 as i128) * (x0 as i128) + (y0 as i128) * (y0 as i128);
            let r1: i128 = (x1 as i128) * (x1 as i128) + (y1 as i128) * (y1 as i128);
            r0.cmp(&r1)
        })
        .then(idx0.cmp(&idx1))
    })
}

pub fn same_dir(a: (isize, isize, usize), b: (isize, isize, usize)) -> bool {
    let (x0, y0, _) = a;
    let (x1, y1, _) = b;
    let cross = (x0 as i128) * (y1 as i128) - (y0 as i128) * (x1 as i128);
    let dot = (x0 as i128) * (x1 as i128) + (y0 as i128) * (y1 as i128);
    cross == 0 && dot > 0
}

#[cfg(test)]
mod tests {
    use super::{argcmp, same_dir};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::{cmp::Ordering, f64::consts::TAU};

    fn angle_key(x: isize, y: isize, cw: bool) -> f64 {
        let mut ang = (y as f64).atan2(x as f64);
        if ang < 0.0 {
            ang += TAU;
        }
        if cw {
            let mut k = TAU - ang;
            if k >= TAU {
                k -= TAU;
            }
            k
        } else {
            ang
        }
    }

    #[test]
    fn cardinal_order_ccw_and_cw() {
        let mut pts = vec![(1, 0, 0), (0, 1, 1), (-1, 0, 2), (0, -1, 3)];

        pts.sort_by(|&a, &b| argcmp(a, b, false));
        assert_eq!(
            pts.into_iter().map(|p| p.2).collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );

        let mut pts = vec![(1, 0, 0), (0, 1, 1), (-1, 0, 2), (0, -1, 3)];
        pts.sort_by(|&a, &b| argcmp(a, b, true));
        assert_eq!(
            pts.into_iter().map(|p| p.2).collect::<Vec<_>>(),
            vec![0, 3, 2, 1]
        );
    }

    #[test]
    fn same_dir_basic() {
        assert!(same_dir((1, 2, 0), (2, 4, 1)));
        assert!(!same_dir((1, 2, 0), (-1, -2, 1)));
        assert!(!same_dir((1, 0, 0), (0, 1, 1)));
        assert!(!same_dir((0, 0, 0), (1, 0, 1)));
    }

    #[test]
    fn random_sort_matches_atan2_reference() {
        let mut rng = StdRng::seed_from_u64(20260421);

        for _ in 0..1000 {
            let n = rng.random_range(0..=120usize);
            let pts: Vec<(isize, isize, usize)> = (0..n)
                .map(|idx| {
                    let x = rng.random_range(-40..=40i32) as isize;
                    let y = rng.random_range(-40..=40i32) as isize;
                    (x, y, idx)
                })
                .collect();

            for &cw in &[false, true] {
                let mut got = pts.clone();
                got.sort_by(|&a, &b| argcmp(a, b, cw));

                let mut exp = pts.clone();
                exp.sort_by(|&(x0, y0, i0), &(x1, y1, i1)| {
                    angle_key(x0, y0, cw)
                        .total_cmp(&angle_key(x1, y1, cw))
                        .then_with(|| {
                            let r0 = (x0 as i128) * (x0 as i128) + (y0 as i128) * (y0 as i128);
                            let r1 = (x1 as i128) * (x1 as i128) + (y1 as i128) * (y1 as i128);
                            r0.cmp(&r1)
                        })
                        .then(i0.cmp(&i1))
                });

                assert_eq!(
                    got.iter().map(|p| p.2).collect::<Vec<_>>(),
                    exp.iter().map(|p| p.2).collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn comparator_is_antisymmetric() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..10_000 {
            let a = (
                rng.random_range(-50..=50i32) as isize,
                rng.random_range(-50..=50i32) as isize,
                rng.random_range(0..200usize),
            );
            let b = (
                rng.random_range(-50..=50i32) as isize,
                rng.random_range(-50..=50i32) as isize,
                rng.random_range(0..200usize),
            );

            for &cw in &[false, true] {
                let ab = argcmp(a, b, cw);
                let ba = argcmp(b, a, cw);
                assert_eq!(ab, ba.reverse(), "cw={cw} a={a:?} b={b:?}");
                assert_ne!(argcmp(a, a, cw), Ordering::Less);
                assert_ne!(argcmp(a, a, cw), Ordering::Greater);
            }
        }
    }
}
