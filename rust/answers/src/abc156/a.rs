mod b;

use proconio::input;

fn main() {
    input! {
        n: i64,
        r: i64
    }
    if 10 <= n {
        println!("{}", r)
    } else {
        println!("{}", r + 100 * (10 - n))
    }
}
