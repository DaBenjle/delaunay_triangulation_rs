use crate::geometry::{Point, Triangle};

pub trait Triangulation {
    fn triangulate(&self, points: Vec<Point>) -> Vec<Triangle>;
}

pub use crate::algorithms::bowyer_watson::BowyerWatson;
pub use crate::algorithms::divide_and_conquer::DivideAndConquer;
pub use crate::algorithms::incremental::Incremental;
