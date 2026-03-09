//! Shared Leptos application (SSR + hydrate).
//! Customize: add pages, components, and server functions here.

use leptos::prelude::*;
use leptos::hydration::HydrationScripts;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

pub use app::App;

mod api;
mod app;
mod pages;

/// Root shell: layout, meta, and router outlet.
/// Called by the server for SSR and by the client for hydration.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Title text="Leptos + Axum"/>
                <Stylesheet id="leptos" href="/pkg/app.css"/>
                <meta name="description" content="Leptos + Axum monorepo template"/>
                <HydrationScripts options=options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
