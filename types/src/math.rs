pub use num_integer::{gcd, lcm};
pub use std::cmp::{max, min};

use num_integer::div_floor;
use std::mem::size_of;

pub fn upper_multiple(x: usize, m: usize) -> usize {
    m * div_floor(x + m - 1, m)
}

pub fn aligned_add(a: usize, b: usize, m: usize) -> usize {
    upper_multiple(a, m) + b
}

pub fn aligned_max(a: usize, b: usize, m: usize) -> usize {
    upper_multiple(max(a, b), m)
}

pub fn ceil_pow2(mut n: usize) -> usize {
    n -= 1;
    let mut p = 1;
    while p < 8 * size_of::<usize>() {
        n |= n >> p;
        p *= 2;
    }
    n + 1
}

pub fn is_pow2(n: usize) -> bool {
    (n & (n - 1)) == 0
}
