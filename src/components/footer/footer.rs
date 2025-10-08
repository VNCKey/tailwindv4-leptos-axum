use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <p>"© 2025 Mi Web. Todos los derechos reservados."</p>
            </div>
        </footer>
    }
}
