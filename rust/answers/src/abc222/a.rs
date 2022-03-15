#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(n));
}

trait Solve {
    fn apply(&self, n: i32) -> String;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32) -> String {
        format!("{:04}", n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(321), "0321".to_string());
        assert_eq!(solve.apply(7777), "7777".to_string());
        assert_eq!(solve.apply(1), "0001".to_string());
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
