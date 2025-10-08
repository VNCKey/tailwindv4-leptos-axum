/// Renders the home page of your application.
use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1 class="text-4xl font-bold text-sky-100">"Welcome to Leptos!"</h1>
        <button class="mt-4 p-2 bg-blue-500 text-white rounded" on:click=on_click>
            "Click Me: "
            {count}
        </button>
    }
}
