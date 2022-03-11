#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        a: [i32; n]
    }
    let solve = Solve2 {};
    println!("{}", solve.apply(n, a));
}

trait Solve {
    fn apply(&self, n: i32, a: Vec<i32>) -> &str;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32, a: Vec<i32>) -> &str {
        let mut is_ok = true;
        for aa in &a {
            match *aa {
                x if x % 2 == 1 => continue,
                x if x % 3 == 0 || x % 5 == 0 => continue,
                _ => {
                    is_ok = false;
                    break;
                }
            }
        }
        if is_ok {
            "APPROVED"
        } else {
            "DENIED"
        }
    }
}

struct Solve2 {}
impl Solve for Solve2 {
    fn apply(&self, n: i32, a: Vec<i32>) -> &str {
        if a.iter()
            .filter(|&x| x % 2 == 0)
            .all(|x| x % 3 == 0 || x % 5 == 0)
        {
            "APPROVED"
        } else {
            "DENIED"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(5, vec![6, 7, 9, 10, 31]), "APPROVED");
        assert_eq!(solve.apply(3, vec![28, 27, 24]), "DENIED");
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }

    #[test]
    fn test_solve2() {
        let solve = &Solve2 {} as &dyn Solve;
        test(solve);
    }
}
