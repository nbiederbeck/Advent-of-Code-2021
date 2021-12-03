#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        let er = epsilon_rate(input);
        let e: u32 = er.into();
        let er = epsilon_rate(input);
        let g: u32 = er.flip().into();
        assert_eq!(g, 9);
        assert_eq!(e, 22);
    }
    // #[test]
    // fn part_two() {}

    #[test]
    fn bitvec_to_int() {
        let b: u32 = BitVec { bits: vec![0] }.into();
        assert_eq!(b, 0);
        let b: u32 = BitVec { bits: vec![1] }.into();
        assert_eq!(b, 1);
        let b: u32 = BitVec { bits: vec![1, 1] }.into();
        assert_eq!(b, 3);
        let b: u32 = BitVec {
            bits: vec![1, 0, 0, 0],
        }
        .into();
        assert_eq!(b, 8);
    }
}

#[derive(Debug, Clone)]
struct BitVec {
    bits: Vec<u32>,
}

impl Into<u32> for BitVec {
    fn into(self) -> u32 {
        let mut s: u32 = 0;
        for (i, n) in self.bits.iter().rev().enumerate() {
            let j = i as u32;
            s += n * 2_i32.pow(j.into()) as u32;
        }
        s
    }
}

impl BitVec {
    fn new() -> Self {
        BitVec { bits: Vec::new() }
    }
    fn flip(self) -> Self {
        let mut new = BitVec::new();
        for b in self.bits {
            match b {
                1 => new.bits.push(0),
                0 => new.bits.push(1),
                _ => panic!("no binary vec"),
            }
        }
        new
    }
}

fn epsilon_rate(input: &str) -> BitVec {
    let digits: usize = input.lines().nth(0).expect("No first line in input.").len();
    let mut epsilon: Vec<u32> = Vec::new();

    let mut n_lines: u32 = 0;
    for i in 0..digits {
        epsilon.push(0);
        for n in input.chars().skip(i).step_by(digits + 1) {
            epsilon[i] += n.to_digit(10).expect("No number in input.");
            n_lines += 1;
        }
    }
    n_lines /= digits as u32;

    let mut binary_epsilon = BitVec { bits: Vec::new() };

    for g in epsilon {
        binary_epsilon
            .bits
            .push((g as f32 / n_lines as f32).round() as u32);
    }
    binary_epsilon
}

fn main() {
    let input = include_str!("input");
    let er = epsilon_rate(input);
    let e: u32 = er.into();
    let er = epsilon_rate(input);
    let g: u32 = er.flip().into();
    println!("{}", e * g);
}
