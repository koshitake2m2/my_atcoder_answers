use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: String
    }
    println!("{}", solve1(s));
}

fn solve1(s: String) -> i64 {
    let mut max_count = 0;
    let mut count = 0;
    for c in s.chars() {
        match c {
            'R' => {
                count = count + 1;
                max_count = max_count.max(count);
            }
            'S' => {
                count = 0;
            }
            _ => unreachable!(),
        }
    }
    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("RRS".to_string()), 2);
        assert_eq!(solve1("SSS".to_string()), 0);
        assert_eq!(solve1("RSR".to_string()), 1);
    }
}
