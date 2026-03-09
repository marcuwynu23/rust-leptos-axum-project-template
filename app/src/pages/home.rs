use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| count.update(|n| *n += 1);

    view! {
        <div class="page home">
            <h1>"Leptos + Axum"</h1>
            <p>"Monorepo template with SSR and optional API."</p>
            <button on:click=on_click>"Clicked " {count} " times"</button>
        </div>
    }
}
