use std::fmt;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct Octopus {
    pub energy: u32,
    pub flashed: bool,
}

impl AddAssign<u32> for Octopus {
    fn add_assign(&mut self, rhs: u32) {
        self.energy += rhs;
    }
}

impl fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.energy)
    }
}

impl From<u32> for Octopus {
    fn from(u: u32) -> Self {
        Octopus {
            energy: u,
            flashed: false,
        }
    }
}

impl Octopus {
    pub fn new() -> Self {
        Octopus {
            energy: 0,
            flashed: false,
        }
    }
    pub fn flash(&mut self) {
        self.flashed = true;
    }
    pub fn reset(&mut self) {
        self.flashed = false;
    }
}
