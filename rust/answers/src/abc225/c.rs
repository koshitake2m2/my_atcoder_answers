#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        m: i32,
        b: [[i32; m]; n]
    }
    let solve = Solve2 {};
    println!("{}", solve.apply(n, m, b));
}

trait Solve {
    fn apply(&self, n: i32, m: i32, b: Vec<Vec<i32>>) -> String;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32, m: i32, b: Vec<Vec<i32>>) -> String {
        fn get_i(x: i32) -> i32 {
            if x % 7 == 0 {
                x / 7
            } else {
                x / 7 + 1
            }
        }
        fn get_j(x: i32) -> i32 {
            let mod7 = x % 7;
            if mod7 == 0 {
                7
            } else {
                mod7
            }
        }
        const YES: &str = "Yes";
        const NO: &str = "No";
        let first: i32 = b[0][0];
        let first_i: i32 = get_i(first);
        let first_j: i32 = get_j(first);

        let mut is_ok = true;
        'break_point: for i in 0..n as usize {
            for j in 0..m as usize {
                let target: i32 = b[i][j];
                let target_i: i32 = get_i(target);
                let target_j: i32 = get_j(target);
                if !(target_i == first_i + i as i32 && target_j == first_j + j as i32) {
                    is_ok = false;
                    break 'break_point;
                }
            }
        }

        (if is_ok { YES } else { NO }).to_string()
    }
}
struct Solve2 {}
impl Solve for Solve2 {
    fn apply(&self, n: i32, m: i32, b: Vec<Vec<i32>>) -> String {
        fn get_i(x: i32) -> i32 {
            (x + 6) / 7
        }
        fn get_j(x: i32) -> i32 {
            (x - 1) % 7 + 1
        }
        const YES: &str = "Yes";
        const NO: &str = "No";
        let first: i32 = b[0][0];
        let first_i: i32 = get_i(first);
        let first_j: i32 = get_j(first);

        let mut is_ok = true;
        'break_point: for i in 0..n as usize {
            for j in 0..m as usize {
                let target: i32 = b[i][j];
                let target_i: i32 = get_i(target);
                let target_j: i32 = get_j(target);
                if !(target_i == first_i + i as i32 && target_j == first_j + j as i32) {
                    is_ok = false;
                    break 'break_point;
                }
            }
        }

        (if is_ok { YES } else { NO }).to_string()
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
            solve.apply(2, 3, split_and_convert_vec2::<i32>("1 2 3,8 9 10")),
            "Yes".to_string()
        );
        assert_eq!(
            solve.apply(2, 1, split_and_convert_vec2::<i32>("1,2")),
            "No".to_string()
        );
        let input = "1346 1347 1348 1349,1353 1354 1355 1356,1360 1361 1362 1363,1367 1368 1369 1370,1374 1375 1376 1377,1381 1382 1383 1384,1388 1389 1390 1391,1395 1396 1397 1398,1402 1403 1404 1405,1409 1410 1411 1412";
        assert_eq!(
            solve.apply(10, 4, split_and_convert_vec2::<i32>(input)),
            "Yes".to_string()
        );
        // 自作サンプル
        assert_eq!(
            solve.apply(2, 3, split_and_convert_vec2::<i32>("5 6 7,12 13 14")),
            "Yes".to_string()
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
