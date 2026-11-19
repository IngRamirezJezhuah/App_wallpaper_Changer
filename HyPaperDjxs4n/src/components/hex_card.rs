use crate::api::asset_url;
use crate::models::{MediaKind, WallpaperItem};
use leptos::prelude::*;

#[component]
pub fn HexCard(
    item: WallpaperItem,
    is_selected: Signal<bool>,
    on_pick: Callback<WallpaperItem>,
) -> impl IntoView {
    let item_click = item.clone();
    let badge = item.ext.to_uppercase();
    let is_video = item.kind == MediaKind::Video;
    let preview_src = asset_url(&item.path);
    let alt_name = item.name.clone();

    view! {
        <div
            class=move || if is_selected.get() { "hexagon-wrapper selected" } else { "hexagon-wrapper" }
            title=item.name.clone()
            on:click=move |_| on_pick.run(item_click.clone())
        >
            <div class="hexagon-inner">
                {if is_video {
                    view! {
                        <div class="hexagon-video-fallback">
                            <span class="video-icon">"▶"</span>
                            <span class="video-name">{alt_name}</span>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <img
                            src=preview_src
                            alt=alt_name
                            class="hexagon-image"
                            loading="lazy"
                        />
                    }.into_any()
                }}
                <div class="hexagon-badge">{badge}</div>
            </div>
        </div>
    }
}