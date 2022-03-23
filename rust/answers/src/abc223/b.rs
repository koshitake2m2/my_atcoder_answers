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
        let mut candidates: Vec<String> = Vec::new();
        let len = s.len();
        let str_ref = s.as_str();
        for i in 0..len {
            let a = str_ref[i..len].to_string() + &*str_ref[0..i].to_string();
            candidates.push(a);
        }
        candidates.sort();
        let s_min = candidates.get(0).unwrap().clone();
        let s_max = candidates.get(len - 1).unwrap().clone();
        vec![s_min, s_max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string_vec(vec: Vec<&str>) -> Vec<String> {
        vec.iter().map(|&s| s.to_string()).collect()
    }

    fn test(solve: &dyn Solve) {
        assert_eq!(
            solve.apply("aaba".to_string()),
            to_string_vec(vec!["aaab", "baaa"])
        );
        assert_eq!(solve.apply("z".to_string()), to_string_vec(vec!["z", "z"]));
        assert_eq!(
            solve.apply("abracadabra".to_string()),
            to_string_vec(vec!["aabracadabr", "racadabraab"])
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
