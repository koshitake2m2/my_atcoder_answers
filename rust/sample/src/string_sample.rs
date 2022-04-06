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
        assert_eq!(string1.chars().nth(1_usize).unwrap(), 'b');

        // 末尾.
        assert_eq!("abcde".ends_with("de"), true);
        assert_eq!("abcde".ends_with("abc"), false);

        // &str -> Vec<char>
        let char_vec1: Vec<char> = "abc".chars().collect();
        assert_eq!(char_vec1, vec!['a', 'b', 'c']);

        // String -> Vec<char>
        let char_vec2: Vec<char> = "abc".to_string().chars().collect();
        assert_eq!(char_vec2, vec!['a', 'b', 'c']);

        // Vec<char> -> String
        let str_ref: String = vec!['a', 'b', 'c'].iter().collect();
        assert_eq!("abc", &str_ref);
    }
}
