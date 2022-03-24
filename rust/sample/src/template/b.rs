#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: String,
    }
    let solve = Solve1 {};
    for ss in &solve.apply(s) {
        println!("{}", *ss);
    }
}

trait Solve {
    fn apply(&self, s: String) -> Vec<String>;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, s: String) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    use std::str::FromStr;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply("aaba".to_string()), Vec::new());
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
