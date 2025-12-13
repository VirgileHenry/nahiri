mod error;
mod layer;
mod vector;

pub use error::BuildError as HsnwBuildError;
pub use vector::Vector;

struct Node<T, const DIM: usize, const L0: usize, const L1: usize, const L2: usize, const L3: usize> {
    vector: crate::Vector<DIM>,
    data: T,
    layer: layer::Layer<L0, L1, L2, L3>,
}

impl<T, const DIM: usize, const L0: usize, const L1: usize, const L2: usize, const L3: usize> Node<T, DIM, L0, L1, L2, L3> {
    fn compute_l0(&mut self, lower: &[Self], upper: &[Self]) {
        /* Create a function to index lower & upper */
        let get_node = |index: usize| -> &Self {
            if index < lower.len() {
                &lower[index]
            } else {
                &upper[index - lower.len() - 1]
            }
        };

        /* Start by sorting our existing l0 */
        self.layer.l0_mut().sort_by(|idx1, idx2| {
            let node1 = get_node(*idx1);
            let node2 = get_node(*idx2);
            let distance_1 = node1.vector.euclidian_distance_sq(&self.vector);
            let distance_2 = node2.vector.euclidian_distance_sq(&self.vector);
            distance_1.total_cmp(&distance_2)
        });

        /* We already know we have the first L0 + 0/1 data points sorted. */
        /* Insert the rest of the possible data points in the sorted list */

        let lower_nodes_and_indices = lower.iter().enumerate();
        let upper_nodes_and_indices = upper.iter().enumerate().map(|(idx, node)| (idx + lower.len() + 1, node));
        let nodes_and_indices = lower_nodes_and_indices.chain(upper_nodes_and_indices);

        for (node_index, node) in nodes_and_indices {
            if node_index < L0 {
                /* Node was already there in the start l0, no need to reprocess (might even create duplicates) */
                continue;
            }

            /* Find the spot where to insert the new node, using a binary search */
            let candidate_distance = self.vector.euclidian_distance_sq(&node.vector);
            let insert_index = self.layer.l0().binary_search_by(|n_idx| {
                let distance = self.vector.euclidian_distance_sq(&get_node(*n_idx).vector);
                distance.total_cmp(&candidate_distance)
            });
            let insert_index = match insert_index {
                Ok(idx) | Err(idx) => idx,
            };

            /* If the insert index is smaller than the layer size, we should insert it */
            if insert_index < L0 {
                /* Shift all elems after the insert index left */
                let mut prev_elem = 0;
                for elem in &mut self.layer.l0_mut()[insert_index..] {
                    let temp = *elem;
                    *elem = prev_elem;
                    prev_elem = temp;
                }
                /* And insert the new index at the insert index */
                self.layer.l0_mut()[insert_index] = node_index;
            }
        }

        /* Done ! */
    }
}

pub struct Hsnw<K, T, const DIM: usize, const L0: usize, const L1: usize, const L2: usize, const L3: usize> {
    nodes: Vec<Node<T, DIM, L0, L1, L2, L3>>,
    lookup: std::collections::HashMap<K, usize>,
}

impl<K, T, const DIM: usize, const L0: usize, const L1: usize, const L2: usize, const L3: usize> Hsnw<K, T, DIM, L0, L1, L2, L3>
where
    K: Eq + std::hash::Hash,
    T: Clone,
{
    pub fn new<F: Fn(&T) -> K>(data_points: &[(crate::Vector<DIM>, T)], get_key_f: F) -> Result<Self, error::BuildError> {
        /* Sanity check: we need at least enough points for each cache layer, and at least one for the entry point */
        if data_points.is_empty() {
            return Err(error::BuildError::not_enough_data_points(1, "entry point", 0));
        }
        if data_points.len() <= L0 {
            return Err(error::BuildError::not_enough_data_points(L0 + 1, "L0", data_points.len()));
        }
        if data_points.len() <= L1 {
            return Err(error::BuildError::not_enough_data_points(L1 + 1, "L1", data_points.len()));
        }
        if data_points.len() <= L2 {
            return Err(error::BuildError::not_enough_data_points(L2 + 1, "L2", data_points.len()));
        }
        if data_points.len() <= L3 {
            return Err(error::BuildError::not_enough_data_points(L3 + 1, "L3", data_points.len()));
        }

        /* For now, I'm putting every node at L0, so its nsw and not hnsw */
        /* initialize with dummy L0 */
        let mut nodes: Vec<_> = data_points
            .iter()
            .enumerate()
            .map(|(index, (vector, data))| Node {
                vector: vector.clone(),
                data: data.clone(),
                layer: layer::Layer::Layer0 {
                    l0_neighbors: std::array::from_fn(|i| if i < index { i } else { i + 1 }),
                },
            })
            .collect();

        /* For each node, get the L0 closest neighbors */
        for i in 0..nodes.len() {
            let (lower, rest) = nodes.split_at_mut(i);
            let (node, upper) = rest.split_at_mut(1);
            /* SAFETY: with the split at mut 1, node will always be a slice of one elem */
            let [node] = node else { unreachable!() };
            node.compute_l0(lower, upper);
        }

        /* Build the lookup map */
        let lookup: std::collections::HashMap<K, usize> = nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (get_key_f(&node.data), index))
            .collect();

        Ok(Self { nodes, lookup })
    }
}

impl<K, T, const DIM: usize, const L0: usize, const L1: usize, const L2: usize, const L3: usize> Hsnw<K, T, DIM, L0, L1, L2, L3>
where
    K: Eq + std::hash::Hash,
{
    pub fn closest_neighbors_from_key<'f, 'd: 'f, F: Fn(&T) -> bool + 'f>(
        &'d self,
        key: &K,
        max_results: usize,
        filter: Option<F>,
    ) -> Option<impl Iterator<Item = &'d T> + 'f> {
        /* Start by looking up the exact search point by key */
        let start_point_index = self.lookup.get(key)?;
        let start_node = &self.nodes[*start_point_index];

        /* Then, we iterate over all l0 neighbors and retrieve the data */
        let closest_neighbors = start_node.layer.l0().iter();
        let closest_data = closest_neighbors.map(|neighbor_index| &self.nodes[*neighbor_index].data);

        /* When a filter is used, apply it to filter the results */
        let filtered_closest_data = closest_data.filter(move |data| match &filter {
            Some(filter) => filter(data),
            None => true, /* No filter, keep it all */
        });

        /* Finally, take only the closest points. */
        let result = filtered_closest_data.take(max_results);

        /* The result are inexact, when the filter is too restrictive or too many points are requested, we need to explore further. */
        Some(result)
    }
}
