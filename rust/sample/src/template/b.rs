#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: i32,
        w: i32,
        a: [[i32; w]; h]
    }
    println!("{}", solve1(h, w, a));
}

fn solve1(h: i32, w: i32, a: Vec<Vec<i32>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(2, 3, vec![vec![0, 1, 2], vec![3, 4, 5]]), 0);
    }
}
