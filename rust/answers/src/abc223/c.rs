#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        a: [(i32, i32); n]
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(n, a));
}

trait Solve {
    fn apply(&self, n: i32, ab: Vec<(i32, i32)>) -> f64;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, _n: i32, ab: Vec<(i32, i32)>) -> f64 {
        // 2つの火がぶつかるのにかかる時間
        let mut time_cost: Vec<f64> = Vec::new();
        for (ai, bi) in &ab {
            time_cost.push(*ai as f64 / *bi as f64);
        }
        let t = time_cost.iter().fold(0_f64, |a, b| a + b) / 2_f64;

        // 左端からans[cm]の計算
        let mut t_fushime = 0_f64;
        let mut ans: f64 = 0_f64;
        for (ai, bi) in &ab {
            let ti: f64 = *ai as f64 / *bi as f64;
            if t_fushime + ti < t {
                ans += *ai as f64;
                t_fushime += ti;
            } else {
                ans += (t - t_fushime) * (*bi as f64);
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(actual: f64, expected: f64) {
        let diff: f64 = (actual - expected) * 10_f64.powf(5_f64).abs();
        if diff <= 1_f64 {
            println!("(actual, expected) = ({:.10}, {:.10})", actual, expected);
            assert_eq!(0, 0);
        } else {
            println!("{:.10}", diff.abs());
            println!("(actual, expected) = ({:.10}, {:.10})", actual, expected);
            assert_eq!(0, 1);
        }
    }

    fn test(solve: &dyn Solve) {
        check(
            solve.apply(3, vec![(1, 1), (2, 1), (3, 1)]),
            3.000000000000000_f64,
        );
        check(
            solve.apply(3, vec![(1, 3), (2, 2), (3, 1)]),
            3.833333333333333_f64,
        );
        check(
            solve.apply(5, vec![(3, 9), (1, 2), (4, 6), (1, 5), (5, 3)]),
            8.916666666666668_f64,
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
