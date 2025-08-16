use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Graph(dim_px: usize) -> impl IntoView {
    let styler_class = style! { "Graph",
      canvas {
        border: 1px solid black;
        background-color: lightblue;
      }
    };

    view! { class = styler_class,
      <canvas width=dim_px height=dim_px>
      </canvas>
    }
}
