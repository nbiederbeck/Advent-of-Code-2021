#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 26);
    }
}

enum Segment {
    On,
    Off
}

struct SevenSegmentDisplay {
    segments: [Segment; 7],
}

struct Line {
    patterns: [SevenSegmentDisplay; 10],
    output: [SevenSegmentDisplay; 4],
}

impl From<&str> for Line {
    fn frmo(s: &str) -> Self {
        let it = s.split('|');
        let patterns_it = it.next().unwrap();
        let output_it = it.next().unwrap();
        let mut patterns: [SevenSegmentDisplay; 10] = 
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    for line in input.lines() {
        lines.push(line::into())
    }
    lines
}

fn one(input: &str) -> u64 {
    0
}


fn main() {
    println!("Hello, world!");
}
