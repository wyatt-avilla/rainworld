use super::graph::Graph;
use super::interactivity_test::InteractivityTest;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = style! { "App",
      div {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        border: 1px solid pink;
        margin: 25px;
      }
    };

    view! { class = styler_class,
      <div>
        <Graph dim_px=400 />
        <InteractivityTest />
      </div>
    }
}
