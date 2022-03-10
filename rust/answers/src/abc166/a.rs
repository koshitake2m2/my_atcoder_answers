use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: String
    }
    println!("{}", solve1(s));
}

fn solve1(s: String) -> String {
    match s.as_str() {
        "ABC" => "ARC".to_string(),
        "ARC" => "ABC".to_string(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("ABC".to_string()), "ARC");
        assert_eq!(solve1("ARC".to_string()), "ABC");
    }
}
