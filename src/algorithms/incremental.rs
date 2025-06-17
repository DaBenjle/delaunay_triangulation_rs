use crate::algorithms::delaunay_triangulation::Triangulation;
use crate::geometry::{Point, Triangle};

pub struct Incremental;

impl Triangulation for Incremental {
    fn triangulate(&self, _points: Vec<Point>) -> Vec<Triangle> {
        // Implement Incremental Delaunay triangulation
        // This is a placeholder implementation

        // The actual implementation would go here, involving:
        // 1. Inserting points one by one into the triangulation.
        // 2. Maintaining Delaunay properties by checking and flipping edges as necessary.

        Vec::new()
    }
}
