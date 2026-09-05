use leptos::*;
use shared::models::*;
use shared::auth::*;

// ─── Build Management ────────────────────────────────────────────────────────

#[server(CreateBuild, "/api")]
pub async fn create_build(
    name: String,
    build_type: BuildType,
    environment: Environment,
    region: String,
    production_ready: bool,
    config: BuildConfig,
) -> Result<Build, ServerFnError> {
    use crate::server::state::get_state;
    use crate::server::tfvars;
    use shared::cost;
    use uuid::Uuid;
    use chrono::Utc;

    let cost_estimate = match &config {
        BuildConfig::Vpc(c) => cost::estimate_vpc_cost(c, &region),
        BuildConfig::Ec2(c) => cost::estimate_ec2_cost(c, &region),
        BuildConfig::Ebs(c) => cost::estimate_ebs_cost(c, &region),
        BuildConfig::Eks(c) => cost::estimate_eks_cost(c, &region),
    };

    let build = Build {
        id: Uuid::new_v4(),
        account_id: Uuid::nil(),
        name,
        build_type,
        environment,
        region,
        production_ready,
        config,
        status: BuildStatus::Draft,
        cost_estimate: Some(cost_estimate),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Generate terraform files on disk
    tfvars::write_build_files(&build)?;

    let state = get_state();
    let mut builds = state.builds.lock().unwrap();
    builds.push(build.clone());

    Ok(build)
}

#[server(ListBuilds, "/api")]
pub async fn list_builds() -> Result<Vec<Build>, ServerFnError> {
    use crate::server::state::get_state;
    let state = get_state();
    let builds = state.builds.lock().unwrap();
    Ok(builds.clone())
}

#[server(GetBuild, "/api")]
pub async fn get_build(id: String) -> Result<Build, ServerFnError> {
    use crate::server::state::get_state;
    use uuid::Uuid;
    let build_id = Uuid::parse_str(&id)
        .map_err(|_| ServerFnError::ServerError("Invalid ID".into()))?;
    let state = get_state();
    let builds = state.builds.lock().unwrap();
    builds.iter().find(|b| b.id == build_id).cloned()
        .ok_or_else(|| ServerFnError::ServerError("Not found".into()))
}

#[server(RunTerraform, "/api")]
pub async fn run_terraform(build_id: String, action: TerraformAction) -> Result<TerraformResult, ServerFnError> {
    use crate::server::state::get_state;
    use uuid::Uuid;
    use tokio::process::Command;
    use std::path::PathBuf;

    let id = Uuid::parse_str(&build_id)
        .map_err(|_| ServerFnError::ServerError("Invalid ID".into()))?;

    let state = get_state();

    // Get the build
    let build = {
        let builds = state.builds.lock().unwrap();
        builds.iter().find(|b| b.id == id).cloned()
            .ok_or_else(|| ServerFnError::ServerError("Not found".into()))?
    };

    // Update status
    {
        let mut builds = state.builds.lock().unwrap();
        if let Some(b) = builds.iter_mut().find(|b| b.id == id) {
            b.status = match &action {
                TerraformAction::Plan => BuildStatus::Planning,
                TerraformAction::Apply => BuildStatus::Building,
                TerraformAction::Destroy => BuildStatus::Destroying,
            };
        }
    }

    let work_dir = PathBuf::from("deployments")
        .join(build.build_type.to_string().to_lowercase())
        .join(&build.name);

    // terraform init
    let init = Command::new("terraform")
        .args(["init", "-no-color"])
        .current_dir(&work_dir)
        .output().await;

    if let Ok(out) = &init {
        if !out.status.success() {
            update_status(&state, id, BuildStatus::Failed);
            return Ok(TerraformResult {
                success: false,
                stdout: String::from_utf8_lossy(&out.stdout).into(),
                stderr: String::from_utf8_lossy(&out.stderr).into(),
                plan_summary: None,
            });
        }
    }

    // Run action
    let args: Vec<&str> = match &action {
        TerraformAction::Plan => vec!["plan", "-no-color"],
        TerraformAction::Apply => vec!["apply", "-no-color", "-auto-approve"],
        TerraformAction::Destroy => vec!["destroy", "-no-color", "-auto-approve"],
    };

    let result = Command::new("terraform")
        .args(&args)
        .current_dir(&work_dir)
        .output().await;

    let tf_result = match result {
        Ok(out) => TerraformResult {
            success: out.status.success(),
            stdout: String::from_utf8_lossy(&out.stdout).into(),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
            plan_summary: None,
        },
        Err(e) => TerraformResult {
            success: false,
            stdout: String::new(),
            stderr: format!("terraform not found: {}", e),
            plan_summary: None,
        },
    };

    // Update final status
    let final_status = match (&action, tf_result.success) {
        (TerraformAction::Plan, true) => BuildStatus::Planned,
        (TerraformAction::Apply, true) => BuildStatus::Built,
        (TerraformAction::Destroy, true) => BuildStatus::Destroyed,
        (_, false) => BuildStatus::Failed,
    };
    update_status(&state, id, final_status);

    Ok(tf_result)
}

#[server(GetGeneratedFiles, "/api")]
pub async fn get_generated_files(build_id: String) -> Result<Vec<(String, String)>, ServerFnError> {
    use uuid::Uuid;
    use std::path::PathBuf;

    let id = Uuid::parse_str(&build_id)
        .map_err(|_| ServerFnError::ServerError("Invalid ID".into()))?;

    let state = crate::server::state::get_state();
    let builds = state.builds.lock().unwrap();
    let build = builds.iter().find(|b| b.id == id)
        .ok_or_else(|| ServerFnError::ServerError("Not found".into()))?;

    let work_dir = PathBuf::from("deployments")
        .join(build.build_type.to_string().to_lowercase())
        .join(&build.name);

    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&work_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".tf") || name.ends_with(".tfvars") {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        files.push((name.to_string(), content));
                    }
                }
            }
        }
    }

    Ok(files)
}

#[server(PushToGit, "/api")]
pub async fn push_to_git(build_id: String, repo_url: String, branch: String) -> Result<String, ServerFnError> {
    use uuid::Uuid;
    use std::path::PathBuf;
    use tokio::process::Command;

    let id = Uuid::parse_str(&build_id)
        .map_err(|_| ServerFnError::ServerError("Invalid ID".into()))?;

    let state = crate::server::state::get_state();
    let builds = state.builds.lock().unwrap();
    let build = builds.iter().find(|b| b.id == id)
        .ok_or_else(|| ServerFnError::ServerError("Not found".into()))?;

    let work_dir = PathBuf::from("deployments")
        .join(build.build_type.to_string().to_lowercase())
        .join(&build.name);

    // git init + add + commit + push
    Command::new("git").args(["init"]).current_dir(&work_dir).output().await.ok();
    Command::new("git").args(["add", "."]).current_dir(&work_dir).output().await.ok();
    Command::new("git").args(["commit", "-m", "infra-job-killer: generated infrastructure"]).current_dir(&work_dir).output().await.ok();
    Command::new("git").args(["remote", "add", "origin", &repo_url]).current_dir(&work_dir).output().await.ok();

    let push = Command::new("git")
        .args(["push", "-u", "origin", &branch])
        .current_dir(&work_dir)
        .output().await;

    match push {
        Ok(out) if out.status.success() => Ok("Pushed successfully".into()),
        Ok(out) => Err(ServerFnError::ServerError(String::from_utf8_lossy(&out.stderr).into())),
        Err(e) => Err(ServerFnError::ServerError(format!("git push failed: {}", e))),
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

#[cfg(feature = "ssr")]
fn update_status(state: &crate::server::state::AppState, id: uuid::Uuid, status: BuildStatus) {
    let mut builds = state.builds.lock().unwrap();
    if let Some(b) = builds.iter_mut().find(|b| b.id == id) {
        b.status = status;
    }
}

#[cfg(feature = "ssr")]
pub mod state {
    use shared::models::Build;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    pub struct AppState {
        pub builds: Arc<Mutex<Vec<Build>>>,
    }

    impl AppState {
        pub fn new() -> Self {
            Self { builds: Arc::new(Mutex::new(Vec::new())) }
        }
    }

    static STATE: std::sync::OnceLock<AppState> = std::sync::OnceLock::new();

    pub fn get_state() -> &'static AppState {
        STATE.get_or_init(|| AppState::new())
    }
}

#[cfg(feature = "ssr")]
pub mod tfvars;
