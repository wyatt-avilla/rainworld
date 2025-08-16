use gloo_console as console;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn InteractivityTest() -> impl IntoView {
    let styler_class = style! { "InteractivityTest",
      section {
        text-align: center;
        color: blue;
        border: 1px solid lightgreen;
      }
    };

    let panic_handler = move |_| {
        panic!("Test panic from Leptos button click!");
    };

    let log_handler = move |_| console::log!("Hello console");

    view! { class = styler_class,
      <section>
        <h1>"Hello Leptos!"</h1>
        <button on:click=panic_handler>
          "Click to Panic!"
        </button>
        <button on:click=log_handler>
          "Click to log!"
        </button>
      </section>
    }
}
