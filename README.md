# Leptos + Axum Monorepo Template

A **highly customizable** web project template using [Leptos](https://leptos.dev) (frontend + SSR) and [Axum](https://github.com/tokio-rs/axum) in a single **monorepo**.

## Features

- **Leptos** for reactive UI, SSR, and hydration
- **Axum** for the server and REST API routes at `/api`
- **Monorepo** with three crates: `app` (shared UI), `frontend` (WASM), `server` (Axum + API)
- **Customizable** via `Cargo.toml` workspace metadata and env vars
- **GitHub Actions** workflow for check and release build

## Prerequisites

- **Rust** (nightly; see [rust-toolchain.toml](rust-toolchain.toml))
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`
- **cargo-leptos**: `cargo install cargo-leptos --locked`
- **Node.js** (for Tailwind CSS): [Install](https://nodejs.org/) and ensure `npm` is on your `PATH`

## How to run

**One-time setup** (from the project root):

```bash
# 1. Use nightly + wasm32 (rust-toolchain.toml does this; run once)
rustup target add wasm32-unknown-unknown

# 2. Install the Leptos build tool
cargo install cargo-leptos --locked
```

**Windows (OpenSSL/Perl):** `pwsh scripts\script.ps1 install` will use system OpenSSL if Perl is not installed and OpenSSL is found in `C:\OpenSSL-light`, `C:\Program Files\OpenSSL-Win64`, or similar. If none is found, the script can **download and install** the full OpenSSL to `C:\OpenSSL-light` automatically. Otherwise:

- **Option A — Install Perl:** [Strawberry Perl](https://strawberryperl.com/) (add to PATH), then `pwsh scripts\script.ps1 install`.
- **Option B — Install OpenSSL:** Use the **full** [Win64 OpenSSL](https://slproweb.com/products/Win32OpenSSL.html) installer (not "Light") so `lib\` contains `libcrypto.lib` and `libssl.lib`. Then in PowerShell:
  ```powershell
  $env:OPENSSL_NO_VENDOR = "1"
  $env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"   # or your install path
  pwsh scripts\script.ps1 install
  ```

```bash
# 3. Install deps and build Tailwind CSS (style/main.css)
npm install
npm run build:css
```

**Development** (hot reload):

```bash
cargo leptos watch
```

Then open **http://127.0.0.1:3000**. The server and assets (WASM/CSS) are built and served automatically. For CSS hot reload while editing `style/src.css` or Tailwind classes, run `npm run watch:css` in another terminal.

**Production** (build once, then run the server):

```bash
cargo leptos build --release
# Server binary: target/server/release/server
# Site (JS/WASM/CSS): target/site/

# Run the server (from project root; it serves from target/site)
cargo run -p server --release
# Or run the binary directly:
# ./target/server/release/server   (Linux/Mac)
# target\server\release\server.exe (Windows)
```

## Commands

Use the project scripts or raw cargo:

| Action | Windows (PowerShell) | Linux/macOS (Bash) | Cargo |
|--------|---------------------|-------------------|-------|
| One-time setup | `pwsh scripts\script.ps1 install` | `bash scripts/script.sh install` | See "How to run" above |
| Dev server | `pwsh scripts\script.ps1 dev` | `bash scripts/script.sh dev` | `cargo leptos watch` |
| Release build | `pwsh scripts\script.ps1 build` | `bash scripts/script.sh build` | `cargo leptos build --release` |
| Run server | `pwsh scripts\script.ps1 run` | `bash scripts/script.sh run` | `cargo run -p server --release` |
| Check | `pwsh scripts\script.ps1 check` | `bash scripts/script.sh check` | `cargo check -p app -p server` |
| Clean | `pwsh scripts\script.ps1 clean` | `bash scripts/script.sh clean` | — |
| Free space (if install fails) | `pwsh scripts\script.ps1 free-space` | `bash scripts/script.sh free-space` | — |

**Linux/macOS:** make the script executable once: `chmod +x scripts/script.sh`  
Script help: `pwsh scripts\script.ps1 --help` or `bash scripts/script.sh --help`

### If install fails: "no space on device" / LNK1318 PDB

`cargo install cargo-leptos` needs several GB free (C: and temp). Do this:

1. **Free at least 5–10 GB** on the drive where Windows temp and Cargo run (usually C:).
2. **Run the script:** `pwsh scripts\script.ps1 free-space` — it cleans this project’s `target\`, cargo-install temp dirs, and the cargo registry cache.
3. **Optional:** Remove unused Rust toolchains: `rustup toolchain list` then `rustup toolchain uninstall <name>`.
4. **Retry:** `pwsh scripts\script.ps1 install`.

## Project Layout

```
.
├── Cargo.toml              # Workspace + Leptos config (customize here)
├── rust-toolchain.toml     # Nightly + wasm32
├── app/                    # UI + logic (shared: SSR on server, hydrate in browser)
├── frontend/               # WASM build only: tiny crate that calls hydrate() with app
├── server/                 # Axum server (SSR + /api routes)
│   └── src/api.rs          # REST API handlers (e.g. GET /api/health)
├── style/                  # Tailwind: src.css → main.css (see package.json)
├── public/                 # Static assets (copied to site)
├── scripts/                # script.ps1 (Windows), script.sh (Linux/macOS)
├── .github/workflows/      # CI (build.yml)
├── README.md
└── DEVELOPER_GUIDE.md      # Detailed developer guide
```

## Customization

- **App name / site address / assets**: Edit `[workspace.metadata.leptos]` in [Cargo.toml](Cargo.toml).
- **Env overrides**: Copy [.env.example](.env.example) to `.env` and set `LEPTOS_ENV`, `LEPTOS_SITE_ADDR`, etc.
- **API routes**: Add handlers in [server/src/api.rs](server/src/api.rs); they are mounted at `/api`.
- **Styles**: Edit [style/src.css](style/src.css) and Tailwind classes in components; run `npm run build:css` (or `npm run watch:css` in dev).
- **CI**: Adjust [.github/workflows/build.yml](.github/workflows/build.yml) (e.g. deploy, matrix).

## License

MIT or Apache-2.0, at your option.
