use leptos::*;
use leptos_router::*;
use shared::models::*;
use shared::cost::{estimate_eks_cost, format_money};
use crate::server::create_build;

#[component]
pub fn EksBuilderPage() -> impl IntoView {
    let navigate = use_navigate();
    let (name, set_name) = create_signal("my-cluster".to_string());
    let (region, set_region) = create_signal("us-east-1".to_string());
    let (version, set_version) = create_signal("1.30".to_string());
    let (multi_az, set_multi_az) = create_signal(true);
    let (private_nodes, set_private_nodes) = create_signal(true);
    let (enable_nat, set_enable_nat) = create_signal(true);
    let (vpc_endpoints, set_vpc_endpoints) = create_signal(false);
    let (alb_controller, set_alb_controller) = create_signal(true);
    let (instance_type, set_instance_type) = create_signal("m5.xlarge".to_string());
    let (min_nodes, set_min_nodes) = create_signal(2u32);
    let (max_nodes, set_max_nodes) = create_signal(10u32);
    let (desired_nodes, set_desired_nodes) = create_signal(3u32);
    let (argocd, set_argocd) = create_signal(false);
    let (prometheus, set_prometheus) = create_signal(true);
    let (grafana, set_grafana) = create_signal(true);
    let (ext_secrets, set_ext_secrets) = create_signal(true);
    let (cert_manager, set_cert_manager) = create_signal(true);
    let (pod_identity, set_pod_identity) = create_signal(true);
    let (net_policies, set_net_policies) = create_signal(true);
    let (private_api, set_private_api) = create_signal(false);
    let (secrets_enc, set_secrets_enc) = create_signal(true);
    let (production_ready, set_production_ready) = create_signal(false);
    let (submitting, set_submitting) = create_signal(false);

    let cost = create_memo(move |_| {
        let config = EksConfig {
            cluster_name: name.get(), cluster_version: version.get(),
            multi_az: multi_az.get(), private_nodes: private_nodes.get(),
            enable_nat: enable_nat.get(), enable_vpc_endpoints: vpc_endpoints.get(),
            enable_alb_controller: alb_controller.get(), vpc_cidr: "10.0.0.0/16".into(),
            compute_type: EksComputeType::Karpenter, instance_type: instance_type.get(),
            min_nodes: min_nodes.get(), max_nodes: max_nodes.get(), desired_nodes: desired_nodes.get(),
            enable_argocd: argocd.get(), enable_prometheus: prometheus.get(),
            enable_grafana: grafana.get(), enable_external_secrets: ext_secrets.get(),
            enable_cert_manager: cert_manager.get(), enable_external_dns: false,
            enable_istio: false, enable_pod_identity: pod_identity.get(),
            enable_network_policies: net_policies.get(), private_api_endpoint: private_api.get(),
            enable_secrets_encryption: secrets_enc.get(), enable_keda: false,
            enable_descheduler: false, enable_crane: false, enable_nth: true,
            enable_pdb: true, enable_topology_spread: true, tags: Default::default(),
        };
        estimate_eks_cost(&config, &region.get())
    });

    create_effect(move |_| {
        if production_ready.get() {
            set_multi_az.set(true); set_private_nodes.set(true);
            set_vpc_endpoints.set(true); set_prometheus.set(true);
            set_grafana.set(true); set_ext_secrets.set(true);
            set_cert_manager.set(true); set_pod_identity.set(true);
            set_net_policies.set(true); set_private_api.set(true);
            set_secrets_enc.set(true); set_argocd.set(true);
            if min_nodes.get_untracked() < 3 { set_min_nodes.set(3); }
        }
    });

    let on_submit = move |_action: &'static str| {
        set_submitting.set(true);
        let nav = navigate.clone();
        let config = EksConfig {
            cluster_name: name.get(), cluster_version: version.get(),
            multi_az: multi_az.get(), private_nodes: private_nodes.get(),
            enable_nat: enable_nat.get(), enable_vpc_endpoints: vpc_endpoints.get(),
            enable_alb_controller: alb_controller.get(), vpc_cidr: "10.0.0.0/16".into(),
            compute_type: EksComputeType::Karpenter, instance_type: instance_type.get(),
            min_nodes: min_nodes.get(), max_nodes: max_nodes.get(), desired_nodes: desired_nodes.get(),
            enable_argocd: argocd.get(), enable_prometheus: prometheus.get(),
            enable_grafana: grafana.get(), enable_external_secrets: ext_secrets.get(),
            enable_cert_manager: cert_manager.get(), enable_external_dns: false,
            enable_istio: false, enable_pod_identity: pod_identity.get(),
            enable_network_policies: net_policies.get(), private_api_endpoint: private_api.get(),
            enable_secrets_encryption: secrets_enc.get(), enable_keda: false,
            enable_descheduler: false, enable_crane: false, enable_nth: true,
            enable_pdb: true, enable_topology_spread: true, tags: Default::default(),
        };
        let name_val = name.get();
        let region_val = region.get();
        let prod = production_ready.get();
        let env = if prod { Environment::Production } else { Environment::Dev };
        spawn_local(async move {
            match create_build(name_val, BuildType::Eks, env, region_val, prod, BuildConfig::Eks(config)).await {
                Ok(build) => { nav(&format!("/build/{}/output", build.id), Default::default()); }
                Err(_) => { set_submitting.set(false); }
            }
        });
    };

    view! {
        <div class="page builder-page">
            <div class="builder-header">
                <A href="/" class="back-link">"← Back"</A>
                <h1>"☸️ EKS Builder"</h1>
                <p>"Build a production-grade Kubernetes cluster"</p>
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
                        <h3>"Cluster"</h3>
                        <div class="form-grid">
                            <div class="form-group">
                                <label>"Cluster Name"</label>
                                <input type="text" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev))/>
                            </div>
                            <div class="form-group">
                                <label>"Kubernetes Version"</label>
                                <select on:change=move |ev| set_version.set(event_target_value(&ev))>
                                    <option value="1.30">"1.30"</option>
                                    <option value="1.29">"1.29"</option>
                                    <option value="1.28">"1.28"</option>
                                </select>
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
                                    <option value="t3.large">"t3.large (2 vCPU, 8GB)"</option>
                                    <option value="m5.large">"m5.large (2 vCPU, 8GB)"</option>
                                    <option value="m5.xlarge">"m5.xlarge (4 vCPU, 16GB)"</option>
                                    <option value="m5.2xlarge">"m5.2xlarge (8 vCPU, 32GB)"</option>
                                    <option value="c5.xlarge">"c5.xlarge (4 vCPU, 8GB)"</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>"Min Nodes"</label>
                                <input type="number" min="1" max="100" prop:value=move || min_nodes.get().to_string()
                                    on:input=move |ev| set_min_nodes.set(event_target_value(&ev).parse().unwrap_or(2))/>
                            </div>
                            <div class="form-group">
                                <label>"Desired Nodes"</label>
                                <input type="number" min="1" max="100" prop:value=move || desired_nodes.get().to_string()
                                    on:input=move |ev| set_desired_nodes.set(event_target_value(&ev).parse().unwrap_or(3))/>
                            </div>
                            <div class="form-group">
                                <label>"Max Nodes"</label>
                                <input type="number" min="1" max="500" prop:value=move || max_nodes.get().to_string()
                                    on:input=move |ev| set_max_nodes.set(event_target_value(&ev).parse().unwrap_or(10))/>
                            </div>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>"Networking"</h3>
                        <div class="checkbox-group">
                            <label class="checkbox-label"><input type="checkbox" prop:checked=multi_az on:change=move |_| set_multi_az.update(|v| *v = !*v)/>"Multi-AZ"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=private_nodes on:change=move |_| set_private_nodes.update(|v| *v = !*v)/>"Private Nodes"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=enable_nat on:change=move |_| set_enable_nat.update(|v| *v = !*v)/>"NAT Gateway"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=vpc_endpoints on:change=move |_| set_vpc_endpoints.update(|v| *v = !*v)/>"VPC Endpoints"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=alb_controller on:change=move |_| set_alb_controller.update(|v| *v = !*v)/>"ALB Controller"</label>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>"Platform"</h3>
                        <div class="checkbox-group">
                            <label class="checkbox-label"><input type="checkbox" prop:checked=argocd on:change=move |_| set_argocd.update(|v| *v = !*v)/>"Argo CD"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=prometheus on:change=move |_| set_prometheus.update(|v| *v = !*v)/>"Prometheus"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=grafana on:change=move |_| set_grafana.update(|v| *v = !*v)/>"Grafana"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=ext_secrets on:change=move |_| set_ext_secrets.update(|v| *v = !*v)/>"External Secrets"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=cert_manager on:change=move |_| set_cert_manager.update(|v| *v = !*v)/>"cert-manager"</label>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>"Security"</h3>
                        <div class="checkbox-group">
                            <label class="checkbox-label"><input type="checkbox" prop:checked=pod_identity on:change=move |_| set_pod_identity.update(|v| *v = !*v)/>"Pod Identity"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=net_policies on:change=move |_| set_net_policies.update(|v| *v = !*v)/>"Network Policies"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=private_api on:change=move |_| set_private_api.update(|v| *v = !*v)/>"Private API Endpoint"</label>
                            <label class="checkbox-label"><input type="checkbox" prop:checked=secrets_enc on:change=move |_| set_secrets_enc.update(|v| *v = !*v)/>"Secrets Encryption (KMS)"</label>
                        </div>
                    </div>
                </div>

                <div class="builder-sidebar">
                    <div class="cost-card">
                        <h3>"Estimated Monthly Cost"</h3>
                        <div class="cost-total">{move || format!("${}", format_money(cost.get().monthly_total))}<span class="cost-period">"/mo"</span></div>
                        <div class="cost-breakdown">
                            {move || cost.get().line_items.iter().map(|item| {
                                view! {
                                    <div class="cost-line">
                                        <span class="cost-line-name">{&item.resource}</span>
                                        <span class="cost-line-amount">{format!("${:.0}", item.monthly_cost)}</span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                    <div class="action-card">
                        <button class="btn btn-info btn-lg btn-full" disabled=move || submitting.get()
                            on:click={let f = on_submit.clone(); move |_| f("plan")}>"🔍 Test My Build"</button>
                        <button class="btn btn-primary btn-lg btn-full" disabled=move || submitting.get()
                            on:click=move |_| on_submit("apply")>"🚀 GENERATE INFRASTRUCTURE"</button>
                    </div>
                    <div class="output-preview-card">
                        <h4>"What you'll get:"</h4>
                        <pre class="file-tree-preview">"deployments/eks/my-cluster/\n├── main.tf\n├── variables.tf\n└── terraform.tfvars"</pre>
                        <p class="preview-note">"Download it. Push to GitHub. Run Terraform yourself."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
