///
/// Demo 3: Perpendicular Bisectors
/// 
/// For each pair of randomly generated points, computes and draws the perpendicular bisector. This
/// is an important first step in finding the Dulaney triangulation.
/// 
/// 

use wasm_bindgen::prelude::*;
use super::demo_setup;


#[wasm_bindgen]
pub fn demo3(
    canvas: web_sys::HtmlCanvasElement,
    num_points: u32,
) -> Result<(), JsValue> {
    
    let (context, sites) = demo_setup(&canvas, num_points);

    // Get the min and max x value for the canvas, which we use for drawing
    let (x_min, x_max) = (0.0, canvas.width() as f64 - 1.0);

    context.set_stroke_style(&JsValue::from_str("rgba(250, 177, 160, 0.1)"));
    context.set_line_width(3.0);

    // Compute the bisectors for each pair of points.
    for a in sites.iter() {
        for b in sites.iter() {

            // Midpoint
            let (x_m, y_m) = ((a.x + b.x) as f64 / 2.0, (a.y + b.y) as f64 / 2.0);

            // Slope & Slope Inverse
            let m = (b.y - a.y) as f64 / (b.x - a.x) as f64;
            let m_inv = -1.0 / m;

            // The function of the perpendicular bisector is now known:
            // y = m_inv * (x - x_m) + y_m
            let (x_a, y_a) = (x_min, m_inv * (x_min - x_m) + y_m);
            let (x_b, y_b) = (x_max, m_inv * (x_max - x_m) + y_m);

            // Draw the bisector
            context.begin_path();
            context.move_to(x_a, y_a);
            context.line_to(x_b, y_b);
            context.stroke();



        }
    };



    Ok(())
}

