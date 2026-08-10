use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use leptos::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {

    let items = (0..15).collect::<Vec<usize>>();

    view! {
        <main class="container">
            <h1>"Djxs4nHypr"</h1>

            <nav class="filter-bar">
                <button class="filter-btn active">"ALL"</button>
                <button class="filter-btn">"PIC"</button>
                <button class="filter-btn">"VID"</button>
            </nav>

            <section class="horizontal-scroll-container">
                <div class="honeycomb-track">
                    < For 
                        each=move || items.clone()
                        key=|item| * item
                        children= move |_| {
                            view! {
                                <div class="hexago-wrapper">
                                    <div class="hexagon-inner">
                                        <div class="hexagon-content">
                                            <span class="placeholder-icon">PH</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </section>
        </main>
    }
}
