use point::Point;
use std::f64;
use std::u32;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod point;


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
        .map(|_| Point::random(&mut rng, 0..canvas.width(), 0..canvas.height()))
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

