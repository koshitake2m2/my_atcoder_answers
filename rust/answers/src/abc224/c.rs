#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        xy: [[i32; 2]; n]
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(n, xy));
}

trait Solve {
    fn apply(&self, n: i32, xy: Vec<Vec<i32>>) -> i32;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32, xy: Vec<Vec<i32>>) -> i32 {
        fn n_c_k(n: i32, k: i32) -> i32 {
            let bunshi = (n - k + 1..=n).fold(1, |acc, ki| acc * ki);
            (1..=k).fold(bunshi, |acc, ki| acc / ki)
        }
        struct Point {
            x: i32,
            y: i32,
        }
        let mut ng_count: i32 = 0;
        let points: Vec<Point> = xy
            .iter()
            .map(|xy_i| Point {
                x: xy_i[0],
                y: xy_i[1],
            })
            .collect();
        for i in 0..(n - 2) as usize {
            for j in (i + 1)..(n - 1) as usize {
                for k in (j + 1)..n as usize {
                    // 傾きが一緒
                    let point_i: &Point = &points[i];
                    let point_j: &Point = &points[j];
                    let point_k: &Point = &points[k];
                    let delta_xij = point_j.x - point_i.x;
                    let delta_yij = point_j.y - point_i.y;
                    let delta_xik = point_k.x - point_i.x;
                    let delta_yik = point_k.y - point_i.y;
                    if delta_xij as i64 * delta_yik as i64 == delta_xik as i64 * delta_yij as i64 {
                        ng_count += 1
                    }
                }
            }
        }

        n_c_k(n, 3) - ng_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    use std::str::FromStr;
    fn split_and_convert_vec2<T>(input: &str) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        input
            .split(',')
            .map(|s| s.split(' ').map(|s2| s2.parse::<T>().unwrap()).collect())
            .collect()
    }

    fn test(solve: &dyn Solve) {
        assert_eq!(
            solve.apply(4, split_and_convert_vec2::<i32>("0 1,1 3,1 1,-1 -1")),
            3
        );
        assert_eq!(
            solve.apply(20, split_and_convert_vec2::<i32>("224 433,987654321 987654321,2 0,6 4,314159265 358979323,0 0,-123456789 123456789,-1000000000 1000000000,124 233,9 -6,-4 0,9 5,-7 3,333333333 -333333333,-9 -1,7 -10,-1 5,324 633,1000000000 -1000000000,20 0")),
            1124
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
