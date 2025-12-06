#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const DIM: usize> {
    data: [f32; DIM],
}

impl<const DIM: usize> Vector<DIM> {
    pub fn new(data: [f32; DIM]) -> Self {
        Self { data }
    }

    pub fn euclidian_distance(&self, other: &Self) -> f32 {
        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }
}

impl<const DIM: usize> serde::Serialize for Vector<DIM> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(DIM))?;
        for value in self.data.iter() {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}

impl<'de, const DIM: usize> serde::Deserialize<'de> for Vector<DIM> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Visitor used to deserialize the vector.
        struct VectorVisitor<const DIM: usize>;

        impl<'de, const DIM: usize> serde::de::Visitor<'de> for VectorVisitor<DIM> {
            type Value = [f32; DIM];
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A {DIM}-dimension array of f32")
            }
        }

        let data = deserializer.deserialize_seq(VectorVisitor::<DIM>)?;
        Ok(Vector { data })
    }
}
