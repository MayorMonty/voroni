use point::Metric;
use point::Point;
use wasm_bindgen::prelude::*;
mod dulaney;
mod point;
mod demos;
use std::panic;
extern crate console_error_panic_hook;

#[macro_use]
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


