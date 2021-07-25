///
/// Demo 1: Cursor Point
/// 
/// Generates a number of voronoi sites, and as the mouse (or finger) moves around the canvas, draws
/// a line from the cursor to the voronoi site nearest to the cursor.
/// 
/// Author: Brendan McGuire
/// Date: 18 July 2021
/// 
/// https://bren.app/voronoi/
/// 


use std::{cmp::Ordering};
use crate::point::{Point, Metric};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::Touch;
use super::demo_setup;

#[wasm_bindgen]
pub fn demo1(canvas: web_sys::HtmlCanvasElement, num_points: u32) -> Result<(), JsValue> {
    
    // Basic setup
    let (context, sites) = demo_setup(&canvas, num_points);

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