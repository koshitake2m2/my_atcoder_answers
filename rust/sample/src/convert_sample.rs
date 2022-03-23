#![allow(unused_imports)]
#![allow(dead_code)]

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    use std::str::FromStr;

    #[test]
    fn test_to_string_vec() {
        fn to_string_vec_from_str_ref(input: Vec<&str>) -> Vec<String> {
            input.iter().map(|&s| s.to_string()).collect()
        }

        let input = vec!["abc", "def"];
        let output = vec!["abc".to_string(), "def".to_string()];

        assert_eq!(to_string_vec_from_str_ref(input), output);
    }

    #[test]
    fn test_to_string_vec_from_1_line_csv() {
        fn to_string_vec_from_1_line_csv(input: &str) -> Vec<String> {
            input.split(',').map(|s| s.to_string()).collect()
        }

        let input = "abc,def";
        let output = vec!["abc".to_string(), "def".to_string()];

        assert_eq!(to_string_vec_from_1_line_csv(input), output);
    }

    #[test]
    fn test_split_and_convert_vec() {
        fn split_and_convert_vec<T>(input: &str) -> Vec<T>
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            input.split(' ').map(|s| s.parse::<T>().unwrap()).collect()
        }

        // i32
        let input = "1 2 3";
        let output: Vec<i32> = vec![1, 2, 3];
        assert_eq!(split_and_convert_vec::<i32>(input), output);

        // f64
        let input = "1.1 2.2 3.3";
        let output: Vec<f64> = vec![1.1, 2.2, 3.3];
        assert_eq!(split_and_convert_vec::<f64>(input), output);

        // String
        let input = "abc def";
        let output: Vec<String> = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(split_and_convert_vec::<String>(input), output);

        // char
        let input = "a b c";
        let output: Vec<char> = vec!['a', 'b', 'c'];
        assert_eq!(split_and_convert_vec::<char>(input), output);
    }
}
