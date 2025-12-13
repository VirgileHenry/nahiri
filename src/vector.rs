#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const DIM: usize> {
    data: [f32; DIM],
}

impl<const DIM: usize> Vector<DIM> {
    pub fn new(data: [f32; DIM]) -> Self {
        Self { data }
    }

    pub fn euclidian_distance_sq(&self, other: &Self) -> f32 {
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
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut data = [0.0_f32; DIM];

                for i in 0..DIM {
                    let next = seq.next_element()?;
                    data[i] = next.ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }

                Ok(data)
            }
        }

        let data = deserializer.deserialize_seq(VectorVisitor::<DIM>)?;
        Ok(Vector { data })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        /* Simple test */
        let vector = Vector::new([0.0, 1.0, 2.0]);
        let serialized = serde_json::to_string(&vector).unwrap();
        let deserialized: Vector<3> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(vector, deserialized);

        /* with f32 niche values (I wanted to do more funny stuff, but NaN and infs are 'null' in JSON) */
        let vector = Vector::new([
            0.0,               /* positive zero */
            -0.0,              /* negative zero */
            f32::EPSILON,      /* smallest increment > 1.0 */
            f32::MIN_POSITIVE, /* smallest positive normal number */
        ]);
        let serialized = serde_json::to_string(&vector).unwrap();
        let deserialized: Vector<4> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(vector, deserialized);

        /* Maybe do more test with other formats ? binary ? */
    }
}
