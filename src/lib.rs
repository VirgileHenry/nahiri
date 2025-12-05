#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const DIM: usize> {
    data: [f32; DIM],
}

impl<const DIM: usize> Vector<DIM> {
    pub fn new(data: [f32; DIM]) -> Self {
        Self { data }
    }
}
