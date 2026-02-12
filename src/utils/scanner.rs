use std::str::FromStr;

pub struct Scanner<I: Iterator<Item = char>> {
    iter: std::iter::Peekable<I>,
}

macro_rules! exit {
    () => {{
        exit!(0)
    }};
    ($code:expr) => {{
        std::process::exit($code);
    }};
}

impl<I: Iterator<Item = char>> Scanner<I> {
    pub fn new(iter: I) -> Scanner<I> {
        Scanner {
            iter: iter.peekable(),
        }
    }

    pub fn safe_get_token(&mut self) -> Option<String> {
        let token = self
            .iter
            .by_ref()
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            Some(token)
        }
    }

    pub fn token(&mut self) -> String {
        self.safe_get_token().unwrap_or_else(|| exit!())
    }

    pub fn get<T: FromStr>(&mut self) -> T {
        self.token().parse::<T>().unwrap_or_else(|_| exit!())
    }

    pub fn vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
        (0..len).map(|_| self.get()).collect()
    }

    pub fn mat<T: FromStr>(&mut self, row: usize, col: usize) -> Vec<Vec<T>> {
        (0..row).map(|_| self.vec(col)).collect()
    }

    pub fn char(&mut self) -> char {
        self.iter.next().unwrap_or_else(|| exit!())
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }

    pub fn line(&mut self) -> String {
        if self.peek().is_some() {
            self.iter
                .by_ref()
                .take_while(|&c| !(c == '\n' || c == '\r'))
                .collect::<String>()
        } else {
            exit!();
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
}
// fn main() {
//     let cin = stdin().lock();
//     let mut sc = Scanner::new(cin.bytes().map(|e| e.unwrap() as char));
//     loop {
//         let w: usize = sc.get();
//         let h: usize = sc.get();
//     }
// }
