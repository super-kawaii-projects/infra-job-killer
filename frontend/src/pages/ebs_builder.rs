use leptos::*;
use leptos_router::*;
use shared::models::*;
use shared::cost::estimate_ebs_cost;
use crate::server::create_build;

#[component]
pub fn EbsBuilderPage() -> impl IntoView {
    let navigate = use_navigate();
    let (name, set_name) = create_signal("my-volume".to_string());
    let (region, set_region) = create_signal("us-east-1".to_string());
    let (vol_size, set_vol_size) = create_signal(100u32);
    let (vol_type, set_vol_type) = create_signal("gp3".to_string());
    let (iops, set_iops) = create_signal(3000u32);
    let (throughput, set_throughput) = create_signal(125u32);
    let (encrypted, set_encrypted) = create_signal(true);
    let (multi_attach, set_multi_attach) = create_signal(false);
    let (production_ready, set_production_ready) = create_signal(false);
    let (submitting, set_submitting) = create_signal(false);

    let cost = create_memo(move |_| {
        let config = EbsConfig {
            volume_size_gb: vol_size.get(),
            volume_type: vol_type.get(),
            iops: Some(iops.get()),
            throughput: Some(throughput.get()),
            encrypted: encrypted.get(),
            multi_attach: multi_attach.get(),
            tags: Default::default(),
        };
        estimate_ebs_cost(&config, &region.get())
    });

    create_effect(move |_| {
        if production_ready.get() { set_encrypted.set(true); }
    });

    let on_submit = move |_action: &'static str| {
        set_submitting.set(true);
        let nav = navigate.clone();
        let config = EbsConfig {
            volume_size_gb: vol_size.get(),
            volume_type: vol_type.get(),
            iops: Some(iops.get()),
            throughput: Some(throughput.get()),
            encrypted: encrypted.get(),
            multi_attach: multi_attach.get(),
            tags: Default::default(),
        };
        let name_val = name.get();
        let region_val = region.get();
        let prod = production_ready.get();
        let env = if prod { Environment::Production } else { Environment::Dev };

        spawn_local(async move {
            match create_build(name_val, BuildType::Ebs, env, region_val, prod, BuildConfig::Ebs(config)).await {
                Ok(build) => { nav(&format!("/build/{}/output", build.id), Default::default()); }
                Err(_) => { set_submitting.set(false); }
            }
        });
    };

    view! {
        <div class="page builder-page">
            <div class="builder-header">
                <A href="/" class="back-link">"← Back"</A>
                <h1>"💾 EBS Builder"</h1>
                <p>"Configure persistent block storage"</p>
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
                        <h3>"Volume"</h3>
                        <div class="form-grid">
                            <div class="form-group">
                                <label>"Name"</label>
                                <input type="text" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev))/>
                            </div>
                            <div class="form-group">
                                <label>"Region"</label>
                                <select on:change=move |ev| set_region.set(event_target_value(&ev))>
                                    <option value="us-east-1">"US East (N. Virginia)"</option>
                                    <option value="us-east-2">"US East (Ohio)"</option>
                                    <option value="us-west-2">"US West (Oregon)"</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>"Volume Type"</label>
                                <select prop:value=vol_type on:change=move |ev| set_vol_type.set(event_target_value(&ev))>
                                    <option value="gp3">"gp3 (General Purpose)"</option>
                                    <option value="gp2">"gp2 (Previous Gen)"</option>
                                    <option value="io2">"io2 (Provisioned IOPS)"</option>
                                    <option value="st1">"st1 (Throughput HDD)"</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>"Size (GB)"</label>
                                <input type="number" min="1" max="16384"
                                    prop:value=move || vol_size.get().to_string()
                                    on:input=move |ev| set_vol_size.set(event_target_value(&ev).parse().unwrap_or(100))/>
                            </div>
                            <div class="form-group">
                                <label>"IOPS (gp3 baseline: 3000)"</label>
                                <input type="number" min="3000" max="64000"
                                    prop:value=move || iops.get().to_string()
                                    on:input=move |ev| set_iops.set(event_target_value(&ev).parse().unwrap_or(3000))/>
                            </div>
                            <div class="form-group">
                                <label>"Throughput MB/s (gp3 baseline: 125)"</label>
                                <input type="number" min="125" max="1000"
                                    prop:value=move || throughput.get().to_string()
                                    on:input=move |ev| set_throughput.set(event_target_value(&ev).parse().unwrap_or(125))/>
                            </div>
                        </div>
                        <div class="checkbox-group">
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=encrypted
                                    on:change=move |_| set_encrypted.update(|v| *v = !*v)/>
                                "Encrypt at rest (AES-256)"
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" prop:checked=multi_attach
                                    on:change=move |_| set_multi_attach.update(|v| *v = !*v)/>
                                "Multi-Attach (io1/io2 only)"
                            </label>
                        </div>
                    </div>
                </div>
                <div class="builder-sidebar">
                    <div class="cost-card">
                        <h3>"Estimated Monthly Cost"</h3>
                        <div class="cost-total">{move || format!("${:.2}", cost.get().monthly_total)}<span class="cost-period">"/mo"</span></div>
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
                            on:click=move |_| on_submit("plan")>"🔍 Test My Build"</button>
                        <button class="btn btn-primary btn-lg btn-full" disabled=move || submitting.get()
                            on:click=move |_| on_submit("apply")>"🚀 Build It"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
