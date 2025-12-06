mod vector;

pub use vector::Vector;

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
    /// Find the n closest neighbor using linear search, with a O(n k) complexity,
    /// with n the number of points, and k the max number of results.
    pub fn closest_neighbors(&self, initial_point: Vector<DIM>, max_results: usize) -> Vec<&T> {
        /// Structure to store result points as well as their distances to the target
        struct DistancedData<'a, const DIM: usize, T> {
            distance: f32,
            data: &'a T,
        }

        let mut results: Vec<DistancedData<'_, DIM, T>> = Vec::with_capacity(max_results);

        /* Iterate over all data points and their vectors */
        for (vector, data_point) in self.data_points.iter() {
            /* Compute the distance for this data point to the target */
            let candidate = DistancedData {
                distance: vector.euclidian_distance(&initial_point),
                data: data_point,
            };

            /* Linear search in the sorted result array to place the new candidate */
            let mut insert_index = 0;
            while let Some(result) = results.get(insert_index) {
                if candidate.distance < result.distance {
                    /* When the candidate is better than the current result at that index, it shall be inserted here */
                    break;
                } else {
                    /* Otheriwse, keep looking */
                    insert_index += 1;
                }
            }

            /* Insert the candidate only if it is less than the max number of candidates */
            if insert_index < max_results {
                results.insert(insert_index, candidate);
            }
        }

        results.into_iter().map(|distanced| distanced.data).collect()
    }
}
