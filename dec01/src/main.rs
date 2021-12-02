use itertools::Itertools;
use std::error::Error;

#[cfg(test)]
mod test {
    #[test]
    fn part_one() {
        let a = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(super::part_one(&a), 7)
    }
    #[test]
    fn part_two() {
        let a = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(super::part_two(&a), 5)
    }
}

fn part_one(a: &[i64]) -> usize {
    let mut increasing = 0;

    let it = a.iter().tuple_windows::<(_, _)>();

    for (x, y) in it {
        if y > x {
            increasing += 1;
        }
    }

    increasing
}

fn part_two(a: &[i64]) -> usize {
    let mut increasing = 0;

    let it = a.iter().tuple_windows::<(_, _, _)>();

    for (first, second) in it.zip(a.iter().skip(1).tuple_windows::<(_, _, _)>()) {
        let (xa, ya, za) = first;
        let (xb, yb, zb) = second;
        let suma = xa + ya + za;
        let sumb = xb + yb + zb;
        if sumb > suma {
            increasing += 1;
        }
    }

    increasing
}

fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<i64> = include_str!("input")
        .lines()
        .map(str::parse::<i64>)
        .map(|s| s.unwrap_or(0))
        .collect();

    println!("{}", part_one(&a));
    println!("{}", part_two(&a));

    Ok(())
}
