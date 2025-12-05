#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const DIM: usize> {
    data: [f32; DIM],
}

impl<const DIM: usize> Vector<DIM> {
    pub fn new(data: [f32; DIM]) -> Self {
        Self { data }
    }

    fn euclidian_distance(&self, other: &Self) -> f32 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

/// In memory vector database.
///
/// Nahiri vector databased was originally designed for the brainstorm project,
/// and therefore is heaviliy opinionated and not super generic.
///
/// the aimed set of features are:
/// - Fast exact nearest neighbor search
/// - Clustering retrievel
/// - Path detection
pub struct VectorDatabase<const DIM: usize, T> {
    data_points: Vec<(Vector<DIM>, T)>,
}

impl<const DIM: usize, T: Clone> VectorDatabase<DIM, T> {
    pub fn new(data_points: &[([f32; DIM], T)]) -> Self {
        let data_points = data_points
            .iter()
            .map(|(vector_data, data_point)| (Vector::new(vector_data.clone()), data_point.clone()))
            .collect();
        Self { data_points }
    }
}

impl<const DIM: usize, T> VectorDatabase<DIM, T> {
    /// Find the n closest neighbor using linear search, with a O(n log(n)) complexity.
    pub fn closest_neighbors(&self, initial_point: Vector<DIM>, max_results: usize) -> Vec<&T> {
        let mut results: Vec<(f32, &T)> = Vec::with_capacity(max_results);

        /* Iterate over all data points and their vectors */
        for (vector, data_point) in self.data_points.iter() {
            /* Compute the distance for this data point to the target */
            let distance = initial_point.euclidian_distance(vector);

            /* Use binary search in the result to find where we should insert the new point, based on the distance */
            match results.binary_search_by(|(d, _)| distance.total_cmp(d)) {
                Ok(index) | Err(index) => {
                    if results.len() < max_results {
                        /* If the result vec is not yet full, we can simply insert at the given index */
                        results.insert(index, (distance, data_point));
                    } else {
                        /* The index returned by the binary search is not the vec end, so we have a new best candidate */
                        if index < results.len() {
                            results.pop();
                            results.insert(index, (distance, data_point));
                        }
                        /* Otherwise, this data point is worse than our curent best candidates, discard it */
                    }
                }
            }
        }

        results.into_iter().map(|(_, data)| data).collect()
    }
}
