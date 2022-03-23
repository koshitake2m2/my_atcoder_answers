#![allow(unused_imports)]
#![allow(dead_code)]

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

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
}
