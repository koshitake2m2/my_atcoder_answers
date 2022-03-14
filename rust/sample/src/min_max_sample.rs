#![allow(unused_imports)]
#![allow(dead_code)]

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // i32
        assert_eq!(i32::MAX, 2147483647);
        assert_eq!(i32::MIN, -2147483648);
        // panic: attempt to multiply with overflow
        // assert_eq!(i32::MIN, -2_i32.pow(31));

        assert!(10_i32.pow(9) < i32::MAX);
        let i32_max: i64 = From::from(i32::MAX);
        assert!(i32_max < 10_i64.pow(10));

        // i64
        assert_eq!(i64::MAX, 9223372036854775807);
        assert_eq!(i64::MIN, -9223372036854775808);
        // panic: attempt to multiply with overflow
        // assert_eq!(i64::MIN, -2_i64.pow(63));
    }
}
