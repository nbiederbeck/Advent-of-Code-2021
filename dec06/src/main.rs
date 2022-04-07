const AGES: usize = 9;

/// Fish that stay together for social reasons are shoaling:
/// https://en.m.wikipedia.org/wiki/Shoaling_and_schooling
#[derive(Debug, Clone)]
struct Shoal {
    fish: [u64; AGES],
}

impl Shoal {
    fn evolve(&mut self) {
        let new_fish = self.fish[0];
        self.fish[0] = self.fish[1];
        self.fish[1] = self.fish[2];
        self.fish[2] = self.fish[3];
        self.fish[3] = self.fish[4];
        self.fish[4] = self.fish[5];
        self.fish[5] = self.fish[6];
        self.fish[6] = self.fish[7] + new_fish;
        self.fish[7] = self.fish[8];
        self.fish[8] = new_fish;
    }
    fn evolve_for_days(&mut self, days: usize) {
        for _ in 0..days {
            self.evolve();
        }
    }
}

impl From<&str> for Shoal {
    fn from(s: &str) -> Self {
        let mut fish = [0; AGES];

        for age in s.split(',').map(str::parse::<usize>) {
            match age {
                Ok(a) => {
                    fish[a] += 1;
                }
                _ => {}
            }
        }

        Self { fish }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        let input = include_str!("input_test");
        assert_eq!(one(input, 18), 26);
        assert_eq!(one(input, 80), 5934);
        assert_eq!(two(input), 26984457539);
    }
}

fn one(input: &str, days: usize) -> u64 {
    let mut shoal = Shoal::from(input);
    shoal.evolve_for_days(days);
    shoal.fish.iter().sum()
}

fn two(input: &str) -> u64 {
    one(input, 256)
}

fn main() {
    let input = include_str!("input");
    println!("{}", one(input, 80));
    println!("{}", two(input));
}
