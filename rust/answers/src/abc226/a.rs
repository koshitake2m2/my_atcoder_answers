use proconio::input;

fn main() {
    input! {
        x: f32
    }
    let ans = x.round();
    println!("{}", ans)
}
