use std::{
    io::{self, Read, Write},
    hash::Hash,
};
use crate::{config::*, type_::*};

pub struct VecType<T: Type, const N: usize> {
    pub element: T,
}
pub struct MatType<T: Type, const N: usize> {
    pub element: T,
}
