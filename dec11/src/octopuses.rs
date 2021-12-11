use crate::octopus::Octopus;
use crate::MAX;
use crate::NROWS;
use std::fmt;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct Octopuses {
    pub dumbos: [[Octopus; NROWS]; NROWS],
    pub flashes: u32,
}

impl Octopuses {
    pub fn all_flash(self) -> bool {
        let mut n_flashes = 0;
        for row in self.dumbos {
            for d in row {
                if d.flashed {
                    n_flashes += 1;
                }
            }
        }
        n_flashes == NROWS * NROWS
    }
    pub fn reset(&mut self) {
        for (x, row) in self.dumbos.into_iter().enumerate() {
            for (y, _) in row.into_iter().enumerate() {
                self.dumbos[x][y].reset();
            }
        }
    }
    fn flash(&mut self) -> bool {
        let old_flashes = self.flashes;
        for (x, row) in self.dumbos.into_iter().enumerate() {
            for (y, d) in row.into_iter().enumerate() {
                if d.energy > 9 && !d.flashed {
                    self.dumbos[x][y].flash();
                    self.flashes += 1;
                    match (x, y) {
                        // 3 neighbors
                        (0, 0) => {
                            self.dumbos[x][y + 1] += 1;
                            self.dumbos[x + 1][y] += 1;
                            self.dumbos[x + 1][y + 1] += 1;
                        }
                        (0, MAX) => {
                            self.dumbos[x][y - 1] += 1;
                            self.dumbos[x + 1][y] += 1;
                            self.dumbos[x + 1][y - 1] += 1;
                        }
                        (MAX, 0) => {
                            self.dumbos[x][y + 1] += 1;
                            self.dumbos[x - 1][y] += 1;
                            self.dumbos[x - 1][y + 1] += 1;
                        }
                        (MAX, MAX) => {
                            self.dumbos[x][y - 1] += 1;
                            self.dumbos[x - 1][y] += 1;
                            self.dumbos[x - 1][y - 1] += 1;
                        }
                        // 5 neighbors
                        (0, _) => {
                            self.dumbos[x][y - 1] += 1;
                            self.dumbos[x][y + 1] += 1;
                            self.dumbos[x + 1][y - 1] += 1;
                            self.dumbos[x + 1][y] += 1;
                            self.dumbos[x + 1][y + 1] += 1;
                        }
                        (MAX, _) => {
                            self.dumbos[x][y - 1] += 1;
                            self.dumbos[x][y + 1] += 1;
                            self.dumbos[x - 1][y - 1] += 1;
                            self.dumbos[x - 1][y] += 1;
                            self.dumbos[x - 1][y + 1] += 1;
                        }
                        (_, 0) => {
                            self.dumbos[x - 1][y] += 1;
                            self.dumbos[x + 1][y] += 1;
                            self.dumbos[x - 1][y + 1] += 1;
                            self.dumbos[x][y + 1] += 1;
                            self.dumbos[x + 1][y + 1] += 1;
                        }
                        (_, MAX) => {
                            self.dumbos[x - 1][y] += 1;
                            self.dumbos[x + 1][y] += 1;
                            self.dumbos[x - 1][y - 1] += 1;
                            self.dumbos[x][y - 1] += 1;
                            self.dumbos[x + 1][y - 1] += 1;
                        }
                        // 8 neighbors
                        _ => {
                            self.dumbos[x][y - 1] += 1;
                            self.dumbos[x][y + 1] += 1;
                            self.dumbos[x - 1][y] += 1;
                            self.dumbos[x + 1][y] += 1;
                            self.dumbos[x - 1][y - 1] += 1;
                            self.dumbos[x + 1][y - 1] += 1;
                            self.dumbos[x - 1][y + 1] += 1;
                            self.dumbos[x + 1][y + 1] += 1;
                        }
                    }
                }
            }
        }
        self.flashes > old_flashes
    }
    pub fn step(&mut self) {
        self.reset();
        *self += 1;
        while self.flash() {}
        // Set to Zero
        for (x, row) in self.dumbos.into_iter().enumerate() {
            for (y, d) in row.into_iter().enumerate() {
                if d.energy > 9 {
                    self.dumbos[x][y].energy = 0;
                }
            }
        }
    }
}

impl From<&str> for Octopuses {
    fn from(s: &str) -> Self {
        let mut dumbos = [[Octopus::new(); NROWS]; NROWS];
        for (i, row) in s.lines().enumerate() {
            for (j, col) in row.chars().enumerate() {
                match col.to_digit(10) {
                    Some(c) => dumbos[i][j].energy = c,
                    None => panic!("{}", col),
                }
            }
        }
        Octopuses { dumbos, flashes: 0 }
    }
}

impl fmt::Display for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} flashes", self.flashes)?;
        for row in self.dumbos {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl AddAssign<u32> for Octopuses {
    fn add_assign(&mut self, rhs: u32) {
        for (x, row) in self.dumbos.into_iter().enumerate() {
            for (y, _d) in row.into_iter().enumerate() {
                self.dumbos[x][y] += rhs;
            }
        }
    }
}
