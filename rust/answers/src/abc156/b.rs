#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        k: i32,
    }
    println!("{}", solve1(n, k));
}

fn solve1(n: i32, k: i32) -> i32 {
    let mut keta = 0;
    let mut nn = n;
    while nn != 0 {
        keta += 1;
        nn /= k;
    }
    keta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(11, 2), 4);
        assert_eq!(solve1(1010101, 10), 7);
        assert_eq!(solve1(314159265, 3), 18);
    }
}
