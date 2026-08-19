use leptos::attr::r#async;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use leptos::*;
//use leptos::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
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
    // esta madre al comando invoke("apply_wallpaper", ...)
    invoke("apply_wallpaper", args).await;
}

#[component]
pub fn App() -> impl IntoView {
    let filters = vec![ "ALL", "PIC", "VID"];
    // la chinagdera de cuantos cuadros hay
    let items = (0..30).collect::<Vec<usize>>();
    //let wallpapers = create_resource(move || (), |_| async { get_wallpapers().await });
    //let wallpapers = create_slice(/* signal */, move |_| (), |_| async { get_wallpapers().await });
    view! {
        <main class="app-container">
            // Barra de Filtros
            <nav class="filter-bar">
                {filters.into_iter().map(|f| {
                    view! { <button class="filter-btn">{f}</button> }
                }).collect_view()}
            </nav>

            <section class="horizontal-scroll-container">
                <div class="honeycomb-grid">
                /*
                <For
                        each=move || wallpapers.read()
                        key=|wallpaper| wallpaper.path.clone()
                        children=move |wallpaper| {
                            view! {
                                <div 
                                    class="hexagon-wrapper"
                                    on:click=move |_| {
                                        let w_clone = wallpaper.clone();
                                        spawn_local(async move {
                                            apply_wallpaper(w_clone).await;
                                        });
                                    }
                                >
                                    <div class="hexagon-inner">
                                        <img src={wallpaper.path} class="hexagon-image" />
                                        <div class="hexagon-content">
                                            <span class="placeholder-icon">
                                                {if wallpaper.type_ext == "mp4" { "VID" } else { "PIC" }}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                */
                    <For 
                        each=move || items.clone()
                        key=|item| *item
                        children=move |index| {
                            let wallpaper = Wallpaper {
                                path: "".to_string(),
                                name: format!("WP_{}", index),
                                type_ext: if index % 3 == 0 { "mp4".to_string() } else { "jpg".to_string() },
                            };

                            view! {
                                <div 
                                    class="hexagon-wrapper"
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
                                        <img src="./public/wallpapers/147074309_p0_master1200.png" alt="Img" class="img"/>
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