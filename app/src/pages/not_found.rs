use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page text-center">
            <h1 class="text-3xl font-semibold text-zinc-100">"404"</h1>
            <p class="mt-2 text-zinc-400">"Page not found."</p>
            <a href="/" class="mt-4 inline-block text-violet-400 hover:text-violet-300">"Go home"</a>
        </div>
    }
}
