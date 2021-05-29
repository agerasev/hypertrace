pub struct Image<T: Copy + Default, const N: usize> {
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T: Copy + Default, const N: usize> Image<T, N> {
    pub fn new((width, height): (usize, usize)) -> Self {
        Self {
            shape: (width, height),
            data: vec![T::default(); N * width * height],
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
    pub fn width(&self) -> usize {
        self.shape.0
    }
    pub fn height(&self) -> usize {
        self.shape.1
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
