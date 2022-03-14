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
    let solve = Solve1 {};
    println!("{}", solve.apply(h, w, a));
}

trait Solve {
    fn apply(&self, h: i32, w: i32, a: Vec<Vec<i32>>) -> i32;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, h: i32, w: i32, a: Vec<Vec<i32>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(2, 3, vec![vec![0, 1, 2], vec![3, 4, 5]]), 0);
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
