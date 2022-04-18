use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        a: i32,
    }
    let mut ans = (a + k - 1) % n;
    if ans == 0 {
        ans = n;
    }
    println!("{}", ans);
}
