mod algorithms;
pub mod geometry;

fn delaunay_triangulation(
    points: Vec<geometry::Point>,
    algorithm: impl algorithms::delaunay_triangulation::Triangulation,
) -> Vec<geometry::Triangle> {
    algorithm.triangulate(points)
}

pub fn incremental_delaunay_triangulation(points: Vec<geometry::Point>) -> Vec<geometry::Triangle> {
    delaunay_triangulation(points, algorithms::delaunay_triangulation::Incremental)
}

pub fn divide_and_conquer_delaunay_triangulation(
    points: Vec<geometry::Point>,
) -> Vec<geometry::Triangle> {
    delaunay_triangulation(points, algorithms::delaunay_triangulation::DivideAndConquer)
}

pub fn bowyer_watson_delaunay_triangulation(
    points: Vec<geometry::Point>,
) -> Vec<geometry::Triangle> {
    delaunay_triangulation(points, algorithms::delaunay_triangulation::BowyerWatson)
}

pub fn default_delaunay_triangulation(points: Vec<geometry::Point>) -> Vec<geometry::Triangle> {
    // Default to the quickest algorithm: Divide and Conquer
    divide_and_conquer_delaunay_triangulation(points)
}
