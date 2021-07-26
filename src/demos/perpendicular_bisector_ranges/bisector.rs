use crate::point::Point;
/// Defines a bisector type, which is a line segment defined as the portion of a perpendicular
/// bisector of 2 points (voronoi sites)
use std::{fmt::Display, num::FpCategory};

#[derive(Clone, Copy)]
pub struct Bisector {
    // The midpoint between the two initial points, not the midpoint of the bisector itself
    pub point: Point,

    segment_slope: f64, // Note: this is the slope of the line segment connecting the two points, not the slope of the bisector
    pub slope: f64,     // Inverse of the slope, the slope of the bisector

    // Defines the range of the segment
    pub x_min: f64,
    pub x_max: f64,

    // For vertical lines, we need to store separate bounds for the y-axis. For all other values,
    // these are driven by the x_values.
    pub y_min: f64,
    pub y_max: f64,

    // The original two points
    pub a: Point,
    pub b: Point,
}

impl Bisector {
    /// Constructs a new perpendicular bisector of 2 points, including calculate the minimum and
    /// maximum x values where the line will intersect with the edge of the canvas.
    pub fn new(a: Point, b: Point) -> Self {
        // Right now we are having initial x_min and x_max values be -inf to inf, but this could
        // change, so keep these variables for now.
        // TODO: Refactor this, assuming that x_min and x_max is always -inf to inf

        let min_width = f64::NEG_INFINITY;
        let min_height = f64::NEG_INFINITY;

        let max_width = f64::INFINITY;
        let max_height = f64::INFINITY;

        let (x_m, y_m) = ((a.x + b.x) as f64 / 2.0, (a.y + b.y) as f64 / 2.0);
        let midpoint = Point::new(x_m as i32, y_m as i32);

        // Special case: if the points have the same x value, then the bisector will be a horizontal
        // line with slope zero, and the y bounds will be the midpoint y value.
        if a.x == b.x {
            let segment_slope = f64::INFINITY;
            let slope = 0.0;

            let x_min = min_width;
            let x_max = max_width as f64;

            let y_min = y_m;
            let y_max = y_m;

            Self {
                point: midpoint,
                segment_slope,
                slope,
                x_min,
                x_max,
                a,
                b,
                y_min,
                y_max,
            }
        } else {
            // Calculate the slope of the line segment connecting the two points
            let segment_slope = (b.y - a.y) as f64 / (b.x - a.x) as f64;

            // Handle vertical lines, which have infinite slope
            if segment_slope.classify() == FpCategory::Zero {
                let x_min = x_m;
                let x_max = x_m;

                let y_min = f64::NEG_INFINITY;
                let y_max = f64::INFINITY;

                let slope = f64::INFINITY;

                Self {
                    point: midpoint,
                    segment_slope,
                    slope,
                    x_min,
                    x_max,
                    a,
                    b,
                    y_min,
                    y_max,
                }
            } else {
                let slope = -segment_slope.recip();

                // Calculate the range of the segment, initially based on the view size, the max values or
                // width and height. This will be clamped down further in the future

                // Compute the two endpoints of the bisector. Because of negative values, we need to compute
                // these, and then order them separately.
                let x_a = x_m + segment_slope * (y_m - min_height);
                let x_b = x_m + segment_slope * (y_m - max_height as f64);

                // Sort the endpoints into min and max
                let x_min = f64::max(min_width, f64::min(x_a, x_b));
                let x_max = f64::min(max_width.into(), f64::max(x_a, x_b));

                let y_min = f64::NEG_INFINITY;
                let y_max = f64::INFINITY;

                Self {
                    point: midpoint,
                    segment_slope,
                    slope,
                    x_min,
                    x_max,
                    a,
                    b,
                    y_min,
                    y_max,
                }
            }
        }
    }

    /// Returns true if the perpendicular bisector is a vertical line
    pub fn is_vertical(&self) -> bool {
        self.slope.is_infinite()
    }

    /// Returns true if the perpendicular bisector is a horizontal line
    pub fn is_horizontal(&self) -> bool {
        self.segment_slope.is_infinite()
    }

    /// Returns true if the perpendicular bisector contains the given x value
    pub fn contains_x(&self, x: f64) -> bool {
        self.x_min <= x && x <= self.x_max
    }

    /// Computes the appropriate y value for the given x value, based on the line segment. Does not
    /// consider the x_min and x_max values. Returns None for vertical lines.
    pub fn compute(&self, x: f64) -> Option<f64> {
        if self.is_vertical() {
            None
        } else if self.is_horizontal() {
            Some(self.point.y as f64)
        } else {
            Some(self.slope * (x - self.point.x as f64) + self.point.y as f64)
        }
    }

    /// Computes the appropriate y value for the given x value, based on the line segment. Does not
    /// consider y_min and y_max values. Returns None for horizontal lines.
    pub fn compute_inv(&self, y: f64) -> Option<f64> {
        if self.is_horizontal() {
            None
        } else if self.is_vertical() {
            Some(self.point.x as f64)
        } else {
            Some(self.point.x as f64 + (y - self.point.y as f64) / self.slope)
        }
    }

    /// Returns the point at which this bisector intersects with the other bisector, if it exists.
    /// Considers x_min and x_max values (which should not be equal). Returns None if the lines do not intersect, if they
    /// are coincident, or the the clamped x ranges does not include their intersection point. Note:
    /// this function could return a point if the lines bisect at a point which is in the correct x
    /// range, but is above or below the y-range as specified. This is because these intersections
    /// still "count", even if we are not showing them. Additionally, this function will return
    /// intersections with x values less than 0 or greater than the width of the canvas, for the
    /// same reason.
    ///
    /// The intention of this function is to aid in finding a number of polygons that define each
    /// voronoi region. Some lines will continue out to infinity, and some will
    /// intersect offscreen. These are fundamentally different. This is an important distinction for
    /// computing the polygons.
    ///
    /// This means the x value of the point returned will always be between x_min and x_max.
    pub fn intersection(&self, other: &Self) -> Option<Point> {
        // If either bisector is vertical, check the other's range to see if it intersects
        match (self.is_vertical(), other.is_vertical()) {
            // If both are vertical, then they are either parallel or coincident. Neither of these
            // count as intersections.
            (true, true) => None,

            // Cases where just one is vertical
            (true, false) => {
                if (other.x_min..other.x_max).contains(&(self.point.x as f64)) {
                    other
                        .compute(self.point.x as f64)
                        .map(|y| Point::new(self.point.x as i32, y as i32))
                } else {
                    None
                }
            }
            (false, true) => {
                if (self.x_min..self.x_max).contains(&(other.point.x as f64)) {
                    self.compute(other.point.x as f64)
                        .map(|y| Point::new(other.point.x as i32, y as i32))
                } else {
                    None
                }
            }

            // When neither are vertical, compute if the line segments intersect
            (false, false) => {
                // First, throw out lines which do not contain common x-values
                if self.x_max < other.x_min {
                    return None;
                }

                // Next, throw out parallel lines (which would cause a divide by zero error below).
                if (self.slope - other.slope).classify() == FpCategory::Zero {
                    return None;
                }

                // Compute the y-intercepts
                let b1 = self.compute(0.0).unwrap();
                let b2 = other.compute(0.0).unwrap();

                // Finally, compute the intersection point
                let x = (b2 - b1) / (self.slope - other.slope);
                let y = (self.slope * b2 - other.slope * b1) / (self.slope - other.slope);

                // Only return the point if it is in both x ranges
                if (self.x_min..self.x_max).contains(&x) && (other.x_min..other.x_max).contains(&x) {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            }
        }
    }

    /// Sets the minimum x value, and updates the minimum y value accordingly. No op for vertical lines.
    pub fn set_min_x(&mut self, x_min: f64) {
        if !self.is_vertical() {
            self.x_min = x_min;
            self.y_min = self.compute(x_min).unwrap();
        }
    }

    /// Sets the maximum x value, and updates the maximum y value accordingly. No op for vertical lines.
    pub fn set_max_x(&mut self, x_max: f64) {
        if !self.is_vertical() {
            self.x_min = x_max;
            self.y_min = self.compute(x_max).unwrap();
        }
    }

    /// Sets the minimum y value, and updates the minimum x value accordingly. No op for horizontal lines.
    pub fn set_min_y(&mut self, y_min: f64) {
        if !self.is_horizontal() {
            self.y_min = y_min;
            self.x_min = self.compute_inv(y_min).unwrap();
        }
    }

    /// Sets the maximum y value, and updates the maximum x value accordingly. No op for horizontal lines.
    pub fn set_max_y(&mut self, y_max: f64) {
        if !self.is_horizontal() {
            self.y_max = y_max;
            self.x_max = self.compute_inv(y_max).unwrap();
        }
    }

    /// Bifurcates the line segment at the x value (x_min < x < x_max). Returns None if the line
    /// segment is vertical, otherwise returns the new segments in (left, right) order.
    pub fn split_at_x(&self, x: f64) -> Option<(Self, Self)> {

        if self.is_vertical() {
            None
        } else {
            // Make 2 copies
            let (mut left, mut right) = (self.clone(), self.clone());

            // Update the boundaries
            left.set_max_x(x);
            right.set_min_x(x);

            Some((left, right))
        }
    }

    /// Bifurcates the line segment at the y value (y_min < y < y_max). Returns None if the line
    /// segment is horizontal, otherwise returns the new segments in (bottom, top) order.
    pub fn split_at_y(&self, y: f64) -> Option<(Self, Self)> {

        if self.is_vertical() {
            None
        } else {
            // Make 2 copies
            let (mut bottom, mut top) = (self.clone(), self.clone());

            // Update the boundaries
            bottom.set_max_y(y);
            top.set_min_y(y);

            Some((bottom, top))
        }
    }

}

impl Display for Bisector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_vertical() {
            write!(f, "{}->{} ({} < y < {})", self.a, self.b, self.y_min, self.y_max)
        } else {
            write!(f, "{}->{} ({} < x < {})", self.a, self.b, self.x_min, self.x_max)
        }
    }
}