pub trait Tiling: 'static {}

#[derive(Clone, Copy, Debug)]
pub struct Uniform;
impl Tiling for Uniform {}

#[derive(Clone, Copy, Debug)]
pub struct Square;
impl Tiling for Square {}

#[derive(Clone, Copy, Debug)]
pub struct Pentagonal;
impl Tiling for Pentagonal {}

#[derive(Clone, Copy, Debug)]
pub struct Pentastar;
impl Tiling for Pentastar {}

#[derive(Clone, Copy, Debug)]
pub struct Hexagonal;
impl Tiling for Hexagonal {}
