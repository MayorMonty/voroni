use rand::prelude::*;
use std::f64;
use std::u32;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Generates a random point on the board
pub fn random_point(width: u32, height: u32, rng: &mut ThreadRng) -> (u32, u32) {
    let x = rng.gen_range(0..width);
    let y = rng.gen_range(0..height);

    (x, y)
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
    let points: Vec<(u32, u32)> = (0..num_points)
        .map(|_| random_point(canvas.width(), canvas.height(), &mut rng))
        .collect();

    // Set stroke width and style
    context.set_stroke_style(&JsValue::from_str("#fab1a0"));
    context.set_fill_style(&JsValue::from_str("#fab1a0"));
    context.set_line_width(4.0);

    // Render each point as a little circle, and draw guidance lines to each
    for (x, y) in &points {
        let (x, y): (f64, f64) = ((*x).into(), (*y).into());

        context.begin_path();
        context.arc(x, y, 6.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.stroke();

        // Compute the perpendicular bisector for every other point
        context.set_stroke_style(&JsValue::from_str("rgba(250, 177, 160, 0.1)"));
        context.set_line_width(3.0);
        for (x_1, y_1) in &points {
            let (x_1, y_1): (f64, f64) = ((*x_1).into(), (*y_1).into());

            // Compute the midpoint
            let (x_m, y_m) = ((x + x_1) / 2.0, (y + y_1) / 2.0);

            // Compute the slope and find the negative reciprocal
            let slope = (y_1 - y) / (x_1 - x);
            let slope_inv = -1.0 / slope;

            // The function of the perpendicular bisector is now known
            //  y - y_m = slope_inv(x - x_m)
            //  y = slope_inv(x - x_m) + y_m

            let (x_a, y_a) = (0.0, slope_inv * (0.0 - x_m) + y_m);
            let (x_b, y_b) = (1599.0, slope_inv * (1599.0 - x_m) + y_m);

            context.begin_path();
            context.move_to(x_a, y_a);
            context.line_to(x_b, y_b);
            context.stroke();
        }
        context.set_stroke_style(&JsValue::from_str("#fab1a0"));
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

}
