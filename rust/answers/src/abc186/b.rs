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
    fn apply(&self, _h: i32, _w: i32, a: Vec<Vec<i32>>) -> i32;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, _h: i32, _w: i32, a: Vec<Vec<i32>>) -> i32 {
        // 1. Aijのminを求める
        // 2. Aijとminとの差分の合計を求める
        let mut min_height = 101;
        for aa in &a {
            for aaa in aa {
                min_height = min_height.min(*aaa)
            }
        }
        let mut count = 0;
        for aa in &a {
            for aaa in aa {
                count += *aaa - min_height;
            }
        }
        count
    }
}

struct Solve2 {}
impl Solve for Solve2 {
    fn apply(&self, _h: i32, _w: i32, a: Vec<Vec<i32>>) -> i32 {
        let v: Vec<&i32> = a.iter().flatten().collect::<Vec<_>>();
        let min_height: &i32 = *v.iter().min().unwrap();
        let ans: i32 = v.iter().map(|vv: &&i32| **vv - min_height).sum();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(2, 3, vec![vec![2, 2, 3], vec![3, 2, 2]]), 2);
        assert_eq!(
            solve.apply(
                3,
                3,
                vec![vec![99, 99, 99], vec![99, 0, 99], vec![99, 99, 99]]
            ),
            792
        );
        assert_eq!(
            solve.apply(3, 2, vec![vec![4, 4], vec![4, 4], vec![4, 4]]),
            0
        );
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
