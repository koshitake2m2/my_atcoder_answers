#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        m: i32,
        a: [String; 2*n]
    }
    let solve = Solve1 {};
    for x in &solve.apply(n, m, a) {
        println!("{}", *x);
    }
}

trait Solve {
    fn apply(&self, n: i32, m: i32, a: Vec<String>) -> Vec<i32>;
}

struct Solve1 {}
impl Solve for Solve1 {
    fn apply(&self, n: i32, m: i32, a: Vec<String>) -> Vec<i32> {
        // 初期化
        let mut persons: Vec<Person> = Vec::new();
        let persons_size = 2 * n;
        for i in 0..persons_size {
            let person: Person = Person {
                number: i + 1,
                hands: a.get(i as usize).unwrap().clone(),
                win_count: 0,
            };
            persons.push(person);
        }

        for round in 1..=m {
            let mut new_persons: Vec<Person> = Vec::new();
            for i in 0..n {
                // じゃんけん
                let l_index: usize = 2 * i as usize;
                let r_index: usize = l_index + 1;
                let l: &Person = persons.get(l_index).unwrap();
                let r: &Person = persons.get(r_index).unwrap();

                let (new_l, new_r) = l.janken(r, round);
                new_persons.push(new_l);
                new_persons.push(new_r);
            }

            // ソート(昇順: 番号 → 降順: 勝数)
            new_persons.sort_by_key(|p| p.number);
            new_persons.sort_by_key(|p| -p.win_count);
            persons = new_persons;
        }

        // 順位ごとに人の番号出力
        persons.iter().map(|p| p.number).collect()
    }
}

struct Person {
    number: i32,
    hands: String,
    win_count: i32,
}

impl Person {
    fn clone(&self) -> Person {
        Person {
            number: self.number,
            hands: self.hands.clone(),
            win_count: self.win_count,
        }
    }
    fn win(&self) -> Person {
        Person {
            number: self.number,
            hands: self.hands.clone(),
            win_count: self.win_count + 1,
        }
    }
    fn janken(&self, other: &Person, round: i32) -> (Person, Person) {
        let hand_index: usize = (round - 1) as usize;
        let self_hand: char = self.hands.chars().nth(hand_index).unwrap();
        let other_hand: char = other.hands.chars().nth(hand_index).unwrap();

        match self_hand {
            'G' => match other_hand {
                'C' => (self.win(), other.clone()),
                'P' => (self.clone(), other.win()),
                _ => (self.clone(), other.clone()),
            },
            'C' => match other_hand {
                'P' => (self.win(), other.clone()),
                'G' => (self.clone(), other.win()),
                _ => (self.clone(), other.clone()),
            },
            'P' => match other_hand {
                'G' => (self.win(), other.clone()),
                'C' => (self.clone(), other.win()),
                _ => (self.clone(), other.clone()),
            },
            _ => (self.clone(), other.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(solve: &dyn Solve) {
        assert_eq!(
            solve.apply(
                2,
                3,
                vec!["GCP", "PPP", "CCC", "PPC"]
                    .iter()
                    .map(|&x| x.to_string())
                    .collect()
            ),
            vec![3, 1, 2, 4]
        );
        assert_eq!(
            solve.apply(
                2,
                2,
                vec!["GC", "PG", "CG", "PP"]
                    .iter()
                    .map(|&x| x.to_string())
                    .collect()
            ),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_solve1() {
        let solve = &Solve1 {} as &dyn Solve;
        test(solve);
    }
}
