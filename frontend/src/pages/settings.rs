use leptos::*;
use leptos_router::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="page settings-page">
            <h1>"⚙️ AWS Accounts"</h1>
            <p>"Connect your AWS accounts to build infrastructure."</p>
            <div class="empty-state">
                <div class="empty-icon">"🔑"</div>
                <h3>"No accounts configured"</h3>
                <p>"Add your AWS Account ID, region, and credentials (IAM keys or Assume Role)."</p>
            </div>
        </div>
    }
}
