#!/usr/bin/env bash
# Leptos + Axum project script (Linux/macOS)
# Usage: bash scripts/script.sh <command>
#   install  - one-time setup (rustup target, cargo-leptos, npm + Tailwind CSS)
#   dev      - dev server with hot reload (cargo leptos watch)
#   build    - release build (server + site)
#   run      - run release server (after build)
#   check      - cargo check
#   clean      - remove target/
#   free-space - free disk space if install fails with "no space"

set -e
cmd="${1:-}"

show_help() {
    cat << 'EOF'
Leptos + Axum script
Usage: bash scripts/script.sh <command>

Commands:
  install     One-time setup: wasm32 target, cargo-leptos, npm + Tailwind CSS
  dev         Start dev server with hot reload (cargo leptos watch)
  build       Release build (cargo leptos build --release)
  run         Run release server (cargo run -p server --release)
  check       Run cargo check
  clean       Remove target/

Examples:
  bash scripts/script.sh install
  bash scripts/script.sh dev
EOF
}

script_install() {
    echo -e "\033[36m=== Install (one-time setup) ===\033[0m"

    echo -e "\n\033[33m[1/3] Adding wasm32 target...\033[0m"
    rustup target add wasm32-unknown-unknown

    echo -e "\n\033[33m[2/3] Installing cargo-leptos...\033[0m"
    cargo install cargo-leptos --locked

    echo -e "\n\033[33m[3/3] npm + Tailwind CSS...\033[0m"
    if command -v npm &>/dev/null; then
        npm install
        npm run build:css
        echo -e "\033[32mTailwind CSS built (style/main.css)\033[0m"
    else
        echo -e "\033[33mnpm not found. Install Node.js (https://nodejs.org/) then run: npm install && npm run build:css\033[0m"
    fi

    echo -e "\n\033[32mDone. Run: bash scripts/script.sh dev\033[0m"
}

script_dev() {
    echo -e "\033[36m=== Dev server (cargo leptos watch) ===\033[0m"
    if command -v npm &>/dev/null && [[ ! -f style/main.css ]]; then
        npm run build:css
    fi
    echo -e "\033[90mOpen http://127.0.0.1:3000\033[0m"
    cargo leptos watch
}

script_build() {
    echo -e "\033[36m=== Release build ===\033[0m"
    if command -v npm &>/dev/null; then npm run build:css; fi
    cargo leptos build --release
    echo -e "\033[32mServer: target/server/release/server\033[0m"
    echo -e "\033[32mSite:   target/site/\033[0m"
}

script_run() {
    echo -e "\033[36m=== Run release server ===\033[0m"
    if [[ ! -f target/server/release/server ]]; then
        echo -e "\033[33mRun 'bash scripts/script.sh build' first.\033[0m"
        exit 1
    fi
    echo -e "\033[90mOpen http://127.0.0.1:3000\033[0m"
    cargo run -p server --release
}

script_check() {
    echo -e "\033[36m=== Cargo check ===\033[0m"
    cargo check --workspace --exclude server --exclude frontend
    cargo check -p server
}

script_clean() {
    echo -e "\033[36m=== Clean (remove target/) ===\033[0m"
    if [[ -d target ]]; then
        rm -rf target
        echo -e "\033[32mRemoved target/\033[0m"
    else
        echo -e "\033[90mNo target/ folder.\033[0m"
    fi
}


case "$cmd" in
    install)     script_install ;;
    dev)         script_dev ;;
    build)       script_build ;;
    run)         script_run ;;
    check)       script_check ;;
    clean)       script_clean ;;
    -h|--help|"")
        show_help
        ;;
    *)
        echo -e "\033[31mUnknown command: $cmd\033[0m"
        show_help
        exit 1
        ;;
esac
