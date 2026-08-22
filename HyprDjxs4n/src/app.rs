use leptos::task::spawn_local;
//use leptos::{svg::Filter, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
// --- Modelo de Datos (Debe coincidir con la struct de Rust en src-tauri) ---
#[derive(Clone, Serialize, Deserialize, PartialEq,Debug)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub type_ext: String,
    pub is_animated: bool,
    pub is_placeholder: bool,
}

#[derive( Serialize, Deserialize )]
struct GetArgs {
    filter: String,
}

async fn fetch_wallpapers( filter_val: String) -> Vec<Wallpaper> {
    let args = serde_wasm_bindgen::to_value(&GetArgs { filter: filter_val }).unwrap();
    let res = invoke("get_wallpapers", args).await;
    serde_wasm_bindgen::from_value(res).unwrap_or_default()
}

// --- Función auxiliar para llamar al backend ---
async fn apply_wallpaper(w: Wallpaper) {
    if w.is_placeholder { return; }
    let args = serde_wasm_bindgen::to_value(&w).unwrap();
    // esta madre al comando invoke("apply_wallpaper", ...)
    invoke("apply_wallpaper", args).await;
}

#[component]
pub fn App() -> impl IntoView {
    
    let (curent_filter, set_filter ) = signal("ALL".to_string());
    
    let wallpapers_resource = LocalResource::new(move || {
        let f = curent_filter.get();
        async move { fetch_wallpapers(f).await }
    });

    let filters = vec!["ALL", "PIC", "VID"];

    view! {
        <main class="app-container">
            // Barra de Filtros
            <nav class="filter-bar">
                {filters.into_iter().map(|f| {
                    let f_str = f.to_string();
                    let f_click = f_str.clone();
                    view! { 
                        <button 
                            class=move || if curent_filter.get() == f_str { "filter-btn active" } else { "filter-btn" }
                            on:click=move |_| set_filter.set(f_click.clone())
                            >
                        {f}
                        </button> 
                    }
                }).collect_view()}
            </nav>
            /*<section class="horizontal-scroll-container">
                <div class="honeycomb-grid">
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
            */
            <section class="horizontal-scroll-container">
                <div class="honeycomb-grid">
                    <Suspense fallback=move || view! { <div class="loading">"Cargando wallpapers..."</div> }>
                        {move || {
                            let list = wallpapers_resource.get().unwrap_or_default();
                            list.into_iter().map(|wallpaper| {
                                let w_click = wallpaper.clone();
                                let is_ph = wallpaper.is_placeholder;
                                let wrapper_class = if is_ph { "hexagon-wrapper placeholder" } else { "hexagon-wrapper" };
                                let img_src = wallpaper.path.clone();
                                let tag_label = wallpaper.type_ext.to_uppercase();

                                view! {
                                    <div 
                                        class={wrapper_class}
                                        on:click=move |_| {
                                            if !is_ph {
                                                let w = w_click.clone();
                                                spawn_local(async move {
                                                    apply_wallpaper(w).await;
                                                });
                                            }
                                        }
                                    >
                                        <div class="hexagon-inner">
                                            {if !is_ph {
                                                view! {
                                                    <img src={img_src} alt={wallpaper.name.clone()} class="hexagon-image" />
                                                    <div class="hexagon-content">
                                                        <span class="placeholder-icon">{tag_label}</span>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="hexagon-content disabled">
                                                        <span class="placeholder-icon">"EMPTY"</span>
                                                    </div>
                                                }.into_any()
                                            }}
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </Suspense>
                </div>
            </section>
        </main>
    }
}