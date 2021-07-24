use point::Metric;
use point::Point;
use wasm_bindgen::prelude::*;
use web_sys::Touch;
mod bfs;
mod dulaney;
mod point;
extern crate console_error_panic_hook;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => {
        unsafe {
            console::log_1(&format_args!($($t)*).to_string().into())
        }
    }
}

#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// Demo 1: Render each point, highlighting the one that is closest to the mouse. This demo function
// will listen to the mousemove and touchmove events.
#[wasm_bindgen]
pub fn demo1(canvas: web_sys::HtmlCanvasElement, num_points: u32) -> Result<(), JsValue> {
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

    // Get the actual dimensions of the canvas, which is based on the size of the device, and will
    // change. The ratio will always be the same (16:9)
    let rect = canvas.get_bounding_client_rect();
    let canvas_width = rect.width() as i32;
    let canvas_height = rect.height() as i32;
    let canvas_left = rect.left();
    let canvas_top = rect.top();

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

    // Copy points so it can be moved into the closure
    let points = sites.clone();
    {
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            // Get the mouse position, in the rendering coordinates
            let (x, y) = (event.offset_x() as f64, event.offset_y() as f64);
            let (x, y) = (
                x / canvas_width as f64 * width as f64,
                y / canvas_height as f64 * height as f64,
            );

            let mouse = Point::new(x as i32, y as i32);

            context.clear_rect(0.0, 0.0, width.into(), height.into());

            // Render each point as a little circle, and draw guidance lines to each
            for point in &points {
                let (x, y): (f64, f64) = (point.x.into(), point.y.into());

                context.begin_path();
                context
                    .arc(x, y, 6.0, 0.0, std::f64::consts::PI * 2.0)
                    .unwrap();
                context.stroke();
            }

            let closest = points.iter().min_by(|a, b| {
                a.dist(&mouse, Metric::Euclidean)
                    .partial_cmp(&b.dist(&mouse, Metric::Euclidean))
                    .unwrap_or(Ordering::Equal)
            });

            if let Some(site) = closest {
                context.begin_path();
                context.move_to(site.x.into(), site.y.into());
                context.line_to(mouse.x.into(), mouse.y.into());
                context.stroke();
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            let touch = event.touches().get(0) as Option<Touch>;
            event.prevent_default();

            if let Some(touch) = touch {
                // Get the mouse position, in the rendering coordinates

                let (x, y) = (
                    touch.client_x() as f64 - canvas_left,
                    touch.client_y() as f64 - canvas_top,
                );
                let (x, y) = (
                    x / canvas_width as f64 * width as f64,
                    y / canvas_height as f64 * height as f64,
                );

                let mouse = Point::new(x as i32, y as i32);

                context.clear_rect(0.0, 0.0, width.into(), height.into());

                // Render each point as a little circle, and draw guidance lines to each
                for point in &sites {
                    let (x, y): (f64, f64) = (point.x.into(), point.y.into());

                    context.begin_path();
                    context
                        .arc(x, y, 6.0, 0.0, std::f64::consts::PI * 2.0)
                        .unwrap();
                    context.stroke();
                }

                let closest = sites.iter().min_by(|a, b| {
                    a.dist(&mouse, Metric::Euclidean)
                        .partial_cmp(&b.dist(&mouse, Metric::Euclidean))
                        .unwrap_or(Ordering::Equal)
                });

                if let Some(site) = closest {
                    context.begin_path();
                    context.move_to(site.x.into(), site.y.into());
                    context.line_to(mouse.x.into(), mouse.y.into());
                    context.stroke();
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}



#[wasm_bindgen]
pub fn demo2(
    canvas: web_sys::HtmlCanvasElement,
    num_points: u32,
    execution_speed: u32
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

    let mut bfs = bfs::BFS::new(sites.clone(), 1.0, width, height);

    
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        let window = web_sys::window().unwrap();
        window
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // Render each point as a little circle, and draw guidance lines to each
    for point in &sites {
        let (x, y): (f64, f64) = (point.x.into(), point.y.into());

        context.begin_path();
        context
            .arc(x, y, 6.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.stroke();
    }

    context.set_fill_style(&JsValue::from_str("#a29bfe"));

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        
        for _ in 0..execution_speed {
            let work_remaining = bfs.step();

            if work_remaining {
                for point in bfs.visited.last() {
                    
                    let (x, y): (f64, f64) = (point.x.into(), point.y.into());
                    context.fill_rect(x, y, 1.0, 1.0);
                }

                request_animation_frame(f.borrow().as_ref().unwrap());
            } else {
                console_log!("Complete!")
            }
        }


    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

