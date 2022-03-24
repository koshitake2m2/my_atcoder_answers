#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: i32,
        w: i32,
        a: [[i32; w]; h]
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(h, w, a));
}

trait Solve {
    fn apply(&self, h: i32, w: i32, a: Vec<Vec<i32>>) -> String;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, h: i32, w: i32, a: Vec<Vec<i32>>) -> String {
        let mut is_ok = true;
        'break_point: for i_1 in 0..(h - 1) as usize {
            for i_2 in (i_1 + 1)..h as usize {
                for j_1 in 0..(w - 1) as usize {
                    for j_2 in (j_1 + 1)..w as usize {
                        if a[i_1][j_1] + a[i_2][j_2] > a[i_2][j_1] + a[i_1][j_2] {
                            is_ok = false;
                            break 'break_point;
                        }
                    }
                }
            }
        }
        if is_ok { "Yes" } else { "No" }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(
            solve.apply(3, 3, vec![vec![2, 1, 4], vec![3, 1, 3], vec![6, 4, 1]]),
            "Yes".to_string()
        );
        assert_eq!(
            solve.apply(2, 4, vec![vec![4, 3, 2, 1], vec![5, 6, 7, 8]]),
            "No".to_string()
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
