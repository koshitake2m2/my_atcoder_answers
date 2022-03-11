#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
use std::iter::Map;

fn main() {
    input! {
        n: i32,
        s: [String; n],
    }
    for ss in solve1(n, &s) {
        println!("{}", ss);
    }
}

fn solve1(_n: i32, s: &[String]) -> Vec<String> {
    let mut dic: HashMap<&String, i32> = HashMap::new();
    for s_i in s {
        if dic.contains_key(s_i) {
            dic.insert(s_i, dic.get(s_i).unwrap() + 1);
        } else {
            dic.insert(s_i, 0);
        }
    }
    let max_count = *dic.values().into_iter().max().unwrap();
    let max_dic: HashMap<&String, i32> = dic
        .iter()
        .filter(|(_, &v)| v == max_count)
        .map(|(&k, &v)| (k, v))
        .collect();
    let mut ans: Vec<String> = max_dic
        .keys()
        .into_iter()
        .map(|&k| k.clone())
        .collect::<Vec<String>>();
    ans.sort();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(
            solve1(
                7,
                &"beat,vet,beet,bed,vet,bet,beet"
                    .split(",")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            ),
            vec!["beet", "vet"]
        );
        assert_eq!(solve1(8, &vec!["buffalo".to_string(); 8]), vec!["buffalo"]);
        assert_eq!(
            solve1(
                7,
                &"bass,bass,kick,kick,bass,kick,kick"
                    .split(",")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            ),
            vec!["kick"]
        );
        assert_eq!(
            solve1(
                4,
                &"ushi,tapu,nichia,kun"
                    .split(",")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            ),
            "kun,nichia,tapu,ushi"
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );
    }
}
