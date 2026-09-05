# infra-job-killer

**Build AWS infrastructure from a web UI — with live cost estimates, generated Terraform, and one-click plan/apply.**

infra-job-killer is a self-hosted web app that turns "I need a VPC / EC2 / EBS / EKS setup" into real, reviewable Terraform. Fill out a form, watch the monthly cost update as you tweak options, then generate the code, run `terraform plan/apply` from the UI, or push the whole thing to a Git repo. No hand-writing HCL, no guessing what it'll cost.

Built in Rust with [Leptos](https://leptos.dev) (full-stack WASM + SSR) — ships as a single server binary that serves both the API and the frontend.

---

## What it does

- **Visual builders** for the infra people actually stand up first:
  - **VPC** — CIDR, AZ count, NAT gateways (single vs per-AZ), flow logs, endpoints
  - **EC2** — instance type/count, EBS root volume, monitoring
  - **EBS** — size, type (gp3/io2/etc.), encryption, IOPS
  - **EKS** — version, node groups vs Karpenter, add-ons (ArgoCD, Prometheus, Grafana, cert-manager, external-secrets, and more), security options
- **Live cost estimation** — the monthly cost updates as you change options, with a per-line breakdown (and production add-on costs called out separately)
- **Generates real Terraform** — `main.tf`, `variables.tf`, `terraform.tfvars` per build
- **Run Terraform from the UI** — plan, apply, or destroy, with output streamed back
- **Push to Git** — commit the generated files and push to a repo you own
- **Build history** — every build is tracked with its status (Draft → Planned → Built)

---

## Quick start

### Docker (recommended)

```bash
docker compose up --build
```

Then open **http://localhost:3000**.

To actually run Terraform against AWS, mount your credentials:

```yaml
# docker-compose.yml
services:
  app:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ~/.aws:/root/.aws:ro          # AWS credentials
      - ./data:/app/data              # build history
      - ./deployments:/app/deployments# generated terraform
    environment:
      - LEPTOS_SITE_ADDR=0.0.0.0:3000
      - RUST_LOG=info
```

### From source (development)

```bash
# one-time
cargo install cargo-leptos

# hot-reloading dev server
cargo leptos watch
```

Open http://localhost:3000. Edits to Rust/CSS reload live.

### From source (production build)

```bash
cargo leptos build --release
# produces the server binary + WASM/site assets
./target/release/platform-made-easy
```

---

## Typical flow

1. **Open the app** → pick a builder (VPC, EC2, EBS, or EKS)
2. **Configure it** with the form — watch the **Estimated Monthly Cost** update as you go
3. **Test My Build** → runs `terraform plan` and shows the result
4. **Build It** → runs `terraform apply` (or just grab the generated files)
5. Optionally **push to Git** to hand the code off to your normal review/CI flow

Every build writes a clean Terraform project under `deployments/<type>/<name>/`:

```
deployments/eks/my-cluster/
├── main.tf
├── variables.tf
└── terraform.tfvars
```

The philosophy: **generate it, review it, own it.** The tool writes standard Terraform you can read, commit, and run yourself — nothing locked in a proprietary format.

---

## Configuration

| Env var | Purpose | Default |
|---------|---------|---------|
| `LEPTOS_SITE_ADDR` | Address the server binds to | `0.0.0.0:3000` |
| `RUST_LOG` | Log level (`info`, `debug`, etc.) | `info` |
| AWS credentials | Standard AWS CLI/SDK env or `~/.aws` mount | — |

Generated Terraform and build history persist to `./deployments` and `./data` — mount these as volumes so they survive container restarts.

---

## Prerequisites

- **Docker** (for the container path) or the **Rust toolchain + cargo-leptos** (for source builds)
- **Terraform** available in the runtime environment if you want to plan/apply from the UI
- **AWS credentials** with permission to create the resources you're building (plan/apply act on your real account)
- **Git** if you use the push-to-repo feature

> ⚠️ **Apply runs against real AWS and costs real money.** Use the cost estimate and `terraform plan` before applying. Start in a sandbox account if you're just evaluating.

---

## Architecture

```
infra-job-killer/
├── frontend/       # Leptos app — pages, builders, server functions (SSR + WASM)
│   └── src/
│       ├── pages/  # vpc/ec2/ebs/eks builders, builds list, build output, settings
│       └── server/ # #[server] fns: create build, run terraform, push to git, tfvars gen
├── shared/         # Types shared client/server: configs, cost models, build state
├── Dockerfile
└── docker-compose.yml
```

- **Frontend (WASM)** renders the builders and reacts to input with live cost memos.
- **Server (SSR)** hosts `#[server]` functions that generate Terraform, shell out to `terraform`, and handle Git pushes.
- **shared** holds the `BuildConfig`, `CostEstimate`, and `Build` types so the client and server speak the same language.

---

## License

Copyright (c) 2026 Stillwater Strategic Solutions LLC. All rights reserved.

This is **source-available** software, not open source. It is free for
personal, non-commercial evaluation and testing. **Commercial and production
use requires a paid license.** See [LICENSE](LICENSE) for full terms, or
contact Stillwater Strategic Solutions LLC at michaelisaacs121092@gmail.com
for commercial licensing.
