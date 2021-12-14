use std::collections::HashSet;
use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Dot {
    x: u32,
    y: u32,
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<&str> for Dot {
    fn from(s: &str) -> Self {
        let mut it = s.split(',');
        let x: u32 = it.next().unwrap().parse().unwrap();
        let y: u32 = it.next().unwrap().parse().unwrap();
        Self { x, y }
    }
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<Dot>,
}

impl Paper {
    fn fold(&mut self, instruction: Instruction) {
        match instruction.coordinate {
            Coordinate::X => {
                for dot in self.dots.drain().collect::<Vec<Dot>>().iter() {
                    if dot.x > instruction.value {
                        self.dots.insert(Dot {
                            x: 2 * instruction.value - dot.x,
                            y: dot.y,
                        });
                    } else {
                        self.dots.insert(*dot);
                    }
                }
            }
            Coordinate::Y => {
                for dot in self.dots.drain().collect::<Vec<Dot>>().iter() {
                    if dot.y > instruction.value {
                        self.dots.insert(Dot {
                            x: dot.x,
                            y: 2 * instruction.value - dot.y,
                        });
                    } else {
                        self.dots.insert(*dot);
                    }
                }
            }
        }
    }
    fn visible_dots(self) -> u32 {
        self.dots.len() as u32
    }
}

impl From<&str> for Paper {
    fn from(s: &str) -> Self {
        Self {
            dots: s.lines().map(|l| l.into()).collect(),
        }
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut paper = [[' '; 40]; 6];
        for dot in &self.dots {
            paper[dot.y as usize][dot.x as usize] = '#';
        }
        for line in paper.iter() {
            for col in line.iter() {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Coordinate {
    X,
    Y,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        match s {
            "x" => Coordinate::X,
            "y" => Coordinate::Y,
            _ => panic!("No valid Coordinate."),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    coordinate: Coordinate,
    value: u32,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut it = s.split_ascii_whitespace().nth(2).unwrap().split('=');
        let coordinate: Coordinate = it.next().unwrap().into();
        let value: u32 = it.next().unwrap().parse().unwrap();
        Self { coordinate, value }
    }
}

fn parse_input(input: &str) -> Result<(Paper, Vec<Instruction>), Box<dyn Error>> {
    let mut it = input.split("\n\n");
    let paper: Paper = it.next().unwrap().into();
    let instructions: Vec<Instruction> = it.next().unwrap().lines().map(|l| l.into()).collect();
    Ok((paper, instructions))
}

fn one(input: &str) -> u32 {
    let (mut paper, instructions) = parse_input(input).unwrap();
    if let Some(i) = instructions.into_iter().next() {
        paper.fold(i);
    }
    paper.visible_dots()
}

fn two(input: &str) -> Paper {
    let (mut paper, instructions) = parse_input(input).unwrap();
    for i in instructions {
        paper.fold(i);
    }
    paper
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_one() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 17);
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", one(input));
    println!("{}", two(input));
}
