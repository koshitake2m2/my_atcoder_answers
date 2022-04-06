#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(&s));
}

trait Solve {
    fn apply(&self, s: &str) -> i32;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, s: &str) -> i32 {
        let mut set: HashSet<char> = HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        let ans: i32 = match set.len() {
            1 => 1,
            2 => 3,
            3 => 6,
            _ => unreachable!(),
        };
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    use std::str::FromStr;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply("aba"), 3);
        assert_eq!(solve.apply("ccc"), 1);
        assert_eq!(solve.apply("xyz"), 6);
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
