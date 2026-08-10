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

/* 
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

*/

// --- Modelo de Datos (Debe coincidir con la struct de Rust en src-tauri) ---
#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Wallpaper {
    path: String,
    name: String,
    type_ext: String, // ej. "jpg", "mp4"
}

// --- Función auxiliar para llamar al backend ---
async fn apply_wallpaper(w: Wallpaper) {
    let args = serde_wasm_bindgen::to_value(&w).unwrap();
    // Llamamos al comando invoke("apply_wallpaper", ...)
    invoke("apply_wallpaper", args).await;
}

#[component]
pub fn App() -> impl IntoView {
    // Definimos los filtros que queremos mostrar
    let filters = vec!["ALL", "PIC", "VID", "W.E.", "ANY", "BIZE", "FAV"];
    
    // Por ahora, simulamos 15 hexágonos vacíos
    let items = (0..3).collect::<Vec<usize>>();

    // En un futuro, aquí pedirás la lista real a Tauri:
    // let wallpapers = create_resource(move || (), |_| async { get_wallpapers().await });

    view! {
        <main class="main-container">
            // 1. Barra de Filtros
            <nav class="filter-bar">
                {filters.into_iter().map(|f| {
                    view! { <button class="filter-btn">{f}</button> }
                }).collect_view()}
            </nav>

            // 2. Contenedor de Scroll Horizontal
            <section class="horizontal-scroll-container">
                // Usamos la nueva clase "honeycomb-grid"
                <div class="honeycomb-grid">
                    <For 
                        each=move || items.clone()
                        key=|item| *item
                        children=move |index| {
                            // Simulamos un Wallpaper para la interacción
                            let wallpaper = Wallpaper {
                                path: "".to_string(),
                                name: format!("WP_{}", index),
                                type_ext: if index % 3 == 0 { "mp4".to_string() } else { "jpg".to_string() },
                            };

                            view! {
                                <div 
                                    class="hexagon-wrapper"
                                    // Al hacer clic, invocamos la función asíncrona
                                    on:click=move |_| {
                                        let w_clone = wallpaper.clone();
                                        spawn_local(async move {
                                            apply_wallpaper(w_clone).await;
                                        });
                                    }
                                >
                                    <div class="hexagon-inner">
                                        // Aquí pondrías la imagen real en el futuro:
                                        // <img src={convert_path(w_path)} class="hexagon-image" />
                                        
                                        // Marcador visual (PH = Placeholder)
                                        <div class="hexagon-content">
                                            <span class="placeholder-icon">
                                                {if index % 3 == 0 { "VID" } else { "PIC" }}
                                            </span>
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