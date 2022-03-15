#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        p: i32,
        a: [i32; n]
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(n, p, a));
}

trait Solve {
    fn apply(&self, n: i32, p: i32, a: Vec<i32>) -> i32;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, _n: i32, p: i32, a: Vec<i32>) -> i32 {
        a.iter().filter(|&x| *x < p).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(4, 50, vec![80, 60, 40, 0]), 2);
        assert_eq!(solve.apply(3, 90, vec![89, 89, 89]), 3);
        assert_eq!(solve.apply(2, 22, vec![6, 37]), 1);
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
