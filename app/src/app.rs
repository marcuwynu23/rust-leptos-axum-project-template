use leptos::prelude::*;
use leptos_router::{components::Route, components::Router, components::Routes, path};
use crate::pages::{HomePage, NotFound};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="app">
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        </main>
    }
}
