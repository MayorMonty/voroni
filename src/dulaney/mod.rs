///
/// An implementation of the Dulaney Triangulation using the Bowyer–Watson algorithm. The Dulaney
/// Triangulation is a way to represent a number of points (the graph) as a series of connected triangles. 
/// Conveniently, this triangulation is a dual graph of the Voronoi diagram for the same set of
/// points. 
/// 
/// Author: Brendan McGuire
/// Date: 18 July 2021
/// 
/// https://bren.app/voronoi/
/// 

mod triangle;

use triangle::Triangle;
use super::Point;
use web_sys::console;

pub fn dulaney_triangulation(points: &[Point], width: i32, height: i32) -> Vec<Triangle> {

    let mut triangulation = Vec::new();

    // First, create the super triangle, which contains all of the points. The easiest way to do
    // this is to have the super triangle contain (circumscribe) the entire canvas.
    // See https://www.desmos.com/calculator/ckld3teujk for how the dimensions of the super triangle
    // can be calculated.

    let super_triangle = Triangle::new(
        Point::new(width / 2, 2 * height),
        Point::new(-width, 0),
        Point::new(2 * width, 0)
    );
    triangulation.push(super_triangle);


    for point in points {

        // Find the bad triangles, whose circumcircle contain this point
        let mut bad_triangles = Vec::new();
        for triangle in triangulation.iter() {


            if triangle.circumcircle_contains(point) {
                bad_triangles.push(triangle.clone());
            }
        }

        // Find the boundary of the polygonal hole created by the bad triangles.
        let mut polygon = Vec::new();
        for triangle in bad_triangles.clone() {
            for edge in triangle.edges.iter() {
                let mut shared = false;
                for other_triangle in bad_triangles.clone() {
                    if triangle != other_triangle && other_triangle.has_edge(edge) {
                            shared = true;
                    }
                }

                if !shared {
                    polygon.push(edge.clone());
                }
            }
        };

        // Remove bad triangles from the triangulation.
        for triangle in bad_triangles.iter() {
            if let Some(triangle) = triangulation.iter().position(|t| t == triangle)  {
                triangulation.remove(triangle);
            }
        };



        // Re-triangulate the boundary of the polygonal hole.
        for edge in polygon {

            let triangle = Triangle::new(edge.0, edge.1, point.clone());
            triangulation.push(triangle);
        };
    }

    // Cleanup: Remove any triangles that contain a vertex from the original super-triangle.
    triangulation.into_iter().filter(|triangle| {
        !triangle.has_vertex(super_triangle.p1) && 
        !triangle.has_vertex(super_triangle.p2) && 
        !triangle.has_vertex(super_triangle.p3) 
    }).collect()
    
}