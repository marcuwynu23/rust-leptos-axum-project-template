use leptos::prelude::*;

use crate::api::ApiStatus;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <header class="border-b border-zinc-800/50 bg-[#0f0f12]/80 backdrop-blur-sm sticky top-0 z-10">
                <nav class="app flex h-14 items-center justify-between">
                    <a href="/" class="text-lg font-semibold text-zinc-100">"App"</a>
                    <div class="flex items-center gap-4">
                        <a href="/auth/login" class="text-zinc-400 hover:text-zinc-100 transition">"Log in"</a>
                        <a href="/auth/register" class="rounded-lg bg-violet-500 px-4 py-2 text-sm font-medium text-white hover:bg-violet-400 transition">"Sign up"</a>
                    </div>
                </nav>
            </header>

            <main class="flex-1 flex flex-col items-center justify-center px-4 py-20">
                <h1 class="text-4xl sm:text-5xl font-bold text-zinc-100 text-center max-w-2xl">
                    "Build something great"
                </h1>
                <p class="mt-4 text-lg text-zinc-400 text-center max-w-xl">
                    "Example landing page. Sign up or log in to try the app — no restrictions, demo only."
                </p>
                <div class="mt-10 flex flex-wrap items-center justify-center gap-4">
                    <a href="/auth/register" class="rounded-lg bg-violet-500 px-6 py-3 font-medium text-white hover:bg-violet-400 transition">"Get started"</a>
                    <a href="/app" class="rounded-lg border border-zinc-600 px-6 py-3 font-medium text-zinc-300 hover:bg-zinc-800/50 transition">"Go to app"</a>
                </div>
            </main>

            <footer class="border-t border-zinc-800/50 py-6">
                <div class="app flex flex-col items-center justify-center gap-2 text-sm text-zinc-500">
                    <span>"Example template · Leptos + Axum"</span>
                    <ApiStatus class="text-xs"/>
                </div>
            </footer>
        </div>
    }
}
