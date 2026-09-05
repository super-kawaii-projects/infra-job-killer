use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="page home-page">
            <div class="hero">
                <h1 class="hero-title">"Build production infrastructure"<br/>"without writing infrastructure code."</h1>
                <p class="hero-subtitle">"Point-and-click → Terraform generated → reviewable → yours forever."</p>
            </div>
            <section class="build-type-grid">
                <h2 class="section-title">"What are you building?"</h2>
                <div class="type-cards">
                    <A href="/build/vpc" class="type-card">
                        <div class="type-icon">"🌐"</div>
                        <h3>"VPC"</h3>
                        <p>"Networking — subnets, NAT, routing, flow logs"</p>
                        <span class="type-from">"from $33/mo"</span>
                    </A>
                    <A href="/build/ec2" class="type-card">
                        <div class="type-icon">"🖥️"</div>
                        <h3>"EC2"</h3>
                        <p>"Virtual servers — instances, security groups, volumes"</p>
                        <span class="type-from">"from $7/mo"</span>
                    </A>
                    <A href="/build/ebs" class="type-card">
                        <div class="type-icon">"💾"</div>
                        <h3>"EBS"</h3>
                        <p>"Block storage — persistent volumes, IOPS, encryption"</p>
                        <span class="type-from">"from $8/mo"</span>
                    </A>
                    <A href="/build/eks" class="type-card type-card-featured">
                        <div class="type-icon">"☸️"</div>
                        <h3>"EKS"</h3>
                        <p>"Managed Kubernetes — cluster, platform addons, observability"</p>
                        <span class="type-from">"from $73/mo"</span>
                        <span class="type-badge">"Most popular"</span>
                    </A>
                </div>
            </section>
        </div>
    }
}
