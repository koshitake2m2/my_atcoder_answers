use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: char,
    }
    println!("{}", solve1(c));
}

fn solve1(c: char) -> char {
    match c {
        x if 'a' <= x && x <= 'z' => 'a',
        x if 'A' <= x && x <= 'Z' => 'A',
        _ => unreachable!(),
    }
}

fn solve2(c: char) -> char {
    match c {
        'a'..='z' => 'a',
        'A'..='Z' => 'A',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1('B'), 'A');
        assert_eq!(solve1('a'), 'a');
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2('B'), 'A');
        assert_eq!(solve2('a'), 'a');
    }
}
