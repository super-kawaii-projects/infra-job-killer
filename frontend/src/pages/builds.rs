use leptos::*;
use leptos_router::*;

#[component]
pub fn BuildsPage() -> impl IntoView {
    view! {
        <div class="page builds-page">
            <div class="page-header">
                <h1>"My Builds"</h1>
                <A href="/" class="btn btn-primary">"+ New Build"</A>
            </div>
            <div class="empty-state">
                <div class="empty-icon">"🏗️"</div>
                <h3>"No builds yet"</h3>
                <p>"Create your first infrastructure build to get started."</p>
                <A href="/" class="btn btn-primary">"Start Building"</A>
            </div>
        </div>
    }
}
