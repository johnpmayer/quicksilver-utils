pub mod app;

#[cfg(feature = "web-sys")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "web-sys")]
use quicksilver::lifecycle::{run, Settings};

#[cfg(feature = "web-sys")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "web-sys")]
#[wasm_bindgen(start)]
pub fn main_js() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    run(Settings::default(), app::app)
}
