///
/// Demo 2: Naive
/// 
/// A naive approach to finding voronoi regions. It simply iterates through every pixel on the
/// screen, and finds the closest site (as defined by euclidean squared distance) and marks each
/// pixel with its closest site
/// 

use std::{rc::Rc};
use crate::point::{Point, Metric};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};


const COLORS: &[&str] = &[
    "#55efc4",
    "#81ecec",
    "#74b9ff",
    "#dfe6e9",
    "#ffeaa7",
    "#fab1a0",
    "#ff7675",
    "#fd79a8",
    "#636e72"
];

#[wasm_bindgen]
pub fn demo2(
    canvas: web_sys::HtmlCanvasElement,
    num_points: u32,
) -> Result<(), JsValue> {
    let mut rng = rand::thread_rng();
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
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

    // For each pixel, find the closest site, and color it appropriately
    for x in 0..width {
        for y in 0..height {
            let point = Point::new(x, y);

            let mut closest_distance = f64::INFINITY;
            let mut closest = None;
            for (i, site) in sites.iter().enumerate() {
                let dist = site.dist(&point, Metric::EuclideanSquared);

                if dist < closest_distance {
                    closest_distance = dist;
                    closest = Some(i);
                }

            }

            if let Some(i) = closest {
                context.set_fill_style(&JsValue::from_str(COLORS[i]));
                context.fill_rect(x.into(), y.into(), 1.0, 1.0);
            }

        }
    }

    context.set_stroke_style(&JsValue::from_str("#333333"));
    // Render each point as a little circle, and draw guidance lines to each
    for (i, point) in sites.iter().enumerate() {
        let (x, y): (f64, f64) = (point.x.into(), point.y.into());


        context.begin_path();
        context
            .arc(x, y, 2.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.stroke();
    }   
    


    Ok(())
}

