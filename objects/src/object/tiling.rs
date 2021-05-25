pub trait Tiling: 'static {}

pub struct Uniform;
impl Tiling for Uniform {}

pub struct Square;
impl Tiling for Square {}

pub struct Pentagonal;
impl Tiling for Pentagonal {}

pub struct Pentastar;
impl Tiling for Pentastar {}

pub struct Hexagonal;
impl Tiling for Hexagonal {}
