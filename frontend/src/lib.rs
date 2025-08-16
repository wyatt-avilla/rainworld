use wasm_bindgen::prelude::*;

mod app;
mod graph;
mod interactivity_test;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}

#[wasm_bindgen(start)]
pub fn main() {}
