#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        x: [i32; n]
    }
    println!("{}", solve1(n, x));
}

fn solve1(_n: i32, x: Vec<i32>) -> i32 {
    let mut min_stamina: i32 = 10_i32.pow(9);
    for p_i in 1..100 {
        let mut stamina: i32 = 0;
        for x_i in &x {
            stamina += (*x_i - p_i).pow(2);
        }
        min_stamina = min_stamina.min(stamina);
    }
    min_stamina
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(2, vec![1, 4]), 5);
        assert_eq!(solve1(7, vec![14, 14, 2, 13, 56, 2, 37]), 2354);
    }
}
