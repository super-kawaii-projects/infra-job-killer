use leptos::*;
use leptos_router::*;
use shared::models::*;
use shared::cost::estimate_ec2_cost;
use crate::server::create_build;

#[component]
pub fn Ec2BuilderPage() -> impl IntoView {
    let navigate = use_navigate();
    let (name, set_name) = create_signal("my-instance".to_string());
    let (region, set_region) = create_signal("us-east-1".to_string());
    let (instance_type, set_instance_type) = create_signal("t3.medium".to_string());
    let (instance_count, set_instance_count) = create_signal(1u32);
    let (volume_size, set_volume_size) = create_signal(30u32);
    let (volume_type, set_volume_type) = create_signal("gp3".to_string());
    let (key_pair, set_key_pair) = create_signal(String::new());
    let (public_ip, set_public_ip) = create_signal(false);
    let (monitoring, set_monitoring) = create_signal(true);
    let (term_protection, set_term_protection) = create_signal(false);
    let (production_ready, set_production_ready) = create_signal(false);
    let (submitting, set_submitting) = create_signal(false);

    let cost = create_memo(move |_| {
        let config = Ec2Config {
            instance_type: instance_type.get(),
            instance_count: instance_count.get(),
            volume_size_gb: volume_size.get(),
            volume_type: volume_type.get(),
            associate_public_ip: public_ip.get(),
            enable_monitoring: monitoring.get(),
            enable_termination_protection: term_protection.get(),
            key_pair_name: String::new(),
            subnet_placement: if public_ip.get() { SubnetPlacement::Public } else { SubnetPlacement::Private },
            tags: Default::default(),
        };
        estimate_ec2_cost(&config, &region.get())
    });

    create_effect(move |_| {
        if production_ready.get() {
            set_monitoring.set(true);
            set_term_protection.set(true);
            set_public_ip.set(false);
        }
    });

    let on_submit = move |_action: &'static str| {
        set_submitting.set(true);
        let nav = navigate.clone();
        let config = Ec2Config {
            instance_type: instance_type.get(),
            instance_count: instance_count.get(),
            volume_size_gb: volume_size.get(),
            volume_type: volume_type.get(),
            associate_public_ip: public_ip.get(),
            enable_monitoring: monitoring.get(),
            enable_termination_protection: term_protection.get(),
            key_pair_name: key_pair.get(),
            subnet_placement: if public_ip.get() { SubnetPlacement::Public } else { SubnetPlacement::Private },
            tags: Default::default(),
        };
        let name_val = name.get();
        let region_val = region.get();
        let prod = production_ready.get();
        let env = if prod { Environment::Production } else { Environment::Dev };

        spawn_local(async move {
            match create_build(name_val, BuildType::Ec2, env, region_val, prod, BuildConfig::Ec2(config)).await {
                Ok(build) => { nav(&format!("/build/{}/output", build.id), Default::default()); }
                Err(_) => { set_submitting.set(false); }
            }
        });
    };

    view! {
        <div class="page builder-page">
            <div class="builder-header">
                <A href="/" class="back-link">"← Back"</A>
                <h1>"🖥️ EC2 Builder"</h1>
                <p>"Configure your virtual servers"</p>
            </div>

            <div class="builder-layout">
                <div class="builder-form">
                    <div class="production-toggle-card">
                        <label class="toggle-label">
                            <span class="toggle-text">"🟢 MAKE THIS PRODUCTION READY"</span>
                            <input type="checkbox" class="toggle-switch"
                                prop:checked=production_ready
                                on:change=move |_| set_production_ready.update(|v| *v = !*v)/>
                        </label>
                    </div>

                    <div class="form-section">
                        <h3>"Instance"</h3>
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
                                <label>"Instance Type"</label>
                                <select prop:value=instance_type on:change=move |ev| set_instance_type.set(event_target_value(&ev))>
                                    <option value="t3.micro">"t3.micro (2 vCPU, 1GB)"</option>
                                    <option value="t3.small">"t3.small (2 vCPU, 2GB)"</option>
                                    <option value="t3.medium">"t3.medium (2 vCPU, 4GB)"</option>
                                    <option value="t3.large">"t3.large (2 vCPU, 8GB)"</option>
                                    <option value="m5.large">"m5.large (2 vCPU, 8GB)"</option>
                                    <option value="m5.xlarge">"m5.xlarge (4 vCPU, 16GB)"</option>
                                    <option value="c5.large">"c5.large (2 vCPU, 4GB)"</option>
                                    <option value="c5.xlarge">"c5.xlarge (4 vCPU, 8GB)"</option>
                                    <option value="r5.large">"r5.large (2 vCPU, 16GB)"</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>"Instance Count"</label>
                                <input type="number" min="1" max="20"
                                    prop:value=move || instance_count.get().to_string()
                                    on:input=move |ev| set_instance_count.set(event_target_value(&ev).parse().unwrap_or(1))/>
                            </div>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>"Storage"</h3>
                        <div class="form-grid">
                            <div class="form-group">
                                <label>"Root Volume Size (GB)"</label>
                                <input type="number" min="8" max="16384"
                                    prop:value=move || volume_size.get().to_string()
                                    on:input=move |ev| set_volume_size.set(event_target_value(&ev).parse().unwrap_or(30))/>
                            </div>
                            <div class="form-group">
                                <label>"Volume Type"</label>
                                <select prop:value=volume_type on:change=move |ev| set_volume_type.set(event_target_value(&ev))>
                                    <option value="gp3">"gp3 (General Purpose SSD)"</option>
                                    <option value="gp2">"gp2 (Previous Gen)"</option>
                                    <option value="io2">"io2 (Provisioned IOPS)"</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>"Network & Security"</h3>
                        <div class="form-group">
                            <label>"Key Pair Name"</label>
                            <input type="text" placeholder="my-key-pair"
                                prop:value=key_pair
                                on:input=move |ev| set_key_pair.set(event_target_value(&ev))/>
                        </div>
                        <div class="checkbox-group">
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=public_ip
                                    on:change=move |_| set_public_ip.update(|v| *v = !*v)/>
                                "Associate Public IP"
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=monitoring
                                    on:change=move |_| set_monitoring.update(|v| *v = !*v)/>
                                "Detailed Monitoring (1-min metrics)"
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=term_protection
                                    on:change=move |_| set_term_protection.update(|v| *v = !*v)/>
                                "Termination Protection"
                            </label>
                        </div>
                    </div>
                </div>

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
                        <button class="btn btn-info btn-lg btn-full" disabled=move || submitting.get()
                            on:click={let f = on_submit.clone(); move |_| f("plan")}>"🔍 Test My Build"</button>
                        <button class="btn btn-primary btn-lg btn-full" disabled=move || submitting.get()
                            on:click=move |_| on_submit("apply")>"🚀 Build It"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
