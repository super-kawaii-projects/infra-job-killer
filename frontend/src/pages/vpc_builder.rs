use leptos::*;
use leptos_router::*;
use shared::models::*;
use shared::cost::estimate_vpc_cost;
use crate::server::create_build;

#[component]
pub fn VpcBuilderPage() -> impl IntoView {
    let navigate = use_navigate();
    let (name, set_name) = create_signal("my-vpc".to_string());
    let (region, set_region) = create_signal("us-east-1".to_string());
    let (vpc_cidr, set_vpc_cidr) = create_signal("10.0.0.0/16".to_string());
    let (az_count, set_az_count) = create_signal(3u8);
    let (enable_nat, set_enable_nat) = create_signal(true);
    let (single_nat, set_single_nat) = create_signal(false);
    let (flow_logs, set_flow_logs) = create_signal(true);
    let (vpc_endpoints, set_vpc_endpoints) = create_signal(false);
    let (production_ready, set_production_ready) = create_signal(false);
    let (submitting, set_submitting) = create_signal(false);

    // Live cost calculation
    let cost = create_memo(move |_| {
        let config = VpcConfig {
            vpc_cidr: vpc_cidr.get(),
            az_count: az_count.get(),
            enable_nat_gateway: enable_nat.get(),
            single_nat_gateway: single_nat.get(),
            enable_vpn_gateway: false,
            enable_flow_logs: flow_logs.get(),
            enable_vpc_endpoints: vpc_endpoints.get(),
            private_subnets: vec![],
            public_subnets: vec![],
            tags: Default::default(),
        };
        estimate_vpc_cost(&config, &region.get())
    });

    // Production toggle
    create_effect(move |_| {
        if production_ready.get() {
            set_az_count.set(3);
            set_single_nat.set(false);
            set_flow_logs.set(true);
            set_vpc_endpoints.set(true);
        }
    });

    let on_submit = move |_action: &'static str| {
        set_submitting.set(true);
        let nav = navigate.clone();
        let config = VpcConfig {
            vpc_cidr: vpc_cidr.get(),
            az_count: az_count.get(),
            enable_nat_gateway: enable_nat.get(),
            single_nat_gateway: single_nat.get(),
            enable_vpn_gateway: false,
            enable_flow_logs: flow_logs.get(),
            enable_vpc_endpoints: vpc_endpoints.get(),
            private_subnets: vec![],
            public_subnets: vec![],
            tags: Default::default(),
        };
        let name_val = name.get();
        let region_val = region.get();
        let prod = production_ready.get();
        let env = if prod { Environment::Production } else { Environment::Dev };

        spawn_local(async move {
            match create_build(name_val, BuildType::Vpc, env, region_val, prod, BuildConfig::Vpc(config)).await {
                Ok(build) => { nav(&format!("/build/{}/output", build.id), Default::default()); }
                Err(_) => { set_submitting.set(false); }
            }
        });
    };

    view! {
        <div class="page builder-page">
            <div class="builder-header">
                <A href="/" class="back-link">"← Back"</A>
                <h1>"🌐 VPC Builder"</h1>
                <p>"Configure your Virtual Private Cloud"</p>
            </div>

            <div class="builder-layout">
                <div class="builder-form">
                    // Production toggle
                    <div class="production-toggle-card">
                        <label class="toggle-label">
                            <span class="toggle-text">"🟢 MAKE THIS PRODUCTION READY"</span>
                            <input type="checkbox" class="toggle-switch"
                                prop:checked=production_ready
                                on:change=move |_| set_production_ready.update(|v| *v = !*v)
                            />
                        </label>
                    </div>

                    <div class="form-section">
                        <h3>"Basic"</h3>
                        <div class="form-grid">
                            <div class="form-group">
                                <label>"Name"</label>
                                <input type="text" prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))/>
                            </div>
                            <div class="form-group">
                                <label>"Region"</label>
                                <select on:change=move |ev| set_region.set(event_target_value(&ev))>
                                    <option value="us-east-1">"US East (N. Virginia)"</option>
                                    <option value="us-east-2">"US East (Ohio)"</option>
                                    <option value="us-west-2">"US West (Oregon)"</option>
                                    <option value="eu-west-1">"EU (Ireland)"</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>"VPC CIDR"</label>
                                <input type="text" prop:value=vpc_cidr
                                    on:input=move |ev| set_vpc_cidr.set(event_target_value(&ev))/>
                            </div>
                            <div class="form-group">
                                <label>"Availability Zones"</label>
                                <select on:change=move |ev| set_az_count.set(event_target_value(&ev).parse().unwrap_or(3))>
                                    <option value="2">"2"</option>
                                    <option value="3" selected=true>"3"</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>"Options"</h3>
                        <div class="checkbox-group">
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=enable_nat
                                    on:change=move |_| set_enable_nat.update(|v| *v = !*v)/>
                                "NAT Gateway (private subnet internet)"
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=single_nat
                                    on:change=move |_| set_single_nat.update(|v| *v = !*v)/>
                                "Single NAT (saves cost, lower HA)"
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=flow_logs
                                    on:change=move |_| set_flow_logs.update(|v| *v = !*v)/>
                                "VPC Flow Logs"
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=vpc_endpoints
                                    on:change=move |_| set_vpc_endpoints.update(|v| *v = !*v)/>
                                "VPC Endpoints (S3, ECR, STS)"
                            </label>
                        </div>
                    </div>
                </div>

                // Cost sidebar
                <div class="builder-sidebar">
                    <div class="cost-card">
                        <h3>"Estimated Monthly Cost"</h3>
                        <div class="cost-total">{move || format!("${:.0}", cost.get().monthly_total)}<span class="cost-period">"/mo"</span></div>
                        <div class="cost-breakdown">
                            {move || cost.get().line_items.iter().map(|item| {
                                view! {
                                    <div class="cost-line">
                                        <span class="cost-line-name">{&item.resource}</span>
                                        <span class="cost-line-amount">{format!("${:.2}", item.monthly_cost)}</span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                    <div class="action-card">
                        <button class="btn btn-info btn-lg btn-full"
                            disabled=move || submitting.get()
                            on:click={let f = on_submit.clone(); move |_| f("plan")}>
                            "🔍 Test My Build"
                        </button>
                        <button class="btn btn-primary btn-lg btn-full"
                            disabled=move || submitting.get()
                            on:click=move |_| on_submit("apply")>
                            "🚀 Build It"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
