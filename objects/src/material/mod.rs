mod basic;

use types::{Named, SizedEntity, Sourced};

pub trait Material: Named + SizedEntity + Sourced {}

pub use basic::*;
