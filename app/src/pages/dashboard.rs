use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-semibold text-zinc-100">"Dashboard"</h1>
            <p class="text-zinc-400">"Main app content. Sidebar and navbar are example layout only — no restrictions."</p>
            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <Card title="Overview" description="Example card."/>
                <Card title="Activity" description="Example card."/>
                <Card title="Quick actions" description="Example card."/>
            </div>
        </div>
    }
}

#[component]
fn Card(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-zinc-800 bg-zinc-900/50 p-4">
            <h2 class="font-medium text-zinc-100">{title}</h2>
            <p class="mt-1 text-sm text-zinc-500">{description}</p>
        </div>
    }
}
