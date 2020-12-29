pub use std::cmp::{min, max};
pub use num_integer::{gcd, lcm};

use num_integer::{div_floor};

pub fn upper_multiple(m: usize, x: usize) -> usize {
    m * div_floor(x + m - 1, m)
}
