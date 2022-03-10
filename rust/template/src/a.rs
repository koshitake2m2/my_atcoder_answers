use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: String
    }
    println!("{}", solve1(s));
}

fn solve1(s: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("ABC".to_string()), 0);
    }
}
