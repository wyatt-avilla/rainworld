use gloo_console as console;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn PlantBox<'a>(
    name: String,
    scientific_name: &'a shared::plant::ScientificPlantName,
) -> impl IntoView {
    let styler_class = style! { "PlantBox",
      section {
        text-align: center;
        color: blue;
        border: 1px solid lightgreen;
      }
    };

    view! { class = styler_class,
      <section>
        <h1>{name}</h1>
        <h2>{scientific_name.to_string()}</h2>
        <WaterSender />
      </section>
    }
}

#[component]
pub fn WaterSender() -> impl IntoView {
    let styler_class = style! { "WaterSender",
      section {
        text-align: center;
        color: blue;
        border: 1px solid lightgreen;
      }
    };

    let (water_level, set_water_level) = signal(0);

    let step = 25;

    view! { class = styler_class,
      <section>
        <span>"water level is:" {water_level} "!"</span>
        <button on:click=move |_| set_water_level.update(|level| *level -= step)> "-" </button>
        <button on:click=move |_| set_water_level.update(|level| *level += step)> "+" </button>
        <button on:click=move |_| console::log!("water is ", water_level.get())> "X" </button>
      </section>
    }
}
