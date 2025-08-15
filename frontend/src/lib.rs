use gloo_console as console;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    let panic_handler = move |_| {
        panic!("Test panic from Leptos button click!");
    };

    let log_handler = move |_| console::log!("Hello console");

    view! {
        <div>
          <h1>"Hello Leptos!"</h1>
          <button on:click=panic_handler>
            "Click to Panic!"
          </button>
          <button on:click=log_handler>
            "Click to log!"
          </button>
        </div>
    }
}

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[wasm_bindgen(start)]
pub fn main() {}
