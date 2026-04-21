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
