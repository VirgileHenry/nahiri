pub enum Layer<const L0: usize, const L1: usize, const L2: usize, const L3: usize> {
    Layer0 {
        l0_neighbors: [usize; L0],
    },
    #[allow(unused)]
    Layer1 {
        l0_neighbors: [usize; L0],
        l1_neighbors: [usize; L1],
    },
    #[allow(unused)]
    Layer2 {
        l0_neighbors: [usize; L0],
        l1_neighbors: [usize; L1],
        l2_neighbors: [usize; L2],
    },
    #[allow(unused)]
    Layer3 {
        l0_neighbors: [usize; L0],
        l1_neighbors: [usize; L1],
        l2_neighbors: [usize; L2],
        l3_neighbors: [usize; L3],
    },
}

impl<const L0: usize, const L1: usize, const L2: usize, const L3: usize> Layer<L0, L1, L2, L3> {
    pub fn l0(&self) -> &[usize; L0] {
        match self {
            Self::Layer0 { l0_neighbors } => l0_neighbors,
            Self::Layer1 { l0_neighbors, .. } => l0_neighbors,
            Self::Layer2 { l0_neighbors, .. } => l0_neighbors,
            Self::Layer3 { l0_neighbors, .. } => l0_neighbors,
        }
    }

    pub fn l0_mut(&mut self) -> &mut [usize; L0] {
        match self {
            Self::Layer0 { l0_neighbors } => l0_neighbors,
            Self::Layer1 { l0_neighbors, .. } => l0_neighbors,
            Self::Layer2 { l0_neighbors, .. } => l0_neighbors,
            Self::Layer3 { l0_neighbors, .. } => l0_neighbors,
        }
    }
}
