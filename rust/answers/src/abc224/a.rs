#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: String
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(&s));
}

trait Solve {
    fn apply(&self, s: &str) -> String;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, s: &str) -> String {
        let ans: &str = if s.ends_with("er") {
            "er"
        } else if s.ends_with("ist") {
            "ist"
        } else {
            unreachable!()
        };
        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply("atcoder"), "er".to_string());
        assert_eq!(solve.apply("tourist"), "ist".to_string());
        assert_eq!(solve.apply("er"), "er".to_string());
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
