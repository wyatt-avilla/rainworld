use super::graph::Graph;
use super::plantbox::PlantBox;
use leptos::prelude::*;
use shared::plant::ScientificPlantName;
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
        <PlantBox name="Plant Name".to_string() scientific_name=&ScientificPlantName::FicusElastica  />
        <PlantBox name="Other Plant Name".to_string() scientific_name=&ScientificPlantName::MonsteraDeliciosa />
        <PlantBox name="Another Plant".to_string() scientific_name=&ScientificPlantName::DieffenbachiaReflector />
        <PlantBox name="Final Plant".to_string() scientific_name=&ScientificPlantName::MonsteraDeliciosa />
      </div>
    }
}
