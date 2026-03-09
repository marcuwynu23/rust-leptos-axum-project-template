# Developer Guide — Leptos + Axum Monorepo

This guide explains how the template is structured, how to customize it, and how to extend it with new features.

---

## 1. Architecture Overview

### 1.1 Cargo Workspace

The repo is a **Cargo workspace** with three members:

| Crate     | Role |
|----------|------|
| **app**  | Shared Leptos application: components, pages, routes, and server functions. Built with feature `ssr` (server) or `hydrate` (client). |
| **frontend** | Client-only entry: compiles to WASM and exports `hydrate()`, which mounts the app in the browser. Depends on `app` with `hydrate`. |
| **server** | Axum binary: Leptos SSR, static files, and REST API. The `server::api` module (e.g. `server/src/api.rs`) holds `/api` routes and is always included. Depends on `app` with `ssr`. |

### 1.2 Data Flow

- **SSR**: Request → Axum → Leptos route list → `app::shell` + `app::App` → HTML stream → response.
- **Hydration**: Browser loads HTML, then loads WASM and calls `hydrate()` → Leptos hydrates the same component tree.
- **API**: Requests to `/api/*` are handled by `server::api::router()` (see `server/src/api.rs`).

### 1.3 Build Pipeline (cargo-leptos)

- **cargo-leptos** reads `[[workspace.metadata.leptos]]` from the **workspace** `Cargo.toml` (not from a separate Leptos.toml by default).
- It builds:
  - **frontend** (lib) → WASM + JS + CSS (e.g. into `target/site/pkg/`).
  - **server** (bin) → native binary (e.g. `target/server/release/server`).
- It compiles `style/main.scss` with dart-sass and optimizes CSS.
- It copies `public/` into `target/site/` (or your configured `site-root`).

---

## 2. Configuration

### 2.1 Leptos Options (Workspace Cargo.toml)

All cargo-leptos settings live under `[[workspace.metadata.leptos]]` in the **root** [Cargo.toml](Cargo.toml). Important keys:

| Key | Purpose | Example |
|-----|---------|--------|
| `name` | Prefix for output files (e.g. `name.js`, `name.css`) | `"app"` |
| `bin-package` | Crate that contains the server binary | `"server"` |
| `lib-package` | Crate that builds the WASM frontend | `"frontend"` |
| `site-root` | Directory for generated site (JS/WASM/CSS + assets) | `"target/site"` |
| `site-pkg-dir` | Subdir under site-root for pkg output | `"pkg"` |
| `style-file` | Main stylesheet (SCSS/CSS) | `"style/main.scss"` |
| `assets-dir` | Static assets (copied to site-root) | `"public"` |
| `site-addr` | Bind address for dev/prod server | `"127.0.0.1:3000"` |
| `reload-port` | Port for live reload (cargo leptos watch) | `3001` |
| `env` | `DEV` or `PROD` (affects defaults) | `"DEV"` |

Changing `name` changes the generated asset names; the server and shell must reference the same paths (e.g. `/pkg/app.css`).

### 2.2 Environment Variables

cargo-leptos and the Leptos config support overrides via env (see [Leptos config](https://docs.rs/leptos/latest/leptos/config/)):

- `LEPTOS_ENV` — `DEV` / `PROD`
- `LEPTOS_SITE_ADDR` — e.g. `0.0.0.0:8080`
- `LEPTOS_SITE_ROOT` — e.g. `./target/site`

Copy [.env.example](.env.example) to `.env` and set these if you need local overrides. The server uses `get_configuration(None)`, which reads from the workspace Cargo.toml and env.

### 2.3 Rust Toolchain

[rust-toolchain.toml](rust-toolchain.toml) pins **nightly** and adds the **wasm32-unknown-unknown** target. Leptos currently expects nightly for full macro support. To change the channel or targets, edit this file.

### 2.4 Windows: OpenSSL (with lib) and Perl

On **Windows**, `cargo install cargo-leptos` pulls in **openssl-sys**, which needs either a **Perl** runtime (to build OpenSSL from source) or a **system OpenSSL** installation that includes **development libraries** (`.lib` files for linking). You need at least one of the two.

#### Option A — System OpenSSL (recommended if you don’t have Perl)

1. **Install the full OpenSSL for Windows** (not the “Light” package). Use the **full** installer from [Win64 OpenSSL](https://slproweb.com/products/Win32OpenSSL.html) so that the install includes a **`lib`** directory with:
   - **`libcrypto.lib`** and **`libssl.lib`** (import libraries used at compile/link time).

2. **Where the `.lib` files live** depends on the installer:
   - **Classic layout:** `C:\Program Files\OpenSSL-Win64\lib\libcrypto.lib` and `libssl.lib`.
   - **VC layout:** Some installers put them under `lib\VC\x64\`, e.g.:
     - `C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD\libcrypto.lib`
     - `C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD\libssl.lib`  
     (or `MT`, `MDd`, `MTd` for other runtimes.)

3. **Set environment variables** before running `pwsh scripts\script.ps1 install` (or `cargo install cargo-leptos`):
   - **If `.lib` are directly in `lib\`:**
     ```powershell
     $env:OPENSSL_NO_VENDOR = "1"
     $env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"   # or your install path
     ```
   - **If `.lib` are in `lib\VC\x64\MD\` (or another VC subdir):**
     ```powershell
     $env:OPENSSL_NO_VENDOR = "1"
     $env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
     $env:OPENSSL_LIB_DIR = "C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD"
     ```
   The project script (`scripts\script.ps1 install`) auto-detects both layouts when no Perl is found.

4. **Runtime DLLs** (e.g. `libssl-3-x64.dll`, `libcrypto-3-x64.dll`) are used when the built binary runs; the **`.lib`** files are what the linker needs at **build** time. Without the correct `OPENSSL_DIR` (and `OPENSSL_LIB_DIR` for the VC layout), `cargo install cargo-leptos` will fail with missing OpenSSL or linker errors.

#### Option B — Perl runtime

- Install a **Perl** runtime (e.g. [Strawberry Perl](https://strawberryperl.com/)) and add it to your `PATH`.
- With Perl available, **openssl-sys** can build OpenSSL from source during `cargo install cargo-leptos`, so you do **not** need a preinstalled OpenSSL with `lib` in that case.
- If you already have OpenSSL with `lib` configured (Option A), Perl is optional for this project.

**Summary:** On Windows, either install **OpenSSL (full, with `lib`)** and set `OPENSSL_DIR` (and `OPENSSL_LIB_DIR` if using the VC path), or install **Perl**; the install script will use whichever it finds.

---

## 3. App Crate (Shared UI and Logic)

### 3.1 Layout

- **app/src/lib.rs** — Exposes `shell()`, `App`, and re-exports `LeptosOptions`. The shell is the root HTML shell used for SSR and hydration (doctype, head, body, router).
- **app/src/app.rs** — Root `App` component: `provide_meta_context()`, `<Routes>` with `<Route>` definitions.
- **app/src/pages/** — Page components (e.g. `HomePage`, `NotFound`). Add new pages here and register routes in `app.rs`.

### 3.2 Adding a Page

1. Create `app/src/pages/my_page.rs` and implement a component that returns `impl IntoView`.
2. In `app/src/pages/mod.rs`, add `pub mod my_page;` and `pub use my_page::MyPage;`.
3. In `app/src/app.rs`, add `<Route path="/my-page" view=MyPage/>`.

### 3.3 Server Functions

To add Leptos server functions (run on server, callable from client):

- In the `app` crate, use `#[server]` and the appropriate path/prefix as per [Leptos server functions](https://book.leptos.dev/server/01_server_functions.html).
- Ensure the server registers `leptos_axum::handle_server_fns()` (the default `.leptos_routes()` setup typically covers this).

### 3.4 Features

- **hydrate** — Used by `frontend`: enables `leptos/hydrate` and is the default for the app when used as a lib from frontend.
- **ssr** — Used by `server`: enables `leptos/ssr`, `leptos_meta/ssr`, `leptos_router/ssr`, and `leptos_axum`. No default; server explicitly depends on `app` with `features = ["ssr"]`.

---

## 4. Frontend Crate (WASM Hydration)

- **frontend/src/lib.rs** — Single entry: `hydrate()` annotated with `#[wasm_bindgen]`. It initializes logging and panic hook, then calls `leptos::mount::hydrate_body(App)`.

You rarely need to change this unless you add top-level client-only setup (e.g. analytics, error reporting). All UI and routing live in `app`.

---

## 5. Server Crate (Axum)

### 5.1 main.rs Flow

1. Initialize logging.
2. Load config with `get_configuration(None)` (from Leptos prelude).
3. Generate route list with `generate_route_list(App)`.
4. Build Axum `Router`:
   - `.leptos_routes(...)` — registers Leptos routes and the shell.
   - `.fallback(file_and_error_handler(shell))` — static files and 404.
   - `.with_state(leptos_options)`.
   - `.nest("/api", api::router())` — REST API (see `server/src/api.rs`).
5. Bind to `site_addr` and run `axum::serve`.

### 5.2 Adding Middleware

Use Tower layers on the `Router` before or after `.leptos_routes()`, e.g.:

```rust
use tower_http::trace::TraceLayer;

let app = Router::new()
    .layer(TraceLayer::new_for_http())
    .leptos_routes(...)
```

### 5.3 API Routes (server/src/api.rs)

- **server/src/api.rs** — Defines `router()` (e.g. `GET /api/health`) and shared types. Add more routes and handlers here.
- The server mounts this at `/api`, so `route("/health", get(health))` becomes `GET /api/health`.
- Use Leptos server functions in `app` for logic called from the UI; use `api.rs` for REST endpoints.

---

## 6. Styling and Assets

- **style/main.scss** — Global styles and CSS variables. cargo-leptos compiles this and outputs to the site pkg dir (e.g. `app.css`). Customize variables (e.g. `--color-accent`) and add components/layouts here.
- **public/** — Static files (favicon, images, etc.) are copied to `site-root` as-is. Reference them from the app with paths like `/favicon.svg`.

---

## 7. GitHub Actions

- **.github/workflows/build.yml**:
  - **check** — `cargo fmt`, `cargo clippy`, and a workspace build that excludes `server` and `frontend` (so we don’t need cargo-leptos in this job).
  - **build** — Installs Rust (nightly + wasm32), cargo-leptos, and dart-sass; runs `cargo leptos build --release`; uploads server binary and site as artifacts.
Customize: add a deploy job, different branches, or a matrix for multiple platforms.

---

## 8. Customization Checklist

- **Project name / branding**: Update `name` in `[[workspace.metadata.leptos]]` and titles in `app/src/lib.rs` (shell).
- **Port / host**: Set `site-addr` or `LEPTOS_SITE_ADDR`.
- **Styles**: Edit `style/main.scss` and CSS variables.
- **New pages**: Add under `app/src/pages/` and register in `app/src/app.rs`.
- **Server functions**: Add in `app` with `#[server]` and wire any needed extractors.
- **REST API**: Add routes in `server/src/api.rs`.
- **CI**: Extend `.github/workflows/build.yml` (e.g. deploy, caching, version matrix).

---

## 9. Troubleshooting

- **`get_configuration(None)` fails** — Ensure `[[workspace.metadata.leptos]]` exists in the **workspace** (root) `Cargo.toml`, not only in a member. Env vars can override.
- **WASM or CSS not found** — Run a full `cargo leptos build` (or `watch`) so that `target/site` and `target/site/pkg` are populated. The server serves from `site-root`.
- **Sass not found** — Install dart-sass and ensure `sass` is on `PATH` when running cargo-leptos.
- **Nightly required** — Use the toolchain from `rust-toolchain.toml` and `rustup target add wasm32-unknown-unknown`.
- **Windows: OpenSSL / Perl when installing cargo-leptos** — If `cargo install cargo-leptos` fails with OpenSSL or linker errors, see **§2.4 Windows: OpenSSL (with lib) and Perl**. You need either a full OpenSSL install (with `lib` and, if needed, `OPENSSL_LIB_DIR`) or a Perl runtime (e.g. Strawberry Perl) on `PATH`.

For more on Leptos and cargo-leptos, see the [Leptos book](https://book.leptos.dev) and [cargo-leptos](https://github.com/leptos-rs/cargo-leptos).
