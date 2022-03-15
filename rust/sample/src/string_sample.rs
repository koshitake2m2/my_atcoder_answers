#![allow(unused_imports)]
#![allow(dead_code)]

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 要素取得.
        let string1: String = "abcde".to_string();
        assert_eq!(string1.chars().nth(0_usize).unwrap(), 'a');
        assert_eq!(string1.get(0_usize).unwrap(), 'a');
        assert_eq!(string1.chars().nth(1_usize).unwrap(), 'b');
    }
}
