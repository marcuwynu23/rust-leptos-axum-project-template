//! WASM build artifact: the **frontend** crate is not where the UI lives — the UI is in **app**.
//! This crate exists because the toolchain needs a separate *library* compiled to `wasm32` that
//! exports `hydrate()`. The server is a native binary; we can't compile one crate to both.
//! So: **app** = shared UI + logic (used by server for SSR and by this crate for the browser).

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
