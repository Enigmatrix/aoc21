use std::collections::{BTreeSet, HashMap, VecDeque};

const INPUT: &'static str = include_str!("day10.txt");

fn problem1() -> i64 {
    let mut score = HashMap::new();
    score.insert(')', 3);
    score.insert(']', 57);
    score.insert('}', 1197);
    score.insert('>', 25137);
    let mut open_map = HashMap::new();
    open_map.insert(')', '(');
    open_map.insert(']', '[');
    open_map.insert('}', '{');
    open_map.insert('>', '<');

    INPUT
        .lines()
        .filter_map(|s| {
            let mut stack = VecDeque::new();
            for c in s.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push_back(c),
                    ')' | ']' | '}' | '>' => {
                        let prev = stack.pop_back();
                        let open = open_map.get(&c);
                        if open != prev.as_ref() {
                            // corrupter
                            return score.get(&c);
                        }
                    }
                    _ => unreachable!(),
                }
            }
            // incomplete
            None
        })
        .sum()
}

fn problem2() -> i64 {
    let mut score = HashMap::new();
    score.insert('(', 1);
    score.insert('[', 2);
    score.insert('{', 3);
    score.insert('<', 4);
    let mut open_map = HashMap::new();
    open_map.insert(')', '(');
    open_map.insert(']', '[');
    open_map.insert('}', '{');
    open_map.insert('>', '<');

    let nums = INPUT
        .lines()
        .filter_map(|s| {
            let mut stack = VecDeque::new();
            for c in s.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push_front(c),
                    ')' | ']' | '}' | '>' => {
                        let prev = stack.pop_front();
                        let open = open_map.get(&c);
                        if open != prev.as_ref() {
                            // corrupted
                            return None;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            //incomplete
            if stack.is_empty() {
                None
            } else {
                Some(
                    stack
                        .iter()
                        .filter_map(|s| score.get(s))
                        .fold(0, |acc, t| acc * 5 + t),
                )
            }
        })
        .collect::<BTreeSet<i64>>();

    let middle = nums.len() / 2;
    *nums.iter().skip(middle).next().unwrap()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
