use crate::api::{apply_wallpaper, fetch_wallpapers};
use crate::components::filter_bar::FilterBar;
use crate::components::hex_grid::HexGrid;
use crate::models::{FilterKind, WallpaperItem};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    let (filter, set_filter) = signal(FilterKind::All);
    let (selected_path, set_selected_path) = signal::<Option<String>>(None);
    let (status_msg, set_status_msg) = signal::<Option<String>>(None);

    let wallpapers = LocalResource::new(move || {
        let current = filter.get();
        async move { fetch_wallpapers(current).await }
    });

    let on_pick = Callback::new(move |item: WallpaperItem| {
        set_selected_path.set(Some(item.path.clone()));
        spawn_local(async move {
            if let Some(res) = apply_wallpaper(item).await {
                set_status_msg.set(Some(res.message));
            }
        });
    });

    view! {
        <main class="app-container">
            <FilterBar current=filter on_select=set_filter />

            {move || status_msg.get().map(|msg| view! {
                <div class="status-pill">{msg}</div>
            })}

            <Suspense fallback=move || view! { <div class="empty-state">"Escaneando wallpapers..."</div> }>
                {move || {
                    let list = wallpapers.get().unwrap_or_default();
                    view! {
                        <HexGrid
                            items=list
                            selected_path=selected_path
                            on_pick=on_pick
                        />
                    }
                }}
            </Suspense>
        </main>
    }
}