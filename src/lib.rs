use rand::prelude::*;
use std::convert::TryInto;
use std::f64;
use std::u32;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use cached::proc_macro::cached;

#[derive(Clone, Copy)]
pub struct Point { x: u32, y: u32 }

impl Point {
    pub fn nearest_site(&self, sites: &Vec<Point>, metric: Metric) -> Point {

        let mut site = sites[0];
        let mut min_distance = 0;

        for candidate in sites {
            let distance = metric.compute(candidate, self);

            if distance < min_distance {
                site = *candidate;
                min_distance = distance;
            }
        };

        site
    }
}


pub enum Metric {
    Euclidean,
    Manhattan,
}

impl Metric {
    pub fn compute(&self, a: &Point, b: &Point) -> u32 {
        
        let (a_x, a_y): (i64, i64) = (a.x.into(), a.y.into());
        let (b_x, b_y): (i64, i64) = (b.x.into(), b.y.into());

        let x = a_x - b_x;
        let y = a_y - b_y;

        let result = match self {
            &Metric::Euclidean => x * x + y * y,
            &Metric::Manhattan => x.abs() + y.abs(),
        };

        result.try_into().unwrap()
    }
}



/// Generates a random point on the board
pub fn random_point(width: u32, height: u32, rng: &mut ThreadRng) -> Point {
    let x = rng.gen_range(0..width);
    let y = rng.gen_range(0..height);

    Point { x, y }
}

#[wasm_bindgen]
pub fn generate(context: web_sys::CanvasRenderingContext2d, num_points: u32) {
    let mut rng = rand::thread_rng();
    let canvas = context
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    // Generate the initial points to base each region off of
    let sites: Vec<Point> = (0..num_points)
        .map(|_| random_point(canvas.width(), canvas.height(), &mut rng))
        .collect();

    // Set stroke width and style
    context.set_stroke_style(&JsValue::from_str("#fab1a0"));
    context.set_fill_style(&JsValue::from_str("#fab1a0"));
    context.set_line_width(4.0);

    // Render each point as a little circle, and draw guidance lines to each
    for point in &sites {
        let (x, y): (f64, f64) = (point.x.into(), point.y.into());

        context.begin_path();
        context.arc(x, y, 6.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.stroke();
    }

}

