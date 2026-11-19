use crate::models::FilterKind;
use leptos::prelude::*;

#[component]
pub fn FilterBar(
    current: ReadSignal<FilterKind>,
    on_select: WriteSignal<FilterKind>,
) -> impl IntoView {
    let filters = [
        FilterKind::All,
        FilterKind::Img,
        FilterKind::Gif,
        FilterKind::Vid,
    ];

    view! {
        <nav class="filter-bar">
            {filters.into_iter().map(|filter| {
                view! {
                    <button
                        class=move || if current.get() == filter { "filter-btn active" } else { "filter-btn" }
                        on:click=move |_| on_select.set(filter)
                    >
                        {filter.label()}
                    </button>
                }
            }).collect_view()}
        </nav>
    }
}