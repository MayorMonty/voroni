use point::Point;
use std::f64;
use std::u32;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod point;
mod dulaney;
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
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
        context.arc(x, y, 6.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.stroke();
    }


    let triangulation = dulaney::dulaney_triangulation(&sites, width, height);

    
    
        // Render the triangulation
        for triangle in triangulation {

            context.set_stroke_style(&JsValue::from_str("#fab1a0"));
            context.set_fill_style(&JsValue::from_str("#fab1a0"));
            let dash = JsValue::from_serde(&[5.0, 10.0]).unwrap();
            context.set_line_dash(&dash).expect("set_line_dash");
            context.set_line_width(2.0);

            for edge in triangle.edges.iter() {
                let start = edge.0;
                let end = edge.1;

                context.begin_path();
                context.move_to(start.x.into(), start.y.into());
                context.line_to(end.x.into(), end.y.into());   
                context.stroke();

            }

            // Render the circumcenter
            let center = triangle.circumcenter;

            context.set_stroke_style(&JsValue::from_str("#a29bfe"));
            context.set_fill_style(&JsValue::from_str("#a29bfe"));
            context.set_line_width(4.0);
            let dash = JsValue::from_serde(&[0.0]).unwrap();
            context.set_line_dash(&dash).expect("set_line_dash");

            context.begin_path();
            context.arc(center.x.into(), center.y.into(), 4.0, 0.0, f64::consts::PI * 2.0).unwrap();
            context.stroke();
        }

        



}

