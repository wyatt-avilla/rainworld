use gloo_console as console;
use leptos::prelude::*;
use stylers::style;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    let styler_class = style! { "App",
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }

        div.one {
            color: red;
        }
    };

    let panic_handler = move |_| {
        panic!("Test panic from Leptos button click!");
    };

    let log_handler = move |_| console::log!("Hello console");

    view! { class = styler_class,
        <div class="one">
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
