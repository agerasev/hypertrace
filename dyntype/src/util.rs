pub use num_integer::{gcd, lcm};
pub use std::cmp::{max, min};

use num_integer::div_floor;

pub fn upper_multiple(m: usize, x: usize) -> usize {
    m * div_floor(x + m - 1, m)
}
