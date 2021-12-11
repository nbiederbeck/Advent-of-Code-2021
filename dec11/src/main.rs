mod octopus;
mod octopuses;
use crate::octopuses::Octopuses;

const NROWS: usize = 10;
const MAX: usize = NROWS - 1;

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 1656);
    }
    #[test]
    fn part_two() {
        let input = include_str!("input_test");
        assert_eq!(two(input), 195);
    }
}

fn one(input: &str) -> u32 {
    let mut o = Octopuses::from(input);
    for _ in 0..100 {
        o.step();
    }
    o.flashes
}

fn two(input: &str) -> u32 {
    let mut o = Octopuses::from(input);
    let mut steps = 0;
    while !o.all_flash() {
        steps += 1;
        o.step();
    }
    steps
}

fn main() {
    let input = include_str!("input");
    println!("{}", one(input));
    println!("{}", two(input));
}
