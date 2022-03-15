#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
        b: [i32; n],
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(n, a, b));
}

trait Solve {
    fn apply(&self, n: i32, a: Vec<i32>, b: Vec<i32>) -> i64;
}

// WA & TLE
struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32, a: Vec<i32>, b: Vec<i32>) -> i64 {
        const MOD: i64 = 998244353;
        // c[i番目 * 3001 + y座標] = その座標でとりうる数列の数
        let mut c: Vec<i64> = vec![0_i64; (n * 3001) as usize];
        // 0番目
        for y in a[0]..=b[0] {
            c[y as usize] = 1;
        }

        // 1番目以降の処理
        for i in 1..n as usize {
            // i番目における高さに関して
            for y in 1..=3000 {
                // c_iのy座標
                // a_iとb_iの間にあるc_iだけ計算する
                if a[i] <= y && y <= b[i] {
                    if b[i - 1] < y {
                        // ありえるc_(i-1)の数を加算する
                        // c_y_1: c_(i-1)におけるy座標
                        for c_y_1 in a[i - 1]..=b[i - 1] {
                            c[i * 3001 + y as usize] += c[(i - 1) * 3001 + c_y_1 as usize]
                        }
                        c[i * 3001 + y as usize] %= MOD;
                    } else {
                        for c_y_1 in a[i - 1]..=y {
                            c[i * 3001 + y as usize] += c[(i - 1) * 3001 + c_y_1 as usize]
                        }
                        c[i * 3001 + y as usize] %= MOD;
                    }
                }
            }
        }

        // 最後の集計
        let mut ans: i64 = 0;
        for c_y in 1..=3000 {
            ans += c[((n - 1) * 3001 + c_y) as usize];
            ans %= MOD
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(solve.apply(2, vec![1, 1], vec![2, 3]), 5);
        assert_eq!(solve.apply(3, vec![2, 2, 2], vec![2, 2, 2]), 1);
        assert_eq!(
            solve.apply(
                10,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
            ),
            978222082
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
