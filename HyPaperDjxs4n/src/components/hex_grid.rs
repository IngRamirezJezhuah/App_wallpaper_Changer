use crate::components::hex_card::HexCard;
use crate::models::WallpaperItem;
use leptos::html::Section;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn HexGrid(
    items: Vec<WallpaperItem>,
    selected_path: ReadSignal<Option<String>>,
    on_pick: Callback<WallpaperItem>,
) -> impl IntoView {
    let scroll_ref = NodeRef::<Section>::new();

    let on_wheel = move |ev: web_sys::WheelEvent| {
        if let Some(el) = scroll_ref.get() {
            let delta = ev.delta_y();
            if delta.abs() > 0.0 {
                ev.prevent_default();
                let element: &web_sys::Element = el.unchecked_ref();
                element.set_scroll_left(element.scroll_left() + delta as i32);
            }
        }
    };

    if items.is_empty() {
        return view! {
            <section class="empty-state">
                <p>"No hay wallpapers para este filtro en tu carpeta de wallpapers."</p>
            </section>
        }.into_any();
    }

    view! {
        <section
            node_ref=scroll_ref
            class="horizontal-scroll-container"
            on:wheel=on_wheel
        >
            <div class="honeycomb-grid">
                {items.into_iter().map(|item| {
                    let path = item.path.clone();
                    let is_selected = Signal::derive(move || {
                        selected_path.get().as_deref() == Some(path.as_str())
                    });
                    view! {
                        <HexCard item=item is_selected=is_selected on_pick=on_pick />
                    }
                }).collect_view()}
            </div>
        </section>
    }.into_any()
}