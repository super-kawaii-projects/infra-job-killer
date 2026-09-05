use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::{home::HomePage, builds::BuildsPage, build_output::BuildOutputPage};
use crate::pages::{vpc_builder::VpcBuilderPage, ec2_builder::Ec2BuilderPage};
use crate::pages::{ebs_builder::EbsBuilderPage, eks_builder::EksBuilderPage};
use crate::pages::settings::SettingsPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/platform-made-easy.css"/>
        <Title text="infra-job-killer"/>

        <Router>
            <nav class="topbar">
                <A href="/" class="topbar-brand">
                    <span class="brand-logo">"⚡"</span>
                    <span class="brand-name">"infra-job-killer"</span>
                </A>
                <div class="topbar-nav">
                    <A href="/" class="nav-link">"Build"</A>
                    <A href="/builds" class="nav-link">"My Builds"</A>
                    <A href="/settings" class="nav-link">"⚙️ Accounts"</A>
                </div>
            </nav>
            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/build/vpc" view=VpcBuilderPage/>
                    <Route path="/build/ec2" view=Ec2BuilderPage/>
                    <Route path="/build/ebs" view=EbsBuilderPage/>
                    <Route path="/build/eks" view=EksBuilderPage/>
                    <Route path="/build/:id/output" view=BuildOutputPage/>
                    <Route path="/builds" view=BuildsPage/>
                    <Route path="/settings" view=SettingsPage/>
                </Routes>
            </main>
        </Router>
    }
}
