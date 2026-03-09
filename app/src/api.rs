//! Client-side API helpers. Fetches from /api (same origin).

use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

/// Result of checking API health. Used for UI only.
#[derive(Clone)]
#[allow(dead_code)]
pub enum ApiStatus {
    Loading,
    Ok(HealthResponse),
    Error(String),
}

/// Fetches GET /api/health. In the browser (hydrate build) calls the API; on SSR returns Loading.
pub async fn fetch_health() -> ApiStatus {
    #[cfg(feature = "gloo-net")]
    {
        let url = "/api/health";
        match gloo_net::http::Request::get(url).send().await {
            Ok(resp) if resp.ok() => {
                match resp.json::<HealthResponse>().await {
                    Ok(data) => ApiStatus::Ok(data),
                    Err(e) => ApiStatus::Error(format!("Invalid response: {}", e)),
                }
            }
            Ok(resp) => ApiStatus::Error(format!("HTTP {}", resp.status())),
            Err(e) => ApiStatus::Error(e.to_string()),
        }
    }

    #[cfg(not(feature = "gloo-net"))]
    {
        let _ = ();
        ApiStatus::Loading
    }
}

/// Shows API status (loading / ok / error). Call from any page; fetches on mount (client-only).
#[component]
pub fn ApiStatus(
    /// Optional class for the wrapper (e.g. "text-sm").
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let api_status = RwSignal::new(Option::<ApiStatus>::None);

    Effect::new(move |_| {
        spawn_local(async move {
            let result = fetch_health().await;
            api_status.set(Some(result));
        });
    });

    let class = class.unwrap_or("");

    view! {
        <div class=class>
            {move || {
                let (span_class, dot_class, label): (&str, &str, String) = match api_status.get() {
                    None | Some(ApiStatus::Loading) => (
                        "inline-flex items-center gap-1.5 text-zinc-500",
                        "inline-block h-2 w-2 animate-pulse rounded-full bg-zinc-500",
                        "API: checking…".to_string(),
                    ),
                    Some(ApiStatus::Ok(ref h)) => (
                        "inline-flex items-center gap-1.5 text-emerald-500",
                        "inline-block h-2 w-2 rounded-full bg-emerald-500",
                        format!("API: {} ({})", h.status, h.service),
                    ),
                    Some(ApiStatus::Error(_)) => (
                        "inline-flex items-center gap-1.5 text-red-400",
                        "inline-block h-2 w-2 rounded-full bg-red-400",
                        "API: error".to_string(),
                    ),
                };
                view! {
                    <span class=span_class>
                        <span class=dot_class/>
                        {label}
                    </span>
                }
            }}
        </div>
    }
}
