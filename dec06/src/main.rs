#[derive(Debug, Clone, Copy)]
struct Lanternfish {
    age: u8,
}

impl Lanternfish {
    fn new() -> Self {
        Lanternfish { age: 8 }
    }
    fn with_age(age: u8) -> Self {
        Lanternfish { age }
    }
}

/// Fish that stay together for social reasons are shoaling:
/// https://en.m.wikipedia.org/wiki/Shoaling_and_schooling
#[derive(Debug, Clone)]
struct Shoal {
    fish: Vec<Lanternfish>,
}

impl Shoal {
    fn new() -> Self {
        Shoal {
            fish: Vec::<Lanternfish>::new(),
        }
    }
    fn evolve(&mut self) {
        let mut new_fish = Vec::<Lanternfish>::new();

        for &fish in &self.fish {
            match fish.age {
                0 => {
                    new_fish.push(Lanternfish::new());
                    new_fish.push(Lanternfish::with_age(6));
                }
                _ => new_fish.push(Lanternfish::with_age(fish.age - 1)),
            }
        }

        self.fish = new_fish;
    }
    fn evolve_for_days(&mut self, days: u8) {
        for d in 0..days {
            println!("{}/{}", d, days);
            self.evolve();
        }
    }
}

impl From<&str> for Shoal {
    fn from(s: &str) -> Shoal {
        let mut shoal = Shoal::new();

        for age in s.split(',').map(str::parse::<u8>) {
            match age {
                Ok(a) => {
                    shoal.fish.push(Lanternfish::with_age(a));
                }
                _ => println!("No More Fish in Shoal."),
            }
        }

        shoal
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one() {
        let input = include_str!("input_test");
        let mut shoal = Shoal::from(input);
        shoal.evolve_for_days(18);
        assert_eq!(shoal.fish.len(), 26);
        shoal.evolve_for_days(80 - 18);
        assert_eq!(shoal.fish.len(), 5934);
    }
}

fn main() {
    let input = include_str!("input");
    let mut shoal = Shoal::from(input);

    shoal.evolve_for_days(80);
    println!("Part one: {}", shoal.fish.len());

    println!("Part two is taking time. Will skip.")
}
