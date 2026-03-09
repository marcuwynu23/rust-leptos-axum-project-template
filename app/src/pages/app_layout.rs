use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::api::ApiStatus;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="flex h-screen bg-[#0f0f12] text-zinc-200">
            <Sidebar/>
            <div class="flex flex-1 flex-col min-w-0">
                <Navbar/>
                <main class="flex-1 overflow-auto p-6">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside class="flex w-56 flex-col border-r border-zinc-800/50 bg-zinc-900/30">
            <div class="flex h-14 items-center border-b border-zinc-800/50 px-4">
                <a href="/app" class="font-semibold text-zinc-100">"App"</a>
            </div>
            <nav class="flex-1 space-y-0.5 p-3">
                <SidebarLink href="/app" label="Dashboard" icon="📊"/>
                <SidebarLink href="/app/settings" label="Settings" icon="⚙️"/>
                <SidebarLink href="/app/profile" label="Profile" icon="👤"/>
                <div class="my-3 border-t border-zinc-800/50"/>
                <a href="/" class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200">
                    <span>"🏠"</span>
                    "Landing"
                </a>
            </nav>
        </aside>
    }
}

#[component]
fn SidebarLink(href: &'static str, label: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200"
        >
            <span>{icon}</span>
            {label}
        </a>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <header class="flex h-14 shrink-0 items-center justify-between border-b border-zinc-800/50 bg-zinc-900/30 px-6">
            <div class="flex items-center gap-4">
                <span class="text-sm text-zinc-500">"Main app"</span>
                <ApiStatus class="text-xs"/>
            </div>
            <div class="flex items-center gap-3">
                <span class="text-sm text-zinc-400">"Example user"</span>
                <a href="/auth/login" class="rounded-lg border border-zinc-600 px-3 py-1.5 text-sm text-zinc-300 hover:bg-zinc-800">"Log out"</a>
            </div>
        </header>
    }
}
