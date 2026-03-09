use leptos::prelude::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-semibold text-zinc-100">"Profile"</h1>
            <p class="text-zinc-400">"Example profile page. No real auth or persistence."</p>
            <div class="max-w-md space-y-4 rounded-xl border border-zinc-800 bg-zinc-900/50 p-4">
                <div>
                    <label class="block text-sm text-zinc-500">"Display name"</label>
                    <input
                        type="text"
                        value="Example user"
                        class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800 px-3 py-2 text-zinc-100"
                    />
                </div>
                <div>
                    <label class="block text-sm text-zinc-500">"Email"</label>
                    <input
                        type="email"
                        value="user@example.com"
                        class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800 px-3 py-2 text-zinc-100"
                    />
                </div>
            </div>
        </div>
    }
}
