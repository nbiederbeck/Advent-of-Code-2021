#[derive(Debug, Clone)]
struct Crab {
    positions: Vec<u64>,
}

impl From<&str> for Crab {
    fn from(s: &str) -> Self {
        let mut positions = Vec::new();
        for c in s.split(',').map(str::parse::<u64>) {
            match c {
                Ok(n) => positions.push(n),
                _ => {}
            }
        }
        Crab { positions }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        let crabs = Crab::from(input);
        assert_eq!(mean(&crabs.positions), 2);
        assert_eq!(fuel(&crabs.positions, 2), 37);
    }
}

fn mean(a: &Vec<u64>) -> u64 {
    let n: u64 = a.len() as u64;
    let mut s = 0;
    for el in a {
        s += el;
    }
    dbg!(s);
    (s as f64 / (n as f64)) as u64
}

fn fuel(a: &Vec<u64>, mean: u64) -> u64 {
    0
}

fn main() {
    println!("Hello, world!");
}
