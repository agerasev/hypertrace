mod basic;
mod modifier;

use types::{Named, SizedEntity, Sourced};

pub trait Material: Named + SizedEntity + Sourced {}

pub use basic::*;
pub use modifier::*;
