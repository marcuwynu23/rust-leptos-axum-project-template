//! Axum server: Leptos SSR + /api routes.
//! Customize: add middleware and API routes in api.rs.

use std::path::PathBuf;

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use log::info;
use tower_http::services::ServeDir;

use app::{shell, App};

mod api;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).ok();

    let conf = get_configuration(None).expect("Leptos config (workspace.metadata.leptos)");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();

    let routes = generate_route_list(App);

    // Serve /pkg/* (JS, WASM, CSS) from site_root/pkg so requests get correct MIME types.
    // nest_service("/pkg", ...) strips the prefix, so the service must serve the pkg dir itself.
    let pkg_dir = PathBuf::from(leptos_options.site_root.as_ref()).join(leptos_options.site_pkg_dir.as_ref());
    let pkg_service = ServeDir::new(pkg_dir);

    let app = Router::new()
        .nest_service("/pkg", pkg_service)
        .leptos_routes(&leptos_options, routes, {
            let opts = leptos_options.clone();
            move || shell(opts.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone())
        .nest("/api", api::router());

    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve");
}
