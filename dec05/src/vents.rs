use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct OceanFloor {
    points: HashMap<Point, i32>,
}

impl OceanFloor {
    pub fn count_overlapping(self) -> i32 {
        let mut n_overlaps = 0;
        for (_, n) in self.points {
            if n >= 2 {
                n_overlaps += 1;
            }
        }
        n_overlaps
    }
}

impl fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut floor = [[0; 10]; 10];
        for (p, n) in &self.points {
            floor[p.x as usize][p.y as usize] = *n;
        }
        for row in floor {
            for c in row {
                match c {
                    0 => write!(f, ".")?,
                    _ => write!(f, "{}", c)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for OceanFloor {
    fn from(s: &str) -> Self {
        let mut points = HashMap::new();
        for line in s.lines() {
            let vent = Vent::from(line);
            match vent.kind.unwrap() {
                Kind::Vertical | Kind::Horizontal => match vent.points {
                    Some(pts) => {
                        for p in pts {
                            let counter = points.entry(p).or_insert(0);
                            *counter += 1;
                        }
                    }
                    None => panic!("No points in this vent."),
                },
                _ => {}
            }
        }
        OceanFloor { points }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Kind {
    Vertical,
    Horizontal,
    Diagonal45,
    Diagonal,
}

#[derive(PartialEq, Debug)]
pub struct Vent {
    pub start: Point,
    pub end: Point,
    pub kind: Option<Kind>,
    pub points: Option<Vec<Point>>,
}

impl Vent {
    pub fn new(start: Point, end: Point) -> Self {
        Vent {
            start: start,
            end: end,
            kind: None,
            points: None,
        }
    }
    pub fn set_kind(&mut self) {
        if self.start.x == self.end.x {
            self.kind = Some(Kind::Horizontal);
        } else if self.start.y == self.end.y {
            self.kind = Some(Kind::Vertical);
        } else {
            match (self.start.x - self.end.x) / (self.start.y - self.end.y) {
                1 | -1 => self.kind = Some(Kind::Diagonal45),
                _ => self.kind = Some(Kind::Diagonal),
            }
        }
    }
    pub fn fill_ho_ve(&mut self) {
        match self.kind {
            None => {
                self.set_kind();
                self.fill_ho_ve();
            }
            _ => {}
        }
        match self.kind.unwrap() {
            Kind::Horizontal => {
                let mut points: Vec<Point> = Vec::new();
                let x = self.start.x;
                let mut y_min = self.start.y;
                let mut y_max = self.end.y;
                if y_min > y_max {
                    let t = y_min;
                    y_min = y_max;
                    y_max = t;
                }
                for y in y_min..=y_max {
                    points.push(Point::new(x, y));
                }
                self.points = Some(points);
            }
            Kind::Vertical => {
                let mut points: Vec<Point> = Vec::new();
                let y = self.start.y;
                let mut x_min = self.start.x;
                let mut x_max = self.end.x;
                if x_min > x_max {
                    let t = x_min;
                    x_min = x_max;
                    x_max = t;
                }
                for x in x_min..=x_max {
                    points.push(Point::new(x, y));
                }
                self.points = Some(points);
            }
            _ => {}
        };
    }
    pub fn fill_dia(&mut self) {
        match self.kind {
            None => {
                self.set_kind();
                self.fill_dia();
            }
            _ => {}
        }
        match self.kind.unwrap() {
            Kind::Diagonal45 => {
                let mut points: Vec<Point> = Vec::new();
                let mut x_min = self.start.x;
                let mut x_max = self.end.x;
                if x_min > x_max {
                    let t = x_min;
                    x_min = x_max;
                    x_max = t;
                }
                let mut y_min = self.start.y;
                let mut y_max = self.end.y;
                if y_min > y_max {
                    let t = y_min;
                    y_min = y_max;
                    y_max = t;
                }
                for (x, y) in (x_min..=x_max).zip(y_min..=y_max) {
                    points.push(Point::new(x, y));
                }
                self.points = Some(points);
            }
            Kind::Diagonal => {
                self.points = Some(vec![]);
            }
            _ => {}
        };
    }
}

impl From<&str> for Vent {
    fn from(s: &str) -> Self {
        let mut pts = s.split(" -> ");
        let start: Point = pts.next().unwrap().into();
        let end: Point = pts.next().unwrap().into();
        let mut vent = Vent::new(start, end);
        vent.set_kind();
        vent.fill_ho_ve();
        vent.fill_dia();
        vent
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut xy = s.split(',').map(str::parse::<i32>);
        let x = xy.next().unwrap().unwrap();
        let y = xy.next().unwrap().unwrap();
        Point::new(x, y)
    }
}
