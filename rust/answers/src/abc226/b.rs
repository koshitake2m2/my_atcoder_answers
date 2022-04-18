use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
    }
    let mut a: HashSet<Vec<i32>> = HashSet::new();
    for _i in 1..=n {
        input! {
            l: i32,
            a_i: [i32; l]
        }
        a.insert(a_i);
    }
    let ans = a.len();
    println!("{}", ans);
}
