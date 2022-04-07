#[derive(Debug)]
struct Point<'a> {
    neighbors: [Option<&'a Point<'a>>; 4],
    visited: bool,
    value: u8,
}

impl Point<'_> {
    fn new() -> Self {
        Self {
            neighbors: [None; 4],
            value: 0,
            visited: false,
        }
    }
    fn is_visitable(&self) -> bool {
        if self.visited {
            return false;
        } else {
            for n in self.neighbors.iter() {
                match n {
                    Some(neighbor) => {
                        if neighbor.visited {
                            return false;
                        }
                    }
                    None => {}
                }
            }
        }
        true
    }
    fn visit(&mut self) {
        self.visited = true;
    }
}

fn main() {
    let mut point = Point::new();
    let other_point = Point::new();
    point.neighbors[0] = Some(&other_point);
    println!("{:?}", point);
    println!("{}", point.is_visitable());
    point.visit();
    println!("{}", other_point.is_visitable());
}
