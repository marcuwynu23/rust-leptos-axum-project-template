use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-semibold text-zinc-100">"Settings"</h1>
            <p class="text-zinc-400">"Example settings page. No real persistence."</p>
            <div class="max-w-md space-y-4 rounded-xl border border-zinc-800 bg-zinc-900/50 p-4">
                <label class="flex items-center justify-between gap-4">
                    <span class="text-sm text-zinc-300">"Theme"</span>
                    <select class="rounded-lg border border-zinc-700 bg-zinc-800 px-3 py-1.5 text-sm text-zinc-200">
                        <option value="dark">"Dark"</option>
                        <option value="light">"Light"</option>
                    </select>
                </label>
                <label class="flex items-center justify-between gap-4">
                    <span class="text-sm text-zinc-300">"Notifications"</span>
                    <input type="checkbox" class="rounded border-zinc-600"/>
                </label>
            </div>
        </div>
    }
}
