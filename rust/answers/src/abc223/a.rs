#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: i32,
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(x));
}

trait Solve {
    fn apply(&self, x: i32) -> String;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, x: i32) -> String {
        const YES: &str = "Yes";
        const NO: &str = "No";
        let ans: &str = if x % 100 == 0 && x / 100 >= 1 {
            YES
        } else {
            NO
        };
        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(500), "Yes".to_string());
        assert_eq!(solve.apply(40), "No".to_string());
        assert_eq!(solve.apply(0), "No".to_string());
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
