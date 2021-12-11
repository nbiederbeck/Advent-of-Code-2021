const VALID_CHARS: [char; 4] = ['(', '[', '{', '<'];

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
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

fn one(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if VALID_CHARS.contains(&c) {
                chars.push(c);
            } else if opening(c) != chars.pop().unwrap() {
                s += score(c);
                break;
            }
        }
    }
    s
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 26397);
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", one(input));
}
