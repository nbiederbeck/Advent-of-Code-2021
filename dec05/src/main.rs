mod vents;
use vents::*;

fn one(input: &str) -> i32 {
    let floor = OceanFloor::from(input);
    println!("{}", &floor);
    floor.count_overlapping()
}

fn two(input: &str) -> i32 {
    let floor = OceanFloor::from(input);
    println!("{}", &floor);
    floor.count_overlapping()
}

#[cfg(test)]
mod test {
    use crate::vents::{Point, Vent};
    use crate::*;

    #[test]
    fn test_one() {
        let input = include_str!("input_test");
        assert_eq!(one(input), 5);
    }

    #[test]
    fn test_two() {
        let input = include_str!("input_test");
        assert_eq!(two(input), 12);
    }

    #[test]
    fn test_point() {
        assert_eq!(Point::from("3,2"), Point { x: 3, y: 2 });
    }

    #[test]
    fn test_vent() {
        let mut vent = Vent::new(Point::new(3, 2), Point::new(1, 2));
        vent.fill_ho_ve();
        assert_eq!(vent.kind, Some(Kind::Vertical));
        assert_eq!(Vent::from("3,2 -> 1,2"), vent);
    }
}

fn main() {
    let input = include_str!("input_test");
    println!("{}", one(input));
    println!("{}", two(input));
}
