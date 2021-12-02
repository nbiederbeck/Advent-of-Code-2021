#[derive(Debug)]
enum InstructionKind {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Instruction {
    command: InstructionKind,
    units: usize,
}

#[derive(Debug)]
struct Position {
    x: usize,
    depth: usize,
    aim: usize,
}

impl Position {
    fn new() -> Self {
        Position {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn advance_one(&mut self, instruction: &Instruction) {
        match instruction.command {
            InstructionKind::Down => self.depth += instruction.units,
            InstructionKind::Up => self.depth -= instruction.units,
            InstructionKind::Forward => self.x += instruction.units,
        };
    }
    fn advance_two(&mut self, instruction: &Instruction) {
        match instruction.command {
            InstructionKind::Up => self.aim -= instruction.units,
            InstructionKind::Down => self.aim += instruction.units,
            InstructionKind::Forward => {
                self.x += instruction.units;
                self.depth += self.aim * instruction.units;
            }
        }
    }
    fn solution(self) -> usize {
        self.x * self.depth
    }
}

type Course = Vec<Instruction>;

fn parse_course(s: &str) -> Course {
    s.lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            Instruction {
                command: match tokens.next() {
                    Some(tok) => match tok {
                        "forward" => InstructionKind::Forward,
                        "up" => InstructionKind::Up,
                        "down" => InstructionKind::Down,
                        _ => panic!("unknown instruction command {}", tok),
                    },
                    None => panic!("for line {}, expected instruction command", l),
                },
                units: match tokens.next() {
                    Some(tok) => tok.parse().unwrap(),
                    None => panic!("for line {}, expected units", l),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one() {
        let course = parse_course(include_str!("input_test"));
        let mut p = Position::new();
        for instruction in &course {
            p.advance_one(instruction)
        }
        assert_eq!(p.solution(), 150);
    }
    #[test]
    fn part_two() {
        let course = parse_course(include_str!("input_test"));
        let mut p = Position::new();
        for instruction in &course {
            p.advance_two(instruction)
        }
        assert_eq!(p.solution(), 900);
    }
}

fn main() {
    let course = parse_course(include_str!("input"));

    let mut p = Position::new();
    for instruction in &course {
        p.advance_one(instruction)
    }
    println!("{}", p.solution());

    p = Position::new();
    for instruction in &course {
        p.advance_two(instruction)
    }
    println!("{}", p.solution());
}
