use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq)]
pub struct AuthModeParams {
    pub mode: String,
}

#[component]
pub fn AuthPage() -> impl IntoView {
    let params = use_params::<AuthModeParams>();
    let mode = Memo::new(move |_| {
        params
            .get()
            .map(|p| p.mode.clone())
            .unwrap_or_else(|_| "login".to_string())
    });

    view! {
        <div class="min-h-screen flex flex-col items-center justify-center px-4 bg-[#0f0f12]">
            <div class="w-full max-w-sm space-y-6">
                <div class="text-center">
                    <a href="/" class="text-lg font-semibold text-zinc-100">"App"</a>
                </div>

                <div class="rounded-xl border border-zinc-800 bg-zinc-900/50 p-6 shadow-xl">
                    <AuthTabs mode=mode/>
                    <div class="mt-6">
                        <AuthForm mode=mode/>
                    </div>
                </div>

                <p class="text-center text-sm text-zinc-500">
                    "Example only — no real auth or restrictions."
                </p>
            </div>
        </div>
    }
}

#[component]
fn AuthTabs(mode: Memo<String>) -> impl IntoView {
    view! {
                <div class="flex rounded-lg bg-zinc-800/50 p-1">
            <a
                href="/auth/login"
                class=move || {
                    if mode.get() == "login" {
                        "flex-1 rounded-md bg-violet-500 py-2 text-center text-sm font-medium text-white"
                    } else {
                        "flex-1 rounded-md py-2 text-center text-sm font-medium text-zinc-400 hover:text-zinc-200"
                    }
                }
            >
                "Log in"
            </a>
            <a
                href="/auth/register"
                class=move || {
                    if mode.get() == "register" {
                        "flex-1 rounded-md bg-violet-500 py-2 text-center text-sm font-medium text-white"
                    } else {
                        "flex-1 rounded-md py-2 text-center text-sm font-medium text-zinc-400 hover:text-zinc-200"
                    }
                }
            >
                "Register"
            </a>
            <a
                href="/auth/recovery"
                class=move || {
                    if mode.get() == "recovery" {
                        "flex-1 rounded-md bg-violet-500 py-2 text-center text-sm font-medium text-white"
                    } else {
                        "flex-1 rounded-md py-2 text-center text-sm font-medium text-zinc-400 hover:text-zinc-200"
                    }
                }
            >
                "Recovery"
            </a>
        </div>
    }
}

#[component]
fn AuthForm(mode: Memo<String>) -> impl IntoView {
    view! {
        {move || match mode.get().as_str() {
            "register" => view! { <RegisterForm/> }.into_any(),
            "recovery" => view! { <RecoveryForm/> }.into_any(),
            _ => view! { <LoginForm/> }.into_any(),
        }}
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    view! {
        <form class="space-y-4" on:submit=|ev| ev.prevent_default()>
            <div>
                <label for="login-email" class="block text-sm font-medium text-zinc-300">"Email"</label>
                <input
                    id="login-email"
                    type="email"
                    placeholder="you@example.com"
                    class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-violet-500 focus:outline-none focus:ring-1 focus:ring-violet-500"
                />
            </div>
            <div>
                <label for="login-password" class="block text-sm font-medium text-zinc-300">"Password"</label>
                <input
                    id="login-password"
                    type="password"
                    placeholder="••••••••"
                    class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-violet-500 focus:outline-none focus:ring-1 focus:ring-violet-500"
                />
            </div>
            <a href="/auth/recovery" class="block text-sm text-violet-400 hover:text-violet-300">"Forgot password?"</a>
            <button
                type="submit"
                class="w-full rounded-lg bg-violet-500 py-2 font-medium text-white hover:bg-violet-400 transition"
            >
                "Log in"
            </button>
        </form>
    }
}

#[component]
fn RegisterForm() -> impl IntoView {
    view! {
        <form class="space-y-4" on:submit=|ev| ev.prevent_default()>
            <div>
                <label for="reg-name" class="block text-sm font-medium text-zinc-300">"Name"</label>
                <input
                    id="reg-name"
                    type="text"
                    placeholder="Your name"
                    class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-violet-500 focus:outline-none focus:ring-1 focus:ring-violet-500"
                />
            </div>
            <div>
                <label for="reg-email" class="block text-sm font-medium text-zinc-300">"Email"</label>
                <input
                    id="reg-email"
                    type="email"
                    placeholder="you@example.com"
                    class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-violet-500 focus:outline-none focus:ring-1 focus:ring-violet-500"
                />
            </div>
            <div>
                <label for="reg-password" class="block text-sm font-medium text-zinc-300">"Password"</label>
                <input
                    id="reg-password"
                    type="password"
                    placeholder="••••••••"
                    class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-violet-500 focus:outline-none focus:ring-1 focus:ring-violet-500"
                />
            </div>
            <button
                type="submit"
                class="w-full rounded-lg bg-violet-500 py-2 font-medium text-white hover:bg-violet-400 transition"
            >
                "Create account"
            </button>
        </form>
    }
}

#[component]
fn RecoveryForm() -> impl IntoView {
    view! {
        <form class="space-y-4" on:submit=|ev| ev.prevent_default()>
            <p class="text-sm text-zinc-400">
                "Enter your email and we’ll send a reset link (example only)."
            </p>
            <div>
                <label for="rec-email" class="block text-sm font-medium text-zinc-300">"Email"</label>
                <input
                    id="rec-email"
                    type="email"
                    placeholder="you@example.com"
                    class="mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-violet-500 focus:outline-none focus:ring-1 focus:ring-violet-500"
                />
            </div>
            <button
                type="submit"
                class="w-full rounded-lg bg-violet-500 py-2 font-medium text-white hover:bg-violet-400 transition"
            >
                "Send reset link"
            </button>
            <a href="/auth/login" class="block text-center text-sm text-violet-400 hover:text-violet-300">"Back to log in"</a>
        </form>
    }
}
