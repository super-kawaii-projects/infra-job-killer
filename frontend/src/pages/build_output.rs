use leptos::*;
use leptos_router::*;

#[component]
pub fn BuildOutputPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    view! {
        <div class="page build-output-page">
            <A href="/builds" class="back-link">"← My Builds"</A>
            <h1>"Build Output"</h1>
            <p>"Build ID: " {id}</p>
            <div class="tf-console">
                <div class="console-bar"><span class="console-title">"Terraform Output"</span></div>
                <pre class="console-body"><code>"Waiting for action..."</code></pre>
            </div>
        </div>
    }
}
