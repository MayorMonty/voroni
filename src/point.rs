///
/// Represents a point in 2D space. Coordinates are stored as integers, so some computations will
/// need to round to the nearest integer.
/// 
/// Author: Brendan McGuire
/// Date: 18 July 2021
/// 
/// https://bren.app/voronoi/
/// 


use std::{fmt::Display, ops::Range};
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point { pub x: i32, pub y: i32 }
pub enum Metric {
    Euclidean,
    Manhattan,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(rng: &mut ThreadRng, x: Range<i32>, y: Range<i32>) -> Point {
        let x = rng.gen_range(x);
        let y = rng.gen_range(y);

        Self::new(x, y)
    }

    fn dist_euclidean(&self, other: &Point) -> f64 {


        let x_diff: f64 = (self.x - other.x).into();
        let y_diff: f64 = (self.y - other.y).into();

        let diff_sq: f64 = (x_diff * x_diff + y_diff * y_diff).into();
        let sqrt_diff = diff_sq.sqrt();

        sqrt_diff
    }

    fn dist_manhattan(&self, other: &Point) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let diff_abs = x_diff + y_diff;
        diff_abs.into()
    }

    pub fn dist(&self, other: &Point, metric: Metric) -> f64 {
        match metric {
            Metric::Euclidean => self.dist_euclidean(other),
            Metric::Manhattan => self.dist_manhattan(other),
        }
    }


}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
    