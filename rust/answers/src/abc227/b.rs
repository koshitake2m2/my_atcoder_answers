use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        s: [i32; n]
    }
    let mut maybe_answers: HashSet<i32> = HashSet::new();
    fn area(a: i32, b: i32) -> i32 {
        a * (4 * b + 3) + 3 * b
    }

    for a in 1..=142 {
        for b in 1..=142 {
            let s = area(a, b);
            if s > 1000 {
                break;
            }
            maybe_answers.insert(s);
        }
    }

    let mut ans = 0;
    for s_i in &s {
        if !maybe_answers.contains(s_i) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
