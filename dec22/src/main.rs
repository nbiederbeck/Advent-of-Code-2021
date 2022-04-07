use std::collections::HashSet;

/// Help
fn main() {
    let (min, max) = (-50, 50);
    let mut s: HashSet<(i32, i32, i32)> = HashSet::with_capacity(1000000);
    for _ in 0..100 {
        for x in min..=max {
            for y in min..=max {
                for z in min..=max {
                    s.insert((x, y, z));
                }
            }
        }
    }
}
