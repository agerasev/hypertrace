pub trait Tiling: 'static {}

pub struct Uniform;
impl Tiling for Uniform {}

pub struct Square;
impl Tiling for Square {}

pub struct Hexagonal;
impl Tiling for Hexagonal {}
