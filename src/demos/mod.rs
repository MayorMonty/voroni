use std::rc::Rc;
use crate::point::{Point, Metric};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::CanvasRenderingContext2d;

// Performs common demo setup operations: randomly generates points, and renders them to the canvas.
// Returns the relevant points.
pub fn demo_setup(canvas: &web_sys::HtmlCanvasElement, num_points: u32) -> (Rc<CanvasRenderingContext2d>, Vec<Point>) {
    let mut rng = rand::thread_rng();
    let context = canvas
        .get_context("2d")
        .unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().expect("Could not get canvas context");
    let context = Rc::new(context);

    // Get the rendering dimensions of the canvas (1600x900). This is static, to make rendering much
    // much easier.
    let width = canvas.width() as i32;
    let height = canvas.height() as i32;

    // Generate the initial points to base each region off of
    let sites: Vec<Point> = (0..num_points)
        .map(|_| Point::random(&mut rng, 0..width, 0..height))
        .collect();

    // Set stroke width and style
    context.set_stroke_style(&JsValue::from_str("#fab1a0"));
    context.set_fill_style(&JsValue::from_str("#fab1a0"));
    context.set_line_width(4.0);
    let dash = JsValue::from_serde(&[0.0]).unwrap();
    context.set_line_dash(&dash).expect("set_line_dash");

    // Render each point as a little circle, and draw guidance lines to each
    for point in &sites {
        let (x, y): (f64, f64) = (point.x.into(), point.y.into());

        context.begin_path();
        context
            .arc(x, y, 6.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.stroke();
    }

    (context, sites)
}

pub mod cursor_point;
pub mod naive;
pub mod perpendicular_bisector;
pub mod perpendicular_bisector_ranges;