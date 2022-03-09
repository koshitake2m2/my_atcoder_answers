use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let ans = if a == b && b != c {
        "Yes"
    } else if b == c && c != a {
        "Yes"
    } else if c == a && a != b {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
