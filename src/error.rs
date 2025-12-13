#[derive(Debug)]
pub enum BuildError {
    NotEnoughDataPoints {
        expected: usize,
        cause: &'static str,
        found: usize,
    },
}

impl BuildError {
    pub fn not_enough_data_points(expected: usize, cause: &'static str, found: usize) -> Self {
        Self::NotEnoughDataPoints { expected, cause, found }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughDataPoints { expected, cause, found } => {
                write!(f, "Expected at least {expected} data points to build {cause}, found {found}")
            }
        }
    }
}

impl std::error::Error for BuildError {}
