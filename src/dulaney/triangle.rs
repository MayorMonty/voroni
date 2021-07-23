///
/// Represents a single triangle in the triangulation. Each triangle is defined by three points. 
/// 
/// Author: Brendan McGuire
/// Date: 18 July 2021
/// 
/// https://bren.app/voronoi/
/// 

use std::fmt::Display;
use crate::{Point, point::Metric};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,

    pub circumcenter: Point,
    pub circumradius: f64,
    pub edges: [(Point, Point); 3]
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {

        let (circumcenter, circumradius) = Self::circumcircle(p1, p2, p3);

        Triangle {
            p1,
            p2,
            p3,
            circumcenter,
            circumradius,
            edges: [
                (p1, p2),
                (p2, p3),
                (p3, p1)
            ]
        }
    }

    /// Computes the circumcenter of the triangle, which is defined as the point at which the
    /// perpendicular bisectors of the sides of the triangle meet. Alteratively, the circumcenter is
    /// defined as the center of the circumcircle, which is constructed from the 3 vertices of the
    /// triangle. 
    /// 
    /// Note: this assumes that the three points are not collinear.
    pub fn circumcircle(p1: Point, p2: Point, p3: Point) -> (Point, f64) {
        
        // See http://paulbourke.net/geometry/circlesphere/
        
        // Convert to f64 to avoid rounding errors, we'll convert back to i32 at the end
        let (x_1, x_2, x_3): (f64, f64, f64) = (p1.x.into(), p2.x.into(), p3.x.into());
        let (y_1, y_2, y_3): (f64, f64, f64) = (p1.y.into(), p2.y.into(), p3.y.into());


        
        let ma = (y_2 - y_1) / (x_2 - x_1);
        let mb= (y_3 - y_2) / (x_3 - x_2);

        // If either ma or mb is zero, then the perpendicular bisectors are vertical (infinite
        // slope), which causes an error in computation. The easiest way to handle this is just
        // rearrange the original points and retry the computation.
        //
        // If ma == mb, then the triangle is degenerate, and the circumcenter is undefined. The best
        // way to handle this case is to panic until a Result chain can be established.
            
        if ma - mb == 0.0 {
            panic!(
                "Attempt to construct circumcircle of 3 collinear points!"
            );
        } else if ma == 0.0 {
            Self::circumcircle(p3, p2, p1)
        } else if mb == 0.0 {
            Self::circumcircle(p2, p1, p3)
        } else {
            let x = ((ma * mb * (y_1 - y_3) + mb * (x_2 + x_3) - ma * (x_2 + x_3))) / (2.0 * (mb - ma));
            let y = (-1.0 / ma) * (x - (x_1 + x_2) * 0.5) + (y_1 + y_2) * 0.5;
    
            let center = Point::new(x as i32, y as i32);
            let radius = center.dist(&p1, Metric::Euclidean);
    
            (center, radius)
        }       
       
    }

    /// Returns true if the circumcircle of the triangle contains the point p. The circumcircle is
    /// defined as the circle constructed by the 3 vertices of the triangle. See circumcenter above
    /// for more details.
    pub fn circumcircle_contains(&self, p: &Point) -> bool {

        // If the distance between p1 and the circumcenter is less than the distance between point
        // and the circumcenter, then the point is inside the circumcircle.
        self.circumradius > self.circumcenter.dist(p, Metric::Euclidean)
    }

    /// Returns true if the triangle has a vertex of p
    pub fn has_vertex(&self, p: Point) -> bool {
        self.p1 == p || self.p2 == p || self.p3 == p
    }

    /// Returns true if the triangle has an edge
    pub fn has_edge(&self, edge: &(Point, Point)) -> bool {
        self.edges.contains(edge)
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 &&
        self.p2 == other.p2 &&
        self.p3 == other.p3
    }
}

impl Eq for Triangle {}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle(p1: {}, p2: {}, p3: {}) - ({}, {})", self.p1, self.p2, self.p3, self.circumcenter, self.circumradius)
    }
}