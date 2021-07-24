//
// This method implements a Breadth-First Search approach to finding the regions, which should hopefully be able to find edges a bit
// quicker than a DFS.
//
// Author: Brendan McGuire
// Date: 23 July 2021
//
// https://bren.app/voronoi/
//

use crate::point::{Metric, Point};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

/// Determines if the given point is on the voronoi edge, which means the closest two sites to this
/// point have the same distance from this point.
///
///
/// Returns:
///  Some((site, site)) if the point is an edge and
///  None if the point is not an edge
pub fn is_edge(point: Point, sites: Vec<Point>, epsilon: f64) -> Option<(Point, Point)> {
    // First find the closest site
    let closest = sites.iter().min_by(|a, b| {
        a.dist(&point, Metric::Euclidean)
            .partial_cmp(&b.dist(&point, Metric::Euclidean))
            .unwrap_or(Ordering::Equal)
    });

    if let Some(closest) = closest {
        let closest_dist = closest.dist(&point, Metric::Euclidean);
        for site in sites.iter() {
            if site != closest {
                let distance = site.dist(&point, Metric::Euclidean);

                // The traditional definition requires that the distance is the same, but there are
                // some complications with this:
                //
                //  1. Floating-point numbers mean equality isn't meaningful here, as they are not
                //      able to perfectly represent the decimal anyways
                //  2. Because we're working with discrete pixel values and not continuous points, it
                //    is likely that the actual voronoi edge will be between pixels
                //
                // To best deal with both of these, there will be an acceptable difference epsilon,
                // which is configurable.
                if (distance - closest_dist).abs() < epsilon {
                    return Some((*closest, *site));
                }
            }
        }
        None
    } else {
        None
    }
}

pub struct BFS {
    pub sites: Vec<Point>,
    epsilon: f64,
    pub visited: Vec<Point>,
    pub edges: HashMap<(Point, Point), Vec<Point>>,
    pub frontier: VecDeque<Point>,

    width: i32,
    height: i32,
}

impl BFS {
    pub fn new(sites: Vec<Point>, epsilon: f64, width: i32, height: i32) -> Self {

        let mut frontier = VecDeque::new();

        for site in sites.iter() { 
            frontier.push_front(*site);
        }

        BFS {
            sites,
            epsilon,
            visited: vec![],
            edges: HashMap::new(),
            frontier,

            width,
            height,
        }
    }

    pub fn edges(&self) -> HashMap<(Point, Point), Vec<Point>> {
        self.edges.clone()
    }

    pub fn get_unvisited_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = vec![];

        if point.x > 0 {
            let point = Point {
                x: point.x - 1,
                y: point.y,
            };

            if !self.visited.contains(&point) {
                neighbors.push(point);
            }
        }

        if point.y > 0 {
            let point = Point {
                x: point.x,
                y: point.y - 1,
            };

            if !self.visited.contains(&point) {
                neighbors.push(point);
            }
        }

        if point.x < self.width - 1 {
            let point = Point {
                x: point.x + 1,
                y: point.y,
            };

            if !self.visited.contains(&point) {
                neighbors.push(point);
            }
        }

        if point.y < self.height - 1 {
            let point = Point {
                x: point.x,
                y: point.y + 1,
            };

            if !self.visited.contains(&point) {
                neighbors.push(point);
            }
        }

        neighbors
    }

    /// Steps through a single iteration of the BFS algorithm. Returns true if the there is still
    /// work to do, false if the algorithm is done.
    pub fn step(&mut self) -> bool {
        if self.frontier.is_empty() { 
            false
        } else {
            let point = self.frontier.pop_back();
            
            if let Some(point) = point {
                self.visited.push(point);

                if let Some(sites) = is_edge(point, self.sites.clone(), self.epsilon) {
                    if self.edges.contains_key(&sites) {
                        self.edges.get_mut(&sites).unwrap().push(point);
                    } else {
                        self.edges.insert(sites, vec![point]);
                    }
                };

                let neighbor_points = self.get_unvisited_neighbors(point);
                for neighbor_point in neighbor_points.iter() {
                    self.frontier.push_back(*neighbor_point);
                }
            }

            true
        }
    }

}
