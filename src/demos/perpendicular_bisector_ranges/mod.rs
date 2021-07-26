
///
/// Demo 4: Perpendicular Bisector with x ranges
/// 

use wasm_bindgen::prelude::*;
use crate::point::Point;

use self::bisector::Bisector;
use super::demo_setup;
use crate::console_log;

mod bisector;


#[wasm_bindgen]
pub fn demo4(
    canvas: web_sys::HtmlCanvasElement,
    num_points: u32,
) -> Result<(), JsValue> {
    
    let (context, mut sites) = demo_setup(&canvas, num_points);




    // Get the min and max x value for the canvas, which we use for drawing
    let (x_min, x_max) = (0.0, canvas.width() as f64 - 1.0);
    let (y_min, y_max) = (0.0, canvas.height() as f64 - 1.0);

    context.set_stroke_style(&JsValue::from_str("rgba(250, 177, 160, 0.1)"));
    context.set_line_width(3.0);

    let n = sites.len();
    let mut bisectors: Vec<Bisector> = Vec::with_capacity(n * (n-1) / 2);

    // Compute the bisector for each pair of sites
    for i in 0..n {
        for j in (i+1)..n {
            let a = sites[i];
            let b = sites[j];

            let bisector = Bisector::new(a, b);

            let (mut x_a, mut y_a) = (0.0, 0.0);
            let (mut x_b, mut y_b) = (0.0, 0.0);

            if bisector.is_vertical() {
                y_a = y_min;
                y_b = y_max;

                x_a = bisector.compute_inv(y_a).unwrap();
                x_b = bisector.compute_inv(y_b).unwrap();
            } else {
                x_a = x_min;
                x_b = x_max;

                y_a = bisector.compute(x_a).unwrap();
                y_b = bisector.compute(x_b).unwrap();
                
            }

            context.begin_path();
            context.move_to(x_a, y_a);
            context.line_to(x_b, y_b);
            context.stroke();
            
            bisectors.push(bisector);

        }
    };

    let n = bisectors.len();

    console_log!("{} bisectors total", n);

    for i in 0..n {
        for j in (i+1)..n {
            let a = bisectors[i];
            let b = bisectors[j];

            if let Some(point) = a.intersection(&b) {
                console_log!("{} intersects with {} at {}", a, b, point);

                context.begin_path();
                context
                    .arc(point.x.into(), point.y.into(), 3.0, 0.0, std::f64::consts::PI * 2.0)
                    .unwrap();
                context.stroke();

            }

        }
    };


    Ok(())
}

