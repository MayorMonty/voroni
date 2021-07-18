use std::ops::Range;
use rand::prelude::*;

///
/// Represents a point in 2D space.
/// 

pub struct Point { pub x: u32, pub y: u32 }
pub enum Metric {
    Euclidean,
    Manhattan,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub fn random(rng: &mut ThreadRng, x: Range<u32>, y: Range<u32>) -> Point {
        let x = rng.gen_range(x);
        let y = rng.gen_range(y);

        Self::new(x, y)
    }

    fn dist_euclidean(&self, other: &Point) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
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

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
