pub fn replace_manhattan_coord(xy: (isize, isize)) -> (isize, isize) {
    let (x, y) = xy;
    (x - y, x + y)
}
