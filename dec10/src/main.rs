use itertools::sorted;
const VALID_OPENINGS: [char; 4] = ['(', '[', '{', '<'];

fn score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Invalid delimiter."),
    }
}

fn opening(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Invalid delimiter."),
    }
}

fn one(input: &str) -> u64 {
    let mut s = 0;
    for line in input.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if VALID_OPENINGS.contains(&c) {
                chars.push(c);
            } else if opening(c) != chars.pop().unwrap() {
                s += score(c);
                break;
            }
        }
    }
    s
}

fn median(v: Vec<u64>) -> u64 {
    let mid = v.len() / 2;
    let v: Vec<u64> = sorted(v).collect();
    v[mid]
}

fn two(input: &str) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut s = 0;
        let mut chars: Vec<char> = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            if VALID_OPENINGS.contains(&c) {
                chars.push(c);
            } else if opening(c) != chars.pop().unwrap() {
                corrupted = true;
                break;
            }
        }
        if corrupted {
            break;
        }
        for c in chars.into_iter().rev() {
            s *= 5;
            s += score(c);
        }
        scores.push(s);
    }
    median(scores)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 26397);
    }

    #[test]
    fn part_two() {
        let input = include_str!("input_test");
        assert_eq!(two(input), 288957);
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", one(input));
    println!("{}", two(input));
}
