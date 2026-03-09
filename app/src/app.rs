use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use crate::pages::{
    AppLayout, AuthPage, DashboardPage, LandingPage, NotFound, ProfilePage, SettingsPage,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="app">
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=LandingPage/>
                    <Route path=path!("/auth") view=AuthRedirect/>
                    <Route path=path!("/auth/:mode") view=AuthPage/>
                    <ParentRoute path=path!("/app") view=AppLayout>
                        <Route path=path!("") view=DashboardPage/>
                        <Route path=path!("settings") view=SettingsPage/>
                        <Route path=path!("profile") view=ProfilePage/>
                    </ParentRoute>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn AuthRedirect() -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move |_| {
        navigate("/auth/login", Default::default());
    });
    view! { <div class="p-6 text-zinc-400">"Redirecting…"</div> }
}
