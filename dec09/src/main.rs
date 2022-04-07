use std::cmp::Ord;

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn one_test() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 15)
    }
}

struct HeightMap {
    heights: Vec<Vec<u32>>,
}

type LowPoints = Vec<u32>;

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let mut heights = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap());
            }
            heights.push(row);
        }
        Self { heights }
    }
}

struct Neighbors([u32; 4]);

impl Ord<u32> for Neighbors {}

impl HeightMap {
    fn find_low_points(&self) -> LowPoints {
        let max_length = self.length() - 1;
        let max_width = self.width() - 1;
        for i in 0..=max_length {
            for j in 0..=max_width {
                match (i, j) {
                    (0, _) => {}
                    (_, 0) => {}
                    (max_length, _) => {}
                    (_, max_width) => {}
                    _ => {}
                }
            }
        }
        todo!();
    }
    fn length(&self) -> usize {
        self.heights.len()
    }
    fn width(&self) -> usize {
        self.heights[0].len()
    }
}

fn parse_input(input: &str) -> HeightMap {
    HeightMap::from(input)
}

fn risk_level(points: LowPoints) -> u32 {
    points.iter().map(|n| n + 1).sum()
}

fn one(input: &str) -> u32 {
    let heights = parse_input(input);
    let points = heights.find_low_points();
    risk_level(points)
}

fn main() {
    let input = include_str!("input");
    println!("{}", one(input));
}
