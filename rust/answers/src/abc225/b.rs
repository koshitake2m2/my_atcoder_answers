#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        ab: [[i32; 2]; n-1]
    }
    let solve = Solve1 {};
    println!("{}", solve.apply(n, ab));
}

trait Solve {
    fn apply(&self, n: i32, ab: Vec<Vec<i32>>) -> String;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32, ab: Vec<Vec<i32>>) -> String {
        const YES: &str = "Yes";
        const NO: &str = "No";
        let mut set_by_vertex: HashMap<i32, HashSet<i32>> = HashMap::new();
        for n_i in 1..=n {
            set_by_vertex.insert(n_i, HashSet::new());
        }
        for ab_i in &ab {
            let a_i = ab_i.get(0).unwrap();
            let b_i = ab_i.get(1).unwrap();
            let set_a = set_by_vertex.get_mut(a_i).unwrap();
            set_a.insert(*b_i);
            let set_b = set_by_vertex.get_mut(b_i).unwrap();
            set_b.insert(*a_i);
        }

        let mut is_ok = false;
        for (_, set) in set_by_vertex.iter() {
            if set.len() == (n - 1) as usize {
                is_ok = true;
            }
        }

        let ans = (if is_ok { YES } else { NO }).to_string();
        ans
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
            solve.apply(5, split_and_convert_vec2::<i32>("1 4,2 4,3 4,4 5")),
            "Yes".to_string()
        );
        assert_eq!(
            solve.apply(4, split_and_convert_vec2::<i32>("2 4,1 4,2 3")),
            "No".to_string()
        );
        assert_eq!(
            solve.apply(
                10,
                split_and_convert_vec2::<i32>("9 10,3 10,4 10,8 10,1 10,2 10,7 10,6 10,5 10")
            ),
            "Yes".to_string()
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
